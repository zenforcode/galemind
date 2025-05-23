use crate::api::tensor::{self, Data};
use std::collections::HashMap;

pub enum InferParameter {
    Bool(bool),
    Int64(i64),
    Double(f64),
    String(String),
}

pub struct InferenceRequestInput {
    pub name: String,
    pub shape: tensor::DataShape,
    pub datatype: String,
    pub data: Data,
}

pub struct InferenceRequestOutput {
    pub name: String,
    pub parameters: HashMap<String, InferParameter>,
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
    pub shape: tensor::DataShape,
    pub datatype: tensor::DataType,
    pub parameters: Option<HashMap<String, InferParameter>>,
    pub data: Data,
}

pub struct InferenceError {
    pub error: String,
}

pub trait InferenceProcessor {
    fn process(&self, _request: InferenceRequest) -> InferenceResponse;
}
