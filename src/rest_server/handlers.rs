
async fn liveness_handler() -> impl IntoResponse {
    "OK"
}

async fn readyness_handler() -> impl IntoResponse {
    "OK"
}