use super::metadata_model::ServerMetadataResponse;
use crate::metadata_model::ErrorServerMetadataResponse;
use axum::{Json, Router, extract::Path, routing::get};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

async fn server_metadata(
    Path(_): Path<HashMap<String, String>>,
) -> Result<Json<ServerMetadataResponse>, Json<ErrorServerMetadataResponse>> {
    let now = SystemTime::now();

    if now.duration_since(UNIX_EPOCH).unwrap().as_secs() % 2 == 0 {
        Ok(Json(ServerMetadataResponse {
            name: "test".to_string(),
            version: "v2".to_string(),
            extensions: vec!["ext1".to_string()],
        }))
    } else {
        Err(Json(ErrorServerMetadataResponse {
            error: "woomp woomp".to_string(),
        }))
    }
}

pub fn new_server_router() -> Router {
    Router::new().route("/", get(server_metadata))
}
