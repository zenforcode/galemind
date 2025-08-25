#[derive(Debug, Clone)]
pub enum Payload {
    Text(String),
    Json(serde_json::Value),
    Binary(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct ModelRequest {
    pub payload: Payload,
}

impl ModelRequest {
    pub fn new_text(text: impl Into<String>) -> Self {
        Self {
            payload: Payload::Text(text.into()),
        }
    }

    pub fn new_json(json: serde_json::Value) -> Self {
        Self {
            payload: Payload::Json(json),
        }
    }

    pub fn new_binary(data: Vec<u8>) -> Self {
        Self {
            payload: Payload::Binary(data),
        }
    }
}
