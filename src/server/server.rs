use axum::{Router, routing::get};
use std::{collections::HashMap, net::SocketAddr};
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
        let routeTable =
            HashMap::from([("/", Self::root_handler), ("/health", Self::health_handler)]);

        for route in routeTable.iter() {}
        self.router
            .route("/", get(Self::root_handler))
            .layer(TraceLayer::new_for_http());

        axum::Server::bind(&self.addr)
            .serve(&self.router.into_make_service())
            .await
    }

    async fn root_handler() -> &'static str {
        "Hello from Axum Builder!"
    }

    async fn health_handler() -> &'static str {
        "Hello from Axum Builder!"
    }
}
