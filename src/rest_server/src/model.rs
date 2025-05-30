use std::collections::HashMap;

use axum::{
    Router,
    extract::{Json, Path},
    response::IntoResponse,
    routing::{get, post},
};

//  TODO: later change this to galemind::api
use crate::data_model::{
    ErrorMetadataModelResponse, InferenceRequest, InferenceResponse, MetadataModelResponse,
    MetadataTensor,
};

async fn model_ready_handler(Path(model_name): Path<String>) -> impl IntoResponse {
    format!("Model: {}, Ready!", model_name)
}

async fn model_version_ready_handler(
    Path((model_name, model_version)): Path<(String, String)>,
) -> impl IntoResponse {
    format!("Model: {}, Version: {}, Ready!", model_name, model_version)
}

async fn model_infer_handler(
    Path(_params): Path<HashMap<String, String>>,
    Json(_payload): Json<InferenceRequest>,
) -> Json<InferenceResponse> {
    Json(InferenceResponse {
        id: None,
        outputs: Some(vec![MetadataTensor {
            name: "my_tensor".to_string(),
            shape: vec![12, 21],
            datatype: "magic".to_string(),
            parameters: None,
            data: None,
        }]),
    })
}

async fn model_version_handler(
    Path(_): Path<HashMap<String, String>>,
) -> Result<Json<MetadataModelResponse>, Json<ErrorMetadataModelResponse>> {
    let tensor = MetadataTensor {
        name: "my_tensor".to_string(),
        shape: vec![12, 21],
        datatype: "magic".to_string(),
        parameters: None,
        data: None,
    };
    Ok(Json(MetadataModelResponse {
        name: "something".to_string(),
        versions: None,
        platform: vec!["some_platform".to_string()],
        inputs: vec![tensor.clone()],
        outputs: vec![tensor.clone()],
    }))
}

pub fn new_model_router() -> Router {
    Router::new()
        .route("/{model_name}/ready", get(model_ready_handler))
        .route("/{model_name}/infer", post(model_infer_handler))
        .route(
            "/{model_name}/versions/{model_version}",
            post(model_version_handler),
        )
        .route(
            "/{model_name}/versions/{model_version}/ready",
            get(model_version_ready_handler),
        )
        .route(
            "/{model_name}/versions/{model_version}/infer",
            post(model_infer_handler),
        )
}
