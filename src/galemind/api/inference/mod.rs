use std::collections::HashMap;
use crate::galemind::api::tensor::{self, Data};

pub enum InferParameter {
    Bool(bool),
    Int64(i64),
    Double(f64),
    String(String),
}

pub struct InferenceRequestInput {
    name: String,
    shape: tensor::DataShape,
    datatype: String,
    data: Data,
}

pub struct InferenceRequestOutput {
    name: String,
    parameters: HashMap<String, InferParameter>,
}

pub enum InferenceResponse {
    Ok(InferenceOutput),
    Error(InferenceError),
}

pub struct InferenceRequest {
    model_name: String,
    model_version: Option<String>,
    id: String,
    parameters: Option<HashMap<String, InferParameter>>,
    outputs: Option<Vec<InferenceOutput>>,
}

pub struct InferenceOutput {
    name: String,
    shape: tensor::DataShape,
    datatype: tensor::DataType,
    parameters: Option<HashMap<String, InferParameter>>,
    data: Data,
}

pub struct InferenceError {
    error: String,
}

pub trait InferenceProcessor {
    fn process(&self, _request: InferenceRequest) -> InferenceResponse;
}
