use async_trait::async_trait;
use foundation::{InferenceServerBuilder, InferenceServerConfig};
use std::collections::HashMap;
use tonic::{Request, Response, Status, transport::Server};

// Include the generated protobuf code
pub mod grpc_server {
    tonic::include_proto!("grpc_server");
}

use grpc_server::{
    InferTensorContents, ModelInferRequest, ModelInferResponse, ModelMetadataRequest,
    ModelMetadataResponse, ModelReadyRequest, ModelReadyResponse, ServerLiveRequest,
    ServerLiveResponse, ServerMetadataRequest, ServerMetadataResponse, ServerReadyRequest,
    ServerReadyResponse,
    model_infer_response::InferOutputTensor,
    model_metadata_response::TensorMetadata,
    prediction_service_server::{PredictionService, PredictionServiceServer},
};

#[derive(Debug, Default)]
pub struct PredictionServiceImpl;

#[tonic::async_trait]
impl PredictionService for PredictionServiceImpl {
    async fn server_live(
        &self,
        request: Request<ServerLiveRequest>,
    ) -> Result<Response<ServerLiveResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = ServerLiveResponse { live: true };

        Ok(Response::new(reply))
    }

    async fn server_ready(
        &self,
        request: Request<ServerReadyRequest>,
    ) -> Result<Response<ServerReadyResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = ServerReadyResponse { ready: true };

        Ok(Response::new(reply))
    }

    async fn model_ready(
        &self,
        request: Request<ModelReadyRequest>,
    ) -> Result<Response<ModelReadyResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = ModelReadyResponse { ready: true };

        Ok(Response::new(reply))
    }

    async fn server_metadata(
        &self,
        request: Request<ServerMetadataRequest>,
    ) -> Result<Response<ServerMetadataResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = ServerMetadataResponse {
            name: "server_metadata".to_string(),
            version: "v1.0.0".to_string(),
            extensions: vec!["extension1".to_string(), "extension2".to_string()],
        };

        Ok(Response::new(reply))
    }

    async fn model_metadata(
        &self,
        request: Request<ModelMetadataRequest>,
    ) -> Result<Response<ModelMetadataResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = ModelMetadataResponse {
            name: "model_metadata".to_string(),
            versions: vec!["v1.0.0".to_string(), "v2.0.0".to_string()],
            platform: "platform".to_string(),
            inputs: vec![
                TensorMetadata {
                    name: "tensor_metadata_input1".to_string(),
                    datatype: "float32".to_string(),
                    shape: vec![1, 224, 224, 3],
                },
                TensorMetadata {
                    name: "tensor_metadata_input2".to_string(),
                    datatype: "int64".to_string(),
                    shape: vec![1],
                },
            ],
            outputs: vec![
                TensorMetadata {
                    name: "tensor_metadata_output1".to_string(),
                    datatype: "float32".to_string(),
                    shape: vec![1, 1000],
                },
                TensorMetadata {
                    name: "tensor_metadata_output2".to_string(),
                    datatype: "int64".to_string(),
                    shape: vec![1],
                },
            ],
        };

        Ok(Response::new(reply))
    }

    async fn model_infer(
        &self,
        request: Request<ModelInferRequest>,
    ) -> Result<Response<ModelInferResponse>, Status> {
        println!("Got a request: {:?}", request);

        // Process the inference request and return a response.
        // For now, we return a dummy response.
        let reply = ModelInferResponse {
            model_name: "inference_model".to_string(),
            model_version: "v1.0.0".to_string(),
            id: "123".to_string(),
            parameters: HashMap::from([
                (
                    "param1".to_string(),
                    grpc_server::InferParameter {
                        parameter_choice: Some(
                            grpc_server::infer_parameter::ParameterChoice::StringParam(
                                "value1".to_string(),
                            ),
                        ),
                    },
                ),
                (
                    "param2".to_string(),
                    grpc_server::InferParameter {
                        parameter_choice: Some(
                            grpc_server::infer_parameter::ParameterChoice::Int64Param(42),
                        ),
                    },
                ),
            ]),
            outputs: vec![InferOutputTensor {
                name: "infer_tensor_output2".to_string(),
                datatype: "int64".to_string(),
                shape: vec![1],
                parameters: HashMap::from([
                    (
                        "param1".to_string(),
                        grpc_server::InferParameter {
                            parameter_choice: Some(
                                grpc_server::infer_parameter::ParameterChoice::StringParam(
                                    "value1".to_string(),
                                ),
                            ),
                        },
                    ),
                    (
                        "param2".to_string(),
                        grpc_server::InferParameter {
                            parameter_choice: Some(
                                grpc_server::infer_parameter::ParameterChoice::Int64Param(24),
                            ),
                        },
                    ),
                ]),
                contents: Some(InferTensorContents {
                    bool_contents: vec![true, false],
                    int_contents: vec![2],
                    int64_contents: vec![42],
                    uint_contents: vec![10],
                    uint64_contents: vec![100],
                    fp32_contents: vec![1.0],
                    fp64_contents: vec![100.0],
                    bytes_contents: vec![vec![7, 8, 9]],
                }),
            }],
            raw_output_contents: vec![],
        };

        Ok(Response::new(reply))
    }
}

/// Builder for setting up the gRPC server
pub struct GrpcServerBuilder {
    address: String,
    service_impl: PredictionServiceImpl,
}
/// async trait should applied also to the implementation.
#[async_trait]
impl InferenceServerBuilder for GrpcServerBuilder {
    fn configure(context: InferenceServerConfig) -> Self {
        let addr = format!("{}:{}", context.grpc_hostname, context.grpc_port);
        Self {
            address: addr,
            service_impl: PredictionServiceImpl::default(),
        }
    }
    async fn start(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = self.address.parse()?;

        println!("gRPC PredictionService server listening on {}", addr);

        Server::builder()
            .add_service(PredictionServiceServer::new(self.service_impl))
            .serve(addr)
            .await?;
        Ok(())
    }
}
