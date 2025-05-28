use super::inference::{
    InferParameter, InferenceError, InferenceOutput, InferenceProcessor, InferenceRequest,
    InferenceResponse,
};
use super::tensor::{Data, DataType};
use std::collections::HashMap;

/// A dummy implementation of the `InferenceProcessor` trait used for testing or development.
pub struct FakeInferenceProcessor;

impl InferenceProcessor for FakeInferenceProcessor {
    fn process(&self, request: InferenceRequest) -> InferenceResponse {
        if request.parameters.is_none() {
            return InferenceResponse::Error(InferenceError {
                error: "Empty Parameters".to_string(),
            });
        }

        let output = InferenceOutput {
            name: "output_1".to_string(),
            shape: vec![1, 3], // Assuming `DataShape` is type alias for Vec<i64>
            datatype: DataType::VFLOAT,
            parameters: Some(HashMap::from([(
                "confidence".to_string(),
                InferParameter::Double(0.33),
            )])),
            data: Data::VFLOAT(vec![0.1, 0.5, 0.4]),
        };

        InferenceResponse::Ok(output)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_inference_processor_returns_expected_response() {
        let processor = FakeInferenceProcessor;

        let dummy_request = InferenceRequest {
            model_name: "test_model".to_string(),
            model_version: Some("v1".to_string()),
            id: "req_001".to_string(),
            parameters: Some(HashMap::from([
                ("temperature".to_string(), InferParameter::Double(0.7)),
                ("top_p".to_string(), InferParameter::Double(0.9)),
            ])),
            outputs: None,
        };

        let response = processor.process(dummy_request);

        match response {
            InferenceResponse::Ok(output) => {
                assert_eq!(output.name, "output_1");
                assert_eq!(output.shape, vec![1, 3]);
                match output.data {
                    Data::VFLOAT(values) => assert_eq!(values, vec![0.1, 0.5, 0.4]),
                }
            }
            _ => panic!("Expected InferenceResponse::Ok variant"),
        }
    }

    #[test]
    fn process_returns_error_when_parameters_are_none() {
        let processor = FakeInferenceProcessor;

        let request = InferenceRequest {
            model_name: "test_model".to_string(),
            model_version: Some("v1".to_string()),
            id: "req_002".to_string(),
            parameters: None,
            outputs: None,
        };

        let response = processor.process(request);

        match response {
            InferenceResponse::Error(err) => {
                assert_eq!(err.error, "Empty Parameters");
            }
            _ => panic!("Expected error due to missing parameters"),
        }
    }
}
