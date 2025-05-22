use axum::response::IntoResponse;

pub async fn liveness_handler() -> impl IntoResponse {
    "OK"
}

pub async fn readiness_handler() -> impl IntoResponse {
    "OK"
}
