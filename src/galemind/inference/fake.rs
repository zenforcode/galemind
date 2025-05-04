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
        if request.inputs.is_empty() {
            return InferenceResponse::Error("No input tensors provided".to_string());
        }
        if let Some(input) = request.inputs.get(0) {
            match &input.data {
                Data::VFLOAT(v) if v.is_empty() => {
                    return InferenceResponse::Error("Input tensor data is empty".to_string());
                }
                _ => {}
            }
        }
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::galemind::api::inference::{InferenceRequest, InferParameter};
    use crate::galemind::api::tensor::DataType;

    #[test]
    fn fake_inference_processor_returns_expected_response() {
        let processor = FakeInferenceProcessor;
        let dummy_request = InferenceRequest {
            // Populate with minimal valid data as per your actual struct definition
            inputs: vec![1.3,0.4,1.5],
            parameters: None,
        };

        let response = processor.process(dummy_request);

        match response {
            InferenceResponse::Ok(output) => {
                assert_eq!(output.name, "output_1");
                assert_eq!(output.shape, vec![1, 3]);
                assert_eq!(output.datatype, DataType::VFLOAT);
                assert_eq!(
                    output.parameters.unwrap().get("confidence"),
                    Some(&InferParameter::Double(0.99))
                );
                match output.data {
                    Data::VFLOAT(values) => assert_eq!(values, vec![0.1, 0.5, 0.4]),
                    _ => panic!("Unexpected data type"),
                }
            }
            _ => panic!("Expected InferenceResponse::Ok variant"),
        }
    }
    #[test]
    fn process_returns_error_when_no_inputs() {
        let processor = FakeInferenceProcessor;

        let request = InferenceRequest {
            inputs: vec![], // empty input list
            parameters: None,
        };

        let response = processor.process(request);

        match response {
            InferenceResponse::Error(msg) => {
                assert_eq!(msg, "No input tensors provided");
            }
            _ => panic!("Expected error due to empty inputs"),
        }
    }

    #[test]
    fn process_returns_error_when_input_data_is_empty() {
        let processor = FakeInferenceProcessor;

        let empty_tensor = InputTensor {
            name: "input_1".to_string(),
            shape: vec![1, 3],
            datatype: DataType::VFLOAT,
            data: Data::VFLOAT(vec![]), // empty data
        };

        let request = InferenceRequest {
            inputs: vec![empty_tensor],
            parameters: None,
        };

        let response = processor.process(request);

        match response {
            InferenceResponse::Error(msg) => {
                assert_eq!(msg, "Input tensor data is empty");
            }
            _ => panic!("Expected error due to empty tensor data"),
        }
    }
}
