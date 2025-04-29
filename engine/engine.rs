use onnxruntime::{environment::Environment, session::Session, tensor::OrtOwnedTensor};
use std::error::Error;

pub mod engine {
    // Define a trait for model backends
    pub trait InferenceBackend {
        // Load a model from a file (e.g., ONNX, PyTorch, etc.)
        fn load_model(&mut self, model_path: &str) -> Result<(), String>;

        // Run inference and get the output
        fn run_inference(&self, input_data: Vec<f32>) -> Result<Vec<f32>, String>;

        // Optionally, get input/output shapes or other metadata
        fn get_input_shape(&self) -> Vec<i64>;
        fn get_output_shape(&self) -> Vec<i64>;
    }

    // Struct to represent an ONNX inference backend
    pub struct OnnxBackend {
        session: Option<Session>,
        environment: Option<Environment>,
    }

    impl OnnxBackend {
        // Constructor for OnnxBackend
        pub fn new() -> Self {
            OnnxBackend {
                session: None,
                environment: None,
            }
        }
    }

    // Implement the InferenceBackend trait for the ONNX backend
    impl InferenceBackend for OnnxBackend {
        // Load the model (this method needs to be mutable because it updates session and environment)
        fn load_model(&mut self, model_path: &str) -> Result<(), String> {
            // Create the ONNX runtime environment
            let environment = Environment::builder().build().map_err(|e| e.to_string())?;

            // Load the model into a session
            let session = environment
                .new_session_builder()
                .map_err(|e| e.to_string())?
                .with_model_from_file(model_path)
                .map_err(|e| e.to_string())?;

            // Store session and environment in the struct
            self.session = Some(session);
            self.environment = Some(environment);

            Ok(())
        }

        // Run inference and get output (no mutation, so it takes an immutable reference)
        fn run_inference(&self, input_data: Vec<f32>) -> Result<Vec<f32>, String> {
            if let Some(session) = &self.session {
                // Prepare the input tensor for inference
                let input_tensor_values = vec![input_data];

                // Run inference using the ONNX session
                let output: Vec<OrtOwnedTensor<f32, _>> = session
                    .run(input_tensor_values)
                    .map_err(|e| e.to_string())?;

                // Extract the first output tensor and convert it to a vector
                let output_tensor = &output[0];
                Ok(output_tensor.view().iter().cloned().collect())
            } else {
                Err("Model not loaded.".to_string())
            }
        }

        // Get the input shape from the loaded session
        fn get_input_shape(&self) -> Vec<i64> {
            if let Some(session) = &self.session {
                // Retrieve the input shape for the first input
                let input_info = session.input_info(0);
                input_info.shape().to_vec()
            } else {
                vec![]
            }
        }

        // Get the output shape from the loaded session
        fn get_output_shape(&self) -> Vec<i64> {
            if let Some(session) = &self.session {
                // Retrieve the output shape for the first output
                let output_info = session.output_info(0);
                output_info.shape().to_vec()
            } else {
                vec![]
            }
        }
    }
}