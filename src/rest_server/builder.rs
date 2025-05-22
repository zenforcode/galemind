use crate::rest_server::healthcheck::new_health_check_router;
use crate::rest_server::model::new_model_router;
use anyhow::Result;
use axum::{Router, serve};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
pub struct RestServerBuilder {
    addr: SocketAddr,
    app: Router,
}

impl RestServerBuilder {
    pub fn configure(addr: &str) -> Self {
        let addr = addr.parse().expect("Invalid address");
        let app = Router::new()
            .nest("/v2/health", new_health_check_router())
            .nest("/v2/models", new_model_router())
            .layer(TraceLayer::new_for_http());

        Self { addr, app }
    }
    pub async fn start(self) -> Result<()> {
        let listener = tokio::net::TcpListener::bind(self.addr)
            .await
            .expect("Failed to bind address");

        println!("Listening on {}", listener.local_addr().unwrap());
        serve(listener, self.app).await?;
        Ok(())
    }
}
