use axum::{response::IntoResponse, routing::get, extract::Path, Router};

async fn model_ready_handler(Path(model_name): Path<String>) -> impl IntoResponse {
    format!("Model: {}, Ready!", model_name)
}

async fn model_version_ready_handler(
    Path((model_name, model_version)): Path<(String, String)>
) -> impl IntoResponse {
    format!("Model: {}, Version: {}, Ready!", model_name, model_version)
}

pub fn new_model_router() -> Router {
    Router::new()
        .route(
            "/{model_name}/ready",
            get(model_ready_handler),
        )
        .route(
            "/models/{model_name}/versions/{model_version}/ready",
            get(model_version_ready_handler),
        )
}

