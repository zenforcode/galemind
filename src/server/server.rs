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
        let app = Self::create_app();

        axum::Server::bind(&self.addr)
            .serve(app.into_make_service())
            .await
    }

    fn create_app() -> Router {
        Router::new()
            .route("/", get(Self::root_handler))
            .layer(TraceLayer::new_for_http())
    }

    async fn root_handler() -> &'static str {
        "Hello from Axum Builder!"
    }
}