use axum::{
    Router,
    extract::{Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
//  TODO: later change this to galemind::api
use crate::rest_server::datamodel::InferenceRequest;

async fn model_ready_handler(Path(model_name): Path<String>) -> impl IntoResponse {
    format!("Model: {}, Ready!", model_name)
}

async fn model_version_ready_handler(
    Path((model_name, model_version)): Path<(String, String)>,
) -> impl IntoResponse {
    format!("Model: {}, Version: {}, Ready!", model_name, model_version)
}

async fn model_infer_handler(
    Path(_model_name): Path<String>,
    Json(_payload): Json<InferenceRequest>,
) -> Response {
    (StatusCode::NOT_IMPLEMENTED, "Not Yet Implemented").into_response()
}

async fn model_version_infer_handler(
    Path((_model_name, _model_version)): Path<(String, String)>,
    Json(_payload): Json<InferenceRequest>,
) -> Response {
    (StatusCode::NOT_IMPLEMENTED, "Not Yet Implemented").into_response()
}

pub fn new_model_router() -> Router {
    Router::new()
        .route("/models/{model_name}/ready", get(model_ready_handler))
        .route(
            "/models/{model_name}/versions/{model_version}/ready",
            get(model_version_ready_handler),
        )
        .route("/models/{model_name}/infer", post(model_infer_handler))
        .route(
            "/models/{model_name}/versions/{model_version}/infer",
            post(model_version_infer_handler),
        )
}
