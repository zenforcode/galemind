//! Configuration for adaptive batching

use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::defaults;
use crate::error::{BatchError, Result};

/// Configuration for the adaptive batching dispatcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    /// Maximum batch size allowed
    pub max_batch_size: usize,
    
    /// Maximum latency allowed for processing requests
    pub max_latency: Duration,
    
    /// Interval between dispatcher ticks
    pub tick_interval: Duration,
    
    /// Number of samples to keep for optimization
    pub n_kept_samples: usize,
    
    /// Number of initial samples to skip for optimization
    pub n_skipped_samples: usize,
    
    /// Interval for refreshing optimization parameters
    pub refresh_interval: Duration,
    
    /// Enable metrics collection
    pub enable_metrics: bool,
    
    /// Batch dimensions (input_dim, output_dim)
    pub batch_dims: (usize, usize),
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_batch_size: defaults::MAX_BATCH_SIZE,
            max_latency: defaults::MAX_LATENCY,
            tick_interval: defaults::TICK_INTERVAL,
            n_kept_samples: defaults::N_KEPT_SAMPLES,
            n_skipped_samples: defaults::N_SKIPPED_SAMPLES,
            refresh_interval: defaults::REFRESH_INTERVAL,
            enable_metrics: false,
            batch_dims: (0, 0),
        }
    }
}

impl BatchConfig {
    /// Create a new batch configuration with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the maximum batch size
    pub fn max_batch_size(mut self, size: usize) -> Self {
        self.max_batch_size = size;
        self
    }
    
    /// Set the maximum latency in milliseconds
    pub fn max_latency_ms(mut self, ms: u64) -> Self {
        self.max_latency = Duration::from_millis(ms);
        self
    }
    
    /// Set the maximum latency
    pub fn max_latency(mut self, duration: Duration) -> Self {
        self.max_latency = duration;
        self
    }
    
    /// Set the tick interval
    pub fn tick_interval(mut self, duration: Duration) -> Self {
        self.tick_interval = duration;
        self
    }
    
    /// Set the number of samples to keep for optimization
    pub fn n_kept_samples(mut self, n: usize) -> Self {
        self.n_kept_samples = n;
        self
    }
    
    /// Set the number of initial samples to skip
    pub fn n_skipped_samples(mut self, n: usize) -> Self {
        self.n_skipped_samples = n;
        self
    }
    
    /// Set the refresh interval for optimization parameters
    pub fn refresh_interval(mut self, duration: Duration) -> Self {
        self.refresh_interval = duration;
        self
    }
    
    /// Enable or disable metrics collection
    pub fn enable_metrics(mut self, enable: bool) -> Self {
        self.enable_metrics = enable;
        self
    }
    
    /// Set batch dimensions
    pub fn batch_dims(mut self, input_dim: usize, output_dim: usize) -> Self {
        self.batch_dims = (input_dim, output_dim);
        self
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.max_batch_size == 0 {
            return Err(BatchError::ConfigError(
                "max_batch_size must be greater than 0".to_string()
            ));
        }
        
        if self.max_latency.is_zero() {
            return Err(BatchError::ConfigError(
                "max_latency must be greater than 0".to_string()
            ));
        }
        
        if self.tick_interval.is_zero() {
            return Err(BatchError::ConfigError(
                "tick_interval must be greater than 0".to_string()
            ));
        }
        
        if self.n_kept_samples == 0 {
            return Err(BatchError::ConfigError(
                "n_kept_samples must be greater than 0".to_string()
            ));
        }
        
        Ok(())
    }
}
