use hyper::Server;

use crate::rest_server::handlers::{liveness_handler, readiness_handler};
use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

pub struct RestServerBuilder {
    addr: SocketAddr,
}

impl RestServerBuilder {
    pub fn new(addr: &str) -> Self {
        let addr = addr.parse().expect("Invalid address");
        Self { addr }
    }

    pub async fn build(self) -> Result<(), hyper::Error> {
        let mut router = Router::new();

        router = router
            .route("/health/live", get(liveness_handler))
            .route("/health/ready", get(readiness_handler))
            .layer(TraceLayer::new_for_http());
        Server::bind(&self.addr)
            .serve(router.into_make_service())
            .await
    }
}
