mod data_model;
mod metadata_model;
mod healthcheck;
mod model;
mod server;

use crate::healthcheck::new_health_check_router;
use crate::model::new_model_router;
use crate::server::new_server_router;
use anyhow::Result;
use async_trait::async_trait;
use axum::{Router, serve};
use foundation::{InferenceServerBuilder, InferenceServerConfig};
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
pub struct RestServerBuilder {
    addr: SocketAddr,
    app: Router,
}
#[async_trait]
impl InferenceServerBuilder for RestServerBuilder {
    fn configure(context: InferenceServerConfig) -> Self {
        let addr = format!("{}:{}", context.rest_hostname, context.rest_port)
            .parse()
            .expect("Invalid Host/Port");
        let app = Router::new()
            .nest("/{version}", new_server_router())
            .nest("/{version}/health", new_health_check_router())
            .nest("/{version}/models", new_model_router())
            .layer(TraceLayer::new_for_http());

        Self { addr, app }
    }
    async fn start(self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let listener = TcpListener::bind(self.addr)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        let local_addr = listener.local_addr()?;
        println!("Rest Server listening on {}", local_addr);
        serve(listener, self.app)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        Ok(())
    }
}
