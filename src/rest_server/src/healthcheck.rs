use axum::{Router, extract::Path, response::IntoResponse, routing::get};
use std::collections::HashMap;

async fn liveness_handler(Path(_): Path<HashMap<String, String>>) -> impl IntoResponse {
    "OK"
}
async fn readiness_handler(Path(_): Path<HashMap<String, String>>) -> impl IntoResponse {
    "OK"
}
pub fn new_health_check_router() -> Router {
    Router::new()
        .route("/live", get(liveness_handler))
        .route("/ready", get(readiness_handler))
}
