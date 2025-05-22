use axum::{
    body::Body,
    http::Response,
    response::IntoResponse,
    routing::get,
    Router,
};
use crate::rest_server::handlers::{readiness_handler, liveness_handler};
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

        axum::Server::bind(&self.addr)
            .serve(router.into_make_service())
            .await
    }
}