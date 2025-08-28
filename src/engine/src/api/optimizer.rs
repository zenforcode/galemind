//! Optimizer for adaptive batching parameters
//! 
//! This module implements the optimization logic that analyzes historical data
//! to dynamically adjust batching parameters for optimal performance.

use std::collections::VecDeque;
use std::time::{Duration, Instant};
use tracing::{debug, warn};

/// Sample data for optimization analysis
#[derive(Debug, Clone)]
pub struct OptimizationSample {
    /// Batch size for this sample
    pub batch_size: usize,
    /// Processing duration for this batch
    pub duration: Duration,
    /// Wait time before processing started
    pub wait_time: Duration,
    /// Timestamp when the sample was recorded
    pub timestamp: Instant,
}

/// Token bucket for rate limiting optimization parameter updates
#[derive(Debug)]
struct TokenBucket {
    tokens: f64,
    capacity: f64,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: f64) -> Self {
        Self {
            tokens: capacity,
            capacity,
            last_refill: Instant::now(),
        }
    }
    
    fn consume(&mut self, tokens: f64, refill_rate: f64, min_interval: f64) -> bool {
        let now = Instant::now();
        let time_passed = now.duration_since(self.last_refill).as_secs_f64();
        
        if time_passed >= min_interval {
            self.tokens = (self.tokens + time_passed * refill_rate).min(self.capacity);
            self.last_refill = now;
        }
        
        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }
}

/// Adaptive optimizer for batching parameters
/// 
/// Analyzes historical processing data to optimize batch dispatching decisions.
/// Assumes the processing duration follows: duration = o_a * batch_size + o_b
#[derive(Debug)]
pub struct Optimizer {
    /// Statistical samples for analysis
    samples: VecDeque<OptimizationSample>,
    
    /// Linear coefficient (processing time per item)
    pub o_a: f64,
    
    /// Constant coefficient (base processing time)
    pub o_b: f64,
    
    /// Average wait time before processing
    pub wait_time: f64,
    
    /// Token bucket for rate limiting parameter updates
    refresh_bucket: TokenBucket,
    
    /// Counter for processed batches
    batch_counter: usize,
    
    /// Configuration
    max_latency: Duration,
    n_kept_samples: usize,
    n_skipped_samples: usize,
    refresh_interval: Duration,
}

impl Optimizer {
    /// Create a new optimizer
    pub fn new(
        max_latency: Duration,
        n_kept_samples: usize,
        n_skipped_samples: usize,
        refresh_interval: Duration,
    ) -> Self {
        let max_latency_secs = max_latency.as_secs_f64();
        
        Self {
            samples: VecDeque::with_capacity(n_kept_samples),
            // Initialize with conservative estimates
            o_a: (2.0_f64).min(max_latency_secs * 2.0 / 30.0),
            o_b: (1.0_f64).min(max_latency_secs * 1.0 / 30.0),
            wait_time: 0.0,
            refresh_bucket: TokenBucket::new(2.0),
            batch_counter: 0,
            max_latency,
            n_kept_samples,
            n_skipped_samples,
            refresh_interval,
        }
    }
    
    /// Log a batch processing event for optimization
    pub fn log_batch(&mut self, batch_size: usize, wait_time: Duration, duration: Duration) {
        self.batch_counter += 1;
        
        // Skip initial samples as they might be inaccurate
        if self.batch_counter <= self.n_skipped_samples + 4 {
            if self.batch_counter <= self.n_skipped_samples {
                return;
            }
        }
        
        let sample = OptimizationSample {
            batch_size,
            duration,
            wait_time,
            timestamp: Instant::now(),
        };
        
        // Add sample to the collection
        if self.samples.len() >= self.n_kept_samples {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);
        
        // Try to refresh parameters
        let refresh_rate = 1.0 / self.refresh_interval.as_secs_f64();
        if self.refresh_bucket.consume(1.0, refresh_rate, 1.0) {
            self.refresh_parameters();
        }
    }
    
