//! Metrics collection for adaptive batching
//! 
//! This module provides basic metrics collection capabilities for monitoring
//! batch processing performance and optimization behavior.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Metrics collector for batch processing
#[derive(Debug, Default)]
pub struct MetricsCollector {
    inner: Arc<Mutex<MetricsInner>>,
}

#[derive(Debug, Default)]
struct MetricsInner {
    // Batch size histogram
    batch_size_counts: HashMap<usize, u64>,
    
    // Processing time statistics
    total_processing_time: Duration,
    processing_count: u64,
    
    // Latency statistics
    total_latency: Duration,
    latency_count: u64,
    
    // Wait time statistics
    total_wait_time: Duration,
    wait_count: u64,
    
    // Optimization statistics
    parameter_updates: u64,
    last_update: Option<Instant>,
    
    // Error counts
    timeout_errors: u64,
    processing_errors: u64,
    
    // Throughput tracking
    requests_processed: u64,
    start_time: Option<Instant>,
}

/// Snapshot of current metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    /// Histogram of batch sizes
    pub batch_size_histogram: HashMap<usize, u64>,
    
    /// Average processing time per batch
    pub avg_processing_time_ms: f64,
    
    /// Average latency per request
    pub avg_latency_ms: f64,
    
    /// Average wait time before processing
    pub avg_wait_time_ms: f64,
    
    /// Number of optimization parameter updates
    pub parameter_updates: u64,
    
    /// Number of timeout errors
    pub timeout_errors: u64,
    
    /// Number of processing errors
    pub processing_errors: u64,
    
    /// Total requests processed
    pub requests_processed: u64,
    
    /// Requests per second (if duration available)
    pub requests_per_second: Option<f64>,
    
    /// Time since metrics collection started
    pub uptime_seconds: Option<f64>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        let mut inner = MetricsInner::default();
        inner.start_time = Some(Instant::now());
        
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
    
    /// Record a batch processing event
    pub fn record_batch(&self, batch_size: usize, processing_time: Duration, wait_time: Duration) {
        if let Ok(mut inner) = self.inner.lock() {
            // Update batch size histogram
            *inner.batch_size_counts.entry(batch_size).or_insert(0) += 1;
            
            // Update processing time statistics
            inner.total_processing_time += processing_time;
            inner.processing_count += 1;
            
            // Update wait time statistics
            inner.total_wait_time += wait_time;
            inner.wait_count += 1;
            
            // Update request count
            inner.requests_processed += batch_size as u64;
        }
    }
    
    /// Record a request latency
    pub fn record_latency(&self, latency: Duration) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.total_latency += latency;
            inner.latency_count += 1;
        }
    }
    
    /// Record an optimization parameter update
    pub fn record_parameter_update(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.parameter_updates += 1;
            inner.last_update = Some(Instant::now());
        }
    }
    
    /// Record a timeout error
    pub fn record_timeout_error(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.timeout_errors += 1;
        }
    }
    
    /// Record a processing error
    pub fn record_processing_error(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.processing_errors += 1;
        }
    }
    
    /// Get a snapshot of current metrics
    pub fn snapshot(&self) -> MetricsSnapshot {
        if let Ok(inner) = self.inner.lock() {
            let avg_processing_time_ms = if inner.processing_count > 0 {
                inner.total_processing_time.as_secs_f64() * 1000.0 / inner.processing_count as f64
            } else {
                0.0
            };
            
            let avg_latency_ms = if inner.latency_count > 0 {
                inner.total_latency.as_secs_f64() * 1000.0 / inner.latency_count as f64
            } else {
                0.0
            };
            
            let avg_wait_time_ms = if inner.wait_count > 0 {
                inner.total_wait_time.as_secs_f64() * 1000.0 / inner.wait_count as f64
            } else {
                0.0
            };
            
            let (requests_per_second, uptime_seconds) = if let Some(start_time) = inner.start_time {
                let uptime = start_time.elapsed();
                let uptime_secs = uptime.as_secs_f64();
                let rps = if uptime_secs > 0.0 {
                    Some(inner.requests_processed as f64 / uptime_secs)
                } else {
                    None
                };
                (rps, Some(uptime_secs))
            } else {
                (None, None)
            };
            
            MetricsSnapshot {
                batch_size_histogram: inner.batch_size_counts.clone(),
                avg_processing_time_ms,
                avg_latency_ms,
                avg_wait_time_ms,
                parameter_updates: inner.parameter_updates,
                timeout_errors: inner.timeout_errors,
                processing_errors: inner.processing_errors,
                requests_processed: inner.requests_processed,
                requests_per_second,
                uptime_seconds,
            }
        } else {
            // Return empty snapshot if lock fails
            MetricsSnapshot {
                batch_size_histogram: HashMap::new(),
                avg_processing_time_ms: 0.0,
                avg_latency_ms: 0.0,
                avg_wait_time_ms: 0.0,
                parameter_updates: 0,
                timeout_errors: 0,
                processing_errors: 0,
                requests_processed: 0,
                requests_per_second: None,
                uptime_seconds: None,
            }
        }
    }
    
    /// Reset all metrics
    pub fn reset(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            *inner = MetricsInner::default();
            inner.start_time = Some(Instant::now());
        }
    }
    
    /// Get the metrics collector as a cloneable handle
    pub fn handle(&self) -> MetricsHandle {
        MetricsHandle {
            collector: self.inner.clone(),
        }
    }
}

/// A lightweight handle to the metrics collector that can be cloned and shared
#[derive(Debug, Clone)]
pub struct MetricsHandle {
    collector: Arc<Mutex<MetricsInner>>,
}

impl MetricsHandle {
    /// Record a batch processing event
    pub fn record_batch(&self, batch_size: usize, processing_time: Duration, wait_time: Duration) {
        if let Ok(mut inner) = self.collector.lock() {
            *inner.batch_size_counts.entry(batch_size).or_insert(0) += 1;
            inner.total_processing_time += processing_time;
            inner.processing_count += 1;
            inner.total_wait_time += wait_time;
            inner.wait_count += 1;
            inner.requests_processed += batch_size as u64;
        }
    }
    
    /// Record a request latency
    pub fn record_latency(&self, latency: Duration) {
        if let Ok(mut inner) = self.collector.lock() {
            inner.total_latency += latency;
            inner.latency_count += 1;
        }
    }
    
    /// Record a processing error
    pub fn record_processing_error(&self) {
        if let Ok(mut inner) = self.collector.lock() {
            inner.processing_errors += 1;
        }
    }
}

impl Default for MetricsSnapshot {
    fn default() -> Self {
        Self {
            batch_size_histogram: HashMap::new(),
            avg_processing_time_ms: 0.0,
            avg_latency_ms: 0.0,
            avg_wait_time_ms: 0.0,
            parameter_updates: 0,
            timeout_errors: 0,
            processing_errors: 0,
            requests_processed: 0,
            requests_per_second: None,
            uptime_seconds: None,
        }
    }
}
