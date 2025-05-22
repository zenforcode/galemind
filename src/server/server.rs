use axum::{routing::get, Router};
use std::{net::SocketAddr};
use tower_http::trace::TraceLayer;

pub struct RestServerBuilder {
    addr: SocketAddr,
    router: Router,
}

impl RestServerBuilder {
    pub fn new(addr: &str) -> Self {
        let addr = addr.parse().expect("Invalid address");
        Self {
            addr,
            router: Router::new(),
        }
    }

    pub async fn build(self) -> Result<(), hyper::Error> {
        // Instead of route table, register routes directly
        let router = self
            .router
            .route("/", get(Self::root_handler))
            .route("/health", get(Self::health_handler))
            .layer(TraceLayer::new_for_http());

        axum::Server::bind(&self.addr)
            .serve(router.into_make_service())
            .await
    }

    async fn root_handler() -> &'static str {
        "Hello from Axum Builder!"
    }

    async fn health_handler() -> &'static str {
        "Hello from Axum Builder!"
    }
}

