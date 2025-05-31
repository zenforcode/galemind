pub mod api;

pub use api::fake::FakeInferenceProcessor;
pub use api::inference::{InferenceRequest, InferenceResponse};

use async_trait::async_trait;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct InferenceServerContext {
    pub hostname: String,
    pub port: u16,
}

#[async_trait]
pub trait InferenceServerBuilder: Sized + Send + Sync {
    fn configure(context: InferenceServerContext) -> Self;
    async fn start(self) -> Result<(), Box<dyn std::error::Error>>;
}
