/// This module is a fake inference server that will help to experiment and adjust the API of the
/// application.

use crate::galemind::api::inference::{
    InferenceProcessor, InferenceRequest, InferenceResponse
};

struct FakeInferenceProcessor;

impl InferenceProcessor for FakeInferenceProcessor {
    fn process(&self, request: InferenceRequest) -> InferenceResponse {
        todo!()
    }
}
