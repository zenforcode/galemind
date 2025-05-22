use crate::rest_server::handlers::{liveness_handler, readiness_handler};
use anyhow::Result;
use axum::{Router, routing::get, serve};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
pub struct RestServerBuilder {
    addr: SocketAddr,
    app: Router,
}

impl RestServerBuilder {
    pub fn with_routes(addr: &str) -> Self {
        let addr = addr.parse().expect("Invalid address");
        let app = Router::new()
            .route("/v2/health/live", get(liveness_handler))
            .route("/v2/health/ready", get(readiness_handler))
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
