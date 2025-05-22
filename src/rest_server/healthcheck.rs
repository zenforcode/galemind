use axum::response::IntoResponse;
use axum::{Router, routing::get};


async fn liveness_handler() -> impl IntoResponse {
    "OK"
}
async fn readiness_handler() -> impl IntoResponse {
    "OK"
}
pub fn new_health_check_router() -> Router {
    Router::new()
    .route("/live", get(liveness_handler))
    .route("/ready", get(readiness_handler))
}