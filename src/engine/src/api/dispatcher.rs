//! Cork Dispatcher - Adaptive Batching Implementation
//! 
//! This module implements the main CorkDispatcher that provides intelligent
//! request batching with dynamic optimization, translated from BentoML's Python implementation.

use std::collections::VecDeque;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::{oneshot, Mutex, RwLock, mpsc};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::config::BatchConfig;
use crate::container::Batchable;
use crate::error::{BatchError, Result};
use crate::metrics::{MetricsCollector, MetricsHandle};
use crate::optimizer::Optimizer;

/// A job representing a single request in the batching queue
#[derive(Debug)]
struct Job<T, R> {
    /// Time when the request was enqueued
    enqueue_time: Instant,
    /// The request data
    data: T,
    /// Channel to send the result back
    response_tx: oneshot::Sender<Result<R>>,
    /// Time when dispatching started (for metrics)
    dispatch_time: Option<Instant>,
}

/// Semaphore for controlling concurrent batch processing
#[derive(Debug)]
struct NonBlockingSemaphore {
    permits: Arc<Mutex<usize>>,
}

impl NonBlockingSemaphore {
    fn new(permits: usize) -> Self {
        Self {
            permits: Arc::new(Mutex::new(permits)),
        }
    }
    
    async fn try_acquire(&self) -> bool {
        let mut permits = self.permits.lock().await;
        if *permits > 0 {
            *permits -= 1;
            true
        } else {
            false
        }
    }
    
    async fn is_locked(&self) -> bool {
        let permits = self.permits.lock().await;
        *permits == 0
    }
    
    async fn release(&self) {
        let mut permits = self.permits.lock().await;
        *permits += 1;
    }
}

