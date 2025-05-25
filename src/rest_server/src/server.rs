use super::data_server::ServerMetadataResponse;
use axum::{Json, Router, extract::Path, routing::get};
use std::collections::HashMap;

async fn server_metadata(Path(_): Path<HashMap<String, String>>) -> Json<ServerMetadataResponse> {
    Json(ServerMetadataResponse {
        name: "test".to_string(),
        version: "v2".to_string(),
        extensions: vec!["ext1".to_string(), "ext2".to_string()],
    })
}

pub fn new_server_router() -> Router {
    Router::new().route("/", get(server_metadata))
}
