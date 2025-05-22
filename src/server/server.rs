use axum::{
    body::Body,
    extract::Request,
    http::Response,
    routing::get,
    Router,
};
use std::{collections::HashMap, net::SocketAddr};
use tower_http::trace::TraceLayer;
use axum::routing::BoxRoute;

pub struct RestServerBuilder {
    addr: SocketAddr,
    routes: HashMap<&'static str, BoxRoute<Body>>,
}

impl RestServerBuilder {
    pub fn new(addr: &str) -> Self {
        let addr = addr.parse().expect("Invalid address");
        Self {
            addr,
            routes: HashMap::new(),
        }
    }

    pub fn add_route<F, T>(mut self, path: &'static str, handler: F) -> Self
    where
        F: axum::handler::Handler<Body, T> + Clone + Send + 'static,
        T: Send + 'static,
        F::Future: Send + 'static,
    {
        self.routes.insert(path, axum::routing::get(handler).boxed());
        self
    }

    pub async fn build(self) -> Result<(), hyper::Error> {
        let mut router = Router::new();

        for (path, handler) in self.routes {
            router = router.route(path, handler);
        }

        router = router.layer(TraceLayer::new_for_http());

        axum::Server::bind(&self.addr)
            .serve(router.into_make_service())
            .await
    }

    // Example handlers
    async fn root_handler() -> &'static str {
        "Hello from Axum Builder!"
    }

    async fn health_handler() -> &'static str {
        "OK"
    }
}

