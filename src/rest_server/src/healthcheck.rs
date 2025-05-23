use axum::{Router, extract::Path, response::IntoResponse, routing::get};
use std::collections::HashMap;

async fn liveness_handler(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    println!("current API version is: {}", params.get("version").unwrap());
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