    /// Refresh optimization parameters based on collected samples
    fn refresh_parameters(&mut self) {
        if self.samples.is_empty() {
            debug!("No samples available for parameter refresh");
            return;
        }
        
        // Prepare data for least squares regression
        let mut x_matrix = Vec::new();
        let mut y_vector = Vec::new();
        let mut wait_times = Vec::new();
        
        for sample in &self.samples {
            // X matrix: [batch_size, 1] for linear regression
            x_matrix.push([sample.batch_size as f64, 1.0]);
            // Y vector: duration in seconds
            y_vector.push(sample.duration.as_secs_f64());
            wait_times.push(sample.wait_time.as_secs_f64());
        }
        
        // Perform least squares regression using normal equations
        if let Some((new_o_a, new_o_b)) = self.least_squares_regression(&x_matrix, &y_vector) {
            self.o_a = new_o_a.max(0.000001); // Ensure positive
            self.o_b = new_o_b.max(0.0);      // Ensure non-negative
            
            // Calculate average wait time
            if !wait_times.is_empty() {
                self.wait_time = wait_times.iter().sum::<f64>() / wait_times.len() as f64;
                self.wait_time = self.wait_time.max(0.0);
            }
            
            debug!(
                "Optimizer parameters updated: o_a={:.6}, o_b={:.6}, wait_time={:.6}",
                self.o_a, self.o_b, self.wait_time
            );
        } else {
            warn!("Failed to perform least squares regression for parameter optimization");
        }
    }
    
    /// Perform least squares regression to fit duration = o_a * batch_size + o_b
    fn least_squares_regression(&self, x: &[[f64; 2]], y: &[f64]) -> Option<(f64, f64)> {
        if x.len() != y.len() || x.is_empty() {
            return None;
        }
        
        let n = x.len() as f64;
        
        // Calculate sums for normal equations
        let sum_x1 = x.iter().map(|row| row[0]).sum::<f64>();
        let sum_x1_squared = x.iter().map(|row| row[0] * row[0]).sum::<f64>();
        let sum_y = y.iter().sum::<f64>();
        let sum_x1_y = x.iter().zip(y.iter()).map(|(row, &yi)| row[0] * yi).sum::<f64>();
        
        // Solve normal equations: X^T * X * beta = X^T * y
        // For our case: [sum_x1^2, sum_x1] [o_a] = [sum_x1_y]
        //               [sum_x1,   n     ] [o_b]   [sum_y  ]
        
        let det = sum_x1_squared * n - sum_x1 * sum_x1;
        
        if det.abs() < 1e-10 {
            return None; // Matrix is singular
        }
        
        let o_a = (n * sum_x1_y - sum_x1 * sum_y) / det;
        let o_b = (sum_x1_squared * sum_y - sum_x1 * sum_x1_y) / det;
        
        Some((o_a, o_b))
    }
    
    /// Estimate processing duration for a given batch size
    pub fn estimate_duration(&self, batch_size: usize) -> Duration {
        let duration_secs = self.o_a * batch_size as f64 + self.o_b;
        Duration::from_secs_f64(duration_secs.max(0.0))
    }
    
    /// Get the current wait time estimate
    pub fn current_wait_time(&self) -> Duration {
        Duration::from_secs_f64(self.wait_time)
    }
    
    /// Check if the estimated processing time would exceed max latency
    pub fn would_exceed_latency(&self, batch_size: usize, current_wait: Duration) -> bool {
        let estimated_duration = self.estimate_duration(batch_size);
        let total_latency = current_wait + estimated_duration;
        total_latency > self.max_latency
    }
    
    /// Get the number of samples collected
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }
    
    /// Get current optimization parameters for monitoring
    pub fn parameters(&self) -> (f64, f64, f64) {
        (self.o_a, self.o_b, self.wait_time)
    }
}
