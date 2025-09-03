mod translator;

use async_trait::async_trait;
use foundation::api::inference::InferParameter;
use foundation::model::model_manager::{ModelId, ModelManager};
use foundation::{InferenceRequest, InferenceServerBuilder, InferenceServerConfig};
use futures::Stream;
use std::collections::HashMap;
use std::{sync::Arc};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, transport::Server};

// Include the generated protobuf code
pub mod grpc_server {
    tonic::include_proto!("grpc_server");
}

use grpc_server::{
    ModelInferRequest, ModelInferResponse, ModelMetadataRequest,
    ModelMetadataResponse, ModelReadyRequest, ModelReadyResponse, ServerLiveRequest,
    ServerLiveResponse, ServerMetadataRequest, ServerMetadataResponse, ServerReadyRequest,
    ServerReadyResponse,
    model_metadata_response::TensorMetadata,
    prediction_service_server::{PredictionService, PredictionServiceServer},
};

pub struct PredictionServiceImpl {
    model_manager: Arc<ModelManager>,
}

impl PredictionServiceImpl {
    pub fn new(model_manager: Arc<ModelManager>) -> Self {
        Self { model_manager }
    }
}

#[tonic::async_trait]
impl PredictionService for PredictionServiceImpl {
    type ModelInferAsyncStream =
        Pin<Box<dyn Stream<Item = Result<ModelInferResponse, Status>> + Send>>;

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

    async fn model_infer_async(
        &self,
        request: Request<tonic::Streaming<ModelInferRequest>>,
    ) -> Result<Response<Self::ModelInferAsyncStream>, Status> {
        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(4);

        let model_manager = self.model_manager.clone();

        tokio::spawn(async move {
            while let Some(message) = stream.message().await.transpose() {
                match message {
                    Ok(req) => {
                        let model_id = ModelId(req.id.clone());

                        let parameters = req.parameters
                            .into_iter()
                            .map(|(k, v)| (k, InferParameter::from(v)))
                            .collect::<HashMap<_, _>>();

                        let inference_request = InferenceRequest {
                            model_name: req.model_name.clone(),
                            model_version: Some(req.model_version.clone()),
                            id: req.id.clone(),
                            parameters: Some(parameters),
                            outputs: None,
                        };

                        model_manager.add_request(model_id, inference_request);

                        // ACK/dummy responses if needed
                        let response = ModelInferResponse {
                            model_name: req.model_name,
                            model_version: req.model_version,
                            id: req.id,
                            parameters: HashMap::new(),
                            outputs: vec![],
                            raw_output_contents: vec![],
                        };
                        if let Err(e) = tx.send(Ok(response)).await {
                            eprintln!("Error sending response: {:?}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading stream: {:?}", e);
                        break;
                    }
                }
            }
        });

        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::ModelInferAsyncStream
        ))
    }

    async fn model_infer(
        &self,
        request: Request<ModelInferRequest>,
    ) -> Result<Response<ModelInferResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();
        let model_id = ModelId(req.id.clone());

        let domain_params = req.parameters
            .into_iter()
            .map(|(k, v)| (k, InferParameter::from(v)))
            .collect::<HashMap<_, _>>();

        let inference_request = InferenceRequest {
            model_name: req.model_name.clone(),
            model_version: Some(req.model_version.clone()),
            id: req.id.clone(),
            parameters: Some(domain_params),
            outputs: None, // or map req.outputs if needed
        };

        // Enqueue into ModelManager
        self.model_manager.add_request(model_id, inference_request);
        

        let reply = ModelInferResponse {
            model_name: req.model_name,
            model_version: req.model_version,
            id: req.id,
            parameters: HashMap::new(),
            outputs: vec![], 
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
    fn configure(context: InferenceServerConfig, model_manager: Arc<ModelManager>) -> Self {
        let addr = format!("{}:{}", context.grpc_hostname, context.grpc_port);
        Self {
            address: addr,
            service_impl: PredictionServiceImpl::new(model_manager),
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
