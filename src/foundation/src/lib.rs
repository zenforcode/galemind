pub mod api;

pub use api::fake::FakeInferenceProcessor;
pub use api::inference::{InferenceRequest, InferenceResponse};

use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct InferenceServerConfig {
    pub rest_hostname: String,
    pub rest_port: u16,
    pub grpc_hostname: String,
    pub grpc_port: u16,
}

#[async_trait]
pub trait InferenceServerBuilder: Sized + Send + Sync {
    fn configure(context: InferenceServerConfig) -> Self;
    async fn start(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
