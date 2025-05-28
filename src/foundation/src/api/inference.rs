use super::tensor::{Data, DataShape, DataType};
use std::collections::HashMap;
pub enum InferParameter {
    Bool(bool),
    Int64(i64),
    Double(f64),
    String(String),
}

pub enum InferenceResponse {
    Ok(InferenceOutput),
    Error(InferenceError),
}

pub struct InferenceRequest {
    pub model_name: String,
    pub model_version: Option<String>,
    pub id: String,
    pub parameters: Option<HashMap<String, InferParameter>>,
    pub outputs: Option<Vec<InferenceOutput>>,
}

pub struct InferenceOutput {
    pub name: String,
    pub shape: DataShape,
    pub datatype: DataType,
    pub parameters: Option<HashMap<String, InferParameter>>,
    pub data: Data,
}

pub struct InferenceError {
    pub error: String,
}

pub trait InferenceProcessor {
    fn process(&self, _request: InferenceRequest) -> InferenceResponse;
}