/// Cork Dispatcher for adaptive batching
/// 
/// Implements the CORK algorithm to intelligently batch requests based on:
/// - Historical processing patterns
/// - Latency constraints
/// - Batch size limits
/// - Dynamic optimization
pub struct CorkDispatcher<T, R, F, Fut>
where
    T: Batchable + 'static,
    R: Send + 'static,
    F: Fn(Vec<T>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Vec<R>>> + Send + 'static,
{
    /// Configuration for the dispatcher
    config: BatchConfig,
    
    /// The batch processing function
    processor: Arc<F>,
    
    /// Queue for pending requests
    queue: Arc<RwLock<VecDeque<Job<T, R>>>>,
    
    /// Adaptive optimizer
    optimizer: Arc<RwLock<Optimizer>>,
    
    /// Semaphore for controlling concurrent batches
    semaphore: Arc<NonBlockingSemaphore>,
    
    /// Metrics collector
    metrics: Option<MetricsCollector>,
    
    /// Channel for shutdown signaling
    shutdown_tx: mpsc::UnboundedSender<()>,
    shutdown_rx: Arc<Mutex<mpsc::UnboundedReceiver<()>>>,
    
    /// Controller task handle
    controller_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    
    /// Wake-up notification for the controller
    wake_tx: mpsc::UnboundedSender<()>,
    wake_rx: Arc<Mutex<mpsc::UnboundedReceiver<()>>>,
}

impl<T, R, F, Fut> CorkDispatcher<T, R, F, Fut>
where
    T: Batchable + 'static,
    R: Send + 'static,
    F: Fn(Vec<T>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Vec<R>>> + Send + 'static,
{
    /// Create a new Cork Dispatcher
    pub fn new(config: BatchConfig, processor: F) -> Result<Self> {
        config.validate()?;
        
        let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel();
        let (wake_tx, wake_rx) = mpsc::unbounded_channel();
        
        let optimizer = Optimizer::new(
            config.max_latency,
            config.n_kept_samples,
            config.n_skipped_samples,
            config.refresh_interval,
        );
        
        let metrics = if config.enable_metrics {
            Some(MetricsCollector::new())
        } else {
            None
        };
        
        Ok(Self {
            config,
            processor: Arc::new(processor),
            queue: Arc::new(RwLock::new(VecDeque::new())),
            optimizer: Arc::new(RwLock::new(optimizer)),
            semaphore: Arc::new(NonBlockingSemaphore::new(1)),
            metrics,
            shutdown_tx,
            shutdown_rx: Arc::new(Mutex::new(shutdown_rx)),
            controller_handle: Arc::new(Mutex::new(None)),
            wake_tx,
            wake_rx: Arc::new(Mutex::new(wake_rx)),
        })
    }
    
    /// Start the dispatcher controller
    pub async fn start(&self) -> Result<()> {
        let mut handle_guard = self.controller_handle.lock().await;
        if handle_guard.is_some() {
            return Err(BatchError::Internal("Controller already started".to_string()));
        }
        
        let controller_task = self.spawn_controller().await;
        *handle_guard = Some(controller_task);
        
        info!("Cork Dispatcher started");
        Ok(())
    }
    
    /// Dispatch a single request for batch processing
    pub async fn dispatch(&self, data: T) -> Result<R> {
        // Ensure controller is running
        {
            let handle_guard = self.controller_handle.lock().await;
            if handle_guard.is_none() {
                return Err(BatchError::Internal("Controller not started".to_string()));
            }
        }
        
        let (response_tx, response_rx) = oneshot::channel();
        let job = Job {
            enqueue_time: Instant::now(),
            data,
            response_tx,
            dispatch_time: None,
        };
        
        // Add job to queue
        {
            let mut queue = self.queue.write().await;
            queue.push_back(job);
        }
        
        // Notify controller
        let _ = self.wake_tx.send(());
        
        // Wait for response
        match response_rx.await {
            Ok(result) => result,
            Err(_) => Err(BatchError::Internal("Response channel closed".to_string())),
        }
    }
    
    /// Shutdown the dispatcher
    pub async fn shutdown(&self) -> Result<()> {
        // Signal shutdown
        self.shutdown_tx.send(()).map_err(|_| {
            BatchError::Internal("Failed to send shutdown signal".to_string())
        })?;
        
        // Wait for controller to finish
        let mut handle_guard = self.controller_handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            let _ = handle.await;
        }
        
        // Cancel remaining jobs
        let mut queue = self.queue.write().await;
        while let Some(job) = queue.pop_front() {
            let _ = job.response_tx.send(Err(BatchError::Shutdown));
        }
        
        info!("Cork Dispatcher shutdown complete");
        Ok(())
    }
    
    /// Get metrics snapshot if metrics are enabled
    pub fn metrics(&self) -> Option<crate::metrics::MetricsSnapshot> {
        self.metrics.as_ref().map(|m| m.snapshot())
    }
    
    /// Get current optimization parameters
    pub async fn optimization_params(&self) -> (f64, f64, f64) {
        let optimizer = self.optimizer.read().await;
        optimizer.parameters()
    }
    
    /// Spawn the controller task
    async fn spawn_controller(&self) -> JoinHandle<()> {
        let queue = Arc::clone(&self.queue);
        let optimizer = Arc::clone(&self.optimizer);
        let semaphore = Arc::clone(&self.semaphore);
        let processor = Arc::clone(&self.processor);
        let config = self.config.clone();
        let shutdown_rx = Arc::clone(&self.shutdown_rx);
        let wake_rx = Arc::clone(&self.wake_rx);
        let metrics = self.metrics.as_ref().map(|m| m.handle());
        
        tokio::spawn(async move {
            if let Err(e) = Self::controller_loop(
                queue,
                optimizer,
                semaphore,
                processor,
                config,
                shutdown_rx,
                wake_rx,
                metrics,
            ).await {
                error!("Controller loop error: {}", e);
            }
        })
    }
    
    /// Main controller loop implementing the CORK algorithm
    async fn controller_loop(
        queue: Arc<RwLock<VecDeque<Job<T, R>>>>,
        optimizer: Arc<RwLock<Optimizer>>,
        semaphore: Arc<NonBlockingSemaphore>,
        processor: Arc<F>,
        config: BatchConfig,
        shutdown_rx: Arc<Mutex<mpsc::UnboundedReceiver<()>>>,
        wake_rx: Arc<Mutex<mpsc::UnboundedReceiver<()>>>,
        metrics: Option<MetricsHandle>,
    ) -> Result<()> {
        // Training phase
        info!("Starting dispatcher training phase...");
        
        // Warm up the model
        Self::train_optimizer(
            &queue, &optimizer, &semaphore, &processor, &config, &metrics,
            config.n_skipped_samples, config.n_skipped_samples + 6, 1
        ).await?;
        
        debug!("Finished warming up model");
        
        // Training with different batch sizes
        Self::train_optimizer(&queue, &optimizer, &semaphore, &processor, &config, &metrics, 6, 1, 1).await?;
        Self::train_optimizer(&queue, &optimizer, &semaphore, &processor, &config, &metrics, 5, 1, 2).await?;
        Self::train_optimizer(&queue, &optimizer, &semaphore, &processor, &config, &metrics, 3, 1, 3).await?;
        
        // Check if latency constraints are achievable
        {
            let opt = optimizer.read().await;
            let (o_a, o_b, _) = opt.parameters();
            if o_a + o_b >= config.max_latency.as_secs_f64() {
                warn!(
                    "Detected max latency that may be too low for serving. \
                    Consider increasing max_latency if you encounter many timeout errors."
                );
            }
        }
        
        info!("Dispatcher training complete, starting main loop");
        
        // Main processing loop
        loop {
            tokio::select! {
                // Check for shutdown signal
                _ = async {
                    let mut rx = shutdown_rx.lock().await;
                    rx.recv().await
                } => {
                    info!("Received shutdown signal");
                    break;
                }
                // Process wake notifications
                _ = async {
                    let mut rx = wake_rx.lock().await;
                    rx.recv().await
                } => {
                    // Process current queue
                    if let Err(e) = Self::process_queue(
                        &queue, &optimizer, &semaphore, &processor, &config, &metrics
                    ).await {
                        error!("Error processing queue: {}", e);
                    }
                }
                // Regular tick processing
                _ = sleep(config.tick_interval) => {
                    // Regular tick processing
                    if let Err(e) = Self::process_queue(
                        &queue, &optimizer, &semaphore, &processor, &config, &metrics
                    ).await {
                        error!("Error in tick processing: {}", e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Training phase for the optimizer
    async fn train_optimizer(
        queue: &Arc<RwLock<VecDeque<Job<T, R>>>>,
        optimizer: &Arc<RwLock<Optimizer>>,
        semaphore: &Arc<NonBlockingSemaphore>,
        processor: &Arc<F>,
        config: &BatchConfig,
        metrics: &Option<MetricsHandle>,
        num_required_reqs: usize,
        num_reqs_to_train: usize,
        training_batch_size: usize,
    ) -> Result<()> {
        let actual_batch_size = training_batch_size.min(config.max_batch_size);
        let mut req_count = 0;
        
        while req_count < num_reqs_to_train {
            // Wait for requests in queue
            loop {
                let queue_len = {
                    let q = queue.read().await;
                    q.len()
                };
                
                if queue_len > 0 {
                    break;
                }
                
                sleep(config.tick_interval).await;
            }
            
            let (queue_len, first_wait_time) = {
                let q = queue.read().await;
                let len = q.len();
                let wait_time = q.front().map(|job| job.enqueue_time.elapsed()).unwrap_or_default();
                (len, wait_time)
            };
            
            // Cancel requests that have timed out
            if queue_len > num_required_reqs - req_count && first_wait_time >= config.max_latency {
                let mut q = queue.write().await;
                if let Some(job) = q.pop_front() {
                    let _ = job.response_tx.send(Err(BatchError::TimeoutExceeded {
                        expected_ms: config.max_latency.as_millis() as u64,
                        actual_ms: first_wait_time.as_millis() as u64,
                    }));
                }
                continue;
            }
            
            // Wait for optimal batch size during training
            if actual_batch_size > 1 {
                let opt = optimizer.read().await;
                let estimated_duration = opt.estimate_duration(actual_batch_size);
                
                if queue_len < actual_batch_size &&
                   (estimated_duration + first_wait_time) <= Duration::from_secs_f64(config.max_latency.as_secs_f64() * 0.95) {
                    drop(opt);
                    sleep(config.tick_interval).await;
                    continue;
                }
            }
            
            // Wait for semaphore
            if semaphore.is_locked().await {
                sleep(config.tick_interval).await;
                continue;
            }
            
            // Process batch
            if semaphore.try_acquire().await {
                req_count += 1;
                let jobs = Self::extract_jobs(queue, actual_batch_size).await;
                let _ = Self::process_batch(jobs, processor, optimizer, metrics, true).await;
                semaphore.release().await;
            }
        }
        
        Ok(())
    }
    
    /// Process the current queue based on CORK algorithm
    async fn process_queue(
        queue: &Arc<RwLock<VecDeque<Job<T, R>>>>,
        optimizer: &Arc<RwLock<Optimizer>>,
        semaphore: &Arc<NonBlockingSemaphore>,
        processor: &Arc<F>,
        config: &BatchConfig,
        metrics: &Option<MetricsHandle>,
    ) -> Result<()> {
        let (queue_len, first_wait_time, last_wait_time) = {
            let q = queue.read().await;
            let len = q.len();
            
            if len == 0 {
                return Ok(());
            }
            
            let first_wait = q.front().unwrap().enqueue_time.elapsed();
            let last_wait = q.back().unwrap().enqueue_time.elapsed();
            (len, first_wait, last_wait)
        };
        
        let (o_a, o_b, wait_time) = {
            let opt = optimizer.read().await;
            opt.parameters()
        };
        
        let estimated_latency = first_wait_time + Duration::from_secs_f64(o_a * queue_len as f64 + o_b);
        
        // Cancel requests that would exceed latency
        if queue_len > 1 && estimated_latency >= config.max_latency {
            let mut q = queue.write().await;
            if let Some(job) = q.pop_front() {
                let _ = job.response_tx.send(Err(BatchError::TimeoutExceeded {
                    expected_ms: config.max_latency.as_millis() as u64,
                    actual_ms: estimated_latency.as_millis() as u64,
                }));
                if let Some(m) = metrics {
                    m.record_latency(estimated_latency);
                }
            }
            return Ok(());
        }
        
        // Check if semaphore is available
        if semaphore.is_locked().await {
            // Cancel single request if it has timed out
            if queue_len == 1 && first_wait_time >= config.max_latency {
                let mut q = queue.write().await;
                if let Some(job) = q.pop_front() {
                    let _ = job.response_tx.send(Err(BatchError::TimeoutExceeded {
                        expected_ms: config.max_latency.as_millis() as u64,
                        actual_ms: first_wait_time.as_millis() as u64,
                    }));
                }
            }
            return Ok(());
        }
        
        // CORK algorithm decision
        let should_wait = queue_len < config.max_batch_size &&
            queue_len as f64 * (last_wait_time.as_secs_f64() + config.tick_interval.as_secs_f64() + o_a) 
            <= wait_time * 0.95;
        
        if should_wait {
            return Ok(());
        }
        
        // Process the batch
        if semaphore.try_acquire().await {
            let batch_size = queue_len.min(config.max_batch_size);
            let jobs = Self::extract_jobs(queue, batch_size).await;
            let _ = Self::process_batch(jobs, processor, optimizer, metrics, false).await;
            semaphore.release().await;
        }
        
        Ok(())
    }
    
    /// Extract jobs from the queue for batch processing
    async fn extract_jobs(
        queue: &Arc<RwLock<VecDeque<Job<T, R>>>>,
        max_count: usize,
    ) -> Vec<Job<T, R>> {
        let mut q = queue.write().await;
        let mut jobs = Vec::new();
        let mut current_batch_size = 0;
        
        while jobs.len() < max_count && !q.is_empty() {
            if let Some(job) = q.front() {
                let job_batch_size = job.data.batch_size();
                
                if current_batch_size + job_batch_size <= max_count {
                    let mut job = q.pop_front().unwrap();
                    job.dispatch_time = Some(Instant::now());
                    current_batch_size += job_batch_size;
                    jobs.push(job);
                } else {
                    break;
                }
            }
        }
        
        jobs
    }
    
    /// Process a batch of jobs
    async fn process_batch(
        jobs: Vec<Job<T, R>>,
        processor: &Arc<F>,
        optimizer: &Arc<RwLock<Optimizer>>,
        metrics: &Option<MetricsHandle>,
        is_training: bool,
    ) -> Result<()> {
        if jobs.is_empty() {
            return Ok(());
        }
        
        let batch_size = jobs.len();
        let start_time = Instant::now();
        let first_enqueue_time = jobs[0].enqueue_time;
        let wait_time = start_time - first_enqueue_time;
        
        debug!("Processing batch of size: {}", batch_size);
        
        // Extract data for processing
        let batch_data: Vec<T> = jobs.iter().map(|job| job.data.clone()).collect();
        
        // Process the batch
        let process_result = processor(batch_data).await;
        let processing_duration = start_time.elapsed();
        
        // Handle results
        match process_result {
            Ok(results) => {
                if results.len() != jobs.len() {
                    error!("Batch processor returned {} results for {} jobs", results.len(), jobs.len());
                    // Send errors to all jobs
                    for job in jobs {
                        let _ = job.response_tx.send(Err(BatchError::ProcessingError(
                            "Batch processor returned incorrect number of results".to_string()
                        )));
                    }
                } else {
                    // Send results back to jobs
                    for (job, result) in jobs.into_iter().zip(results.into_iter()) {
                        let total_latency = job.enqueue_time.elapsed();
                        let _ = job.response_tx.send(Ok(result));
                        
                        // Record individual latency
                        if let Some(m) = metrics {
                            m.record_latency(total_latency);
                        }
                    }
                }
            }
            Err(e) => {
                // Send error to all jobs
                for job in jobs {
                    let _ = job.response_tx.send(Err(e.clone()));
                }
                
                if let Some(m) = metrics {
                    m.record_processing_error();
                }
            }
        }
        
        // Update optimizer (skip for training requests as mentioned in Python code)
        if !is_training {
            let mut opt = optimizer.write().await;
            opt.log_batch(batch_size, wait_time, processing_duration);
        }
        
        // Record metrics
        if let Some(m) = metrics {
            m.record_batch(batch_size, processing_duration, wait_time);
        }
        
        Ok(())
    }
}

impl<T, R, F, Fut> Drop for CorkDispatcher<T, R, F, Fut>
where
    T: Batchable + 'static,
    R: Send + 'static,
    F: Fn(Vec<T>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Vec<R>>> + Send + 'static,
{
    fn drop(&mut self) {
        // Signal shutdown
        let _ = self.shutdown_tx.send(());
    }
}
