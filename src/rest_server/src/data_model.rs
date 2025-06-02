use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Parameters = HashMap<String, serde_json::Value>;

/// The InferenceRequest struct, top-level request object

#[derive(Serialize, Deserialize, Debug)]
pub struct InferenceRequest {
    /// Optional identifier for this request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Optional parameters as key/value pairs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,

    /// Required input tensors (at least one input required)
    pub inputs: Vec<MetadataTensor>,

    /// Optional requested outputs; if None, all model outputs are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<TensorRequestOutput>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InferenceResponse {
    /// Optional identifier for this request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Optional requested outputs; if None, all model outputs are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<MetadataTensor>>,
}

/// Represents an input tensor to the model
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetadataTensor {
    /// Name of the input tensor
    pub name: String,

    /// Shape of the input tensor (each dimension as u64)
    pub shape: Vec<u64>,

    /// Data type of the tensor elements (e.g. "FP32", "INT64")
    pub datatype: String,

    /// Optional parameters for this input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,

    /// Tensor data (see Tensor Data Types for more info)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TensorData>,
}

/// Represents requested output tensor(s)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TensorRequestOutput {
    /// Name of the output tensor
    pub name: String,

    /// Optional parameters for this output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
}

/// Enum to hold tensor data (extendable based on supported tensor types)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TensorData {
    // Numeric data as a flat array of numbers (integers or floats)
    Int32(Vec<i32>),
    Int64(Vec<i64>),
    Float32(Vec<f32>),
    Float64(Vec<f64>),
    Bool(Vec<bool>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataModelResponse {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
    pub platform: Vec<String>,
    pub inputs: Vec<MetadataTensor>,
    pub outputs: Vec<MetadataTensor>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMetadataModelResponse {
    pub error: String,
}
