/// This module defines a fake inference server to experiment with and adjust
/// the API of the application during development.

use crate::galemind::api::inference::{
    InferenceOutput, InferenceProcessor, InferenceRequest, InferenceResponse,
    InferParameter,
};
use crate::galemind::api::tensor::{Data, DataType};

use std::collections::HashMap;

/// A dummy implementation of the `InferenceProcessor` trait used for testing or development.
///
/// This fake processor doesn't perform real inference. It returns a fixed dummy response.
pub struct FakeInferenceProcessor;

impl InferenceProcessor for FakeInferenceProcessor {
    fn process(&self, _request: InferenceRequest) -> InferenceResponse {
        // Dummy output data
        let output = InferenceOutput {
            name: "output_1".to_string(),
            shape: vec![1, 3], // Assuming shape is Vec<i64>
            datatype: DataType::VFLOAT, // Correctly referencing the DataType enum
            parameters: Some(HashMap::from([
                ("confidence".to_string(), InferParameter::Double(0.99)),
            ])),
            data: Data::VFLOAT(vec![0.1, 0.5, 0.4]), // Correct data enum usage
        };
        InferenceResponse::Ok(output)
    }
}
