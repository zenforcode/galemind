pub mod fake;
pub mod inference;
pub mod tensor;
pub mod config;
pub mod dispatcher;
pub mod optimizer;
pub mod container;
pub mod error;
pub mod metrics;

pub use config::BatchConfig;
pub use dispatcher::CorkDispatcher;
pub use error::{BatchError, Result};
pub use optimizer::Optimizer;

/// Default configuration constants
pub mod defaults {
    use std::time::Duration;
    
    pub const MAX_BATCH_SIZE: usize = 32;
    pub const MAX_LATENCY: Duration = Duration::from_millis(100);
    pub const TICK_INTERVAL: Duration = Duration::from_millis(1);
    pub const N_KEPT_SAMPLES: usize = 50;
    pub const N_SKIPPED_SAMPLES: usize = 2;
    pub const REFRESH_INTERVAL: Duration = Duration::from_secs(5);
}
