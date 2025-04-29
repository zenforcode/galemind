use crate::galemind::api::tensor;
use std::collections::HashMap;

pub struct InferenceRequest {
    id: Option<String>,
    parameters: Option<HashMap<String, InferParameter>>,
    inputs: Vec<InferenceRequestInput>,
    outputs: Option<Vec<InferenceRequestOutput>>
}

pub enum InferParameter {
    BOOL(bool),
    INT64(i64),
    DOUBLE(f64),
    STRING(String)
}

pub struct InferenceRequestInput {
    name: String,
    shape: tensor::DataShape,
    datatype: String,
    data: tensor::Data
}

pub struct InferenceRequestOutput {
    name: String,
    parameters: HashMap<String, InferParameter>
}

pub enum InferenceResponse {
    Ok(Inference),
    Error(InferenceError)
}

pub struct Inference {
    model_name: String,
    model_version: Option<String>,
    id: String,
    parameters: Option<HashMap<String, InferParameter>>,
    outputs: Option<Vec<InferenceOutput>>
}

pub struct InferenceOutput {
    name: String,
    shape: tensor::DataShape,
    datatype: String,
    parameters: Option<HashMap<String, InferParameter>>,
    data: tensor::Data
}

pub struct InferenceError {
    error: String
}

pub trait InferenceProcessor {
    fn process(&self, request: InferenceRequest) -> InferenceResponse;
}

