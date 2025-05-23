
use tonic::{transport::Server, Request, Response, Status};

// Include the generated protobuf code
pub mod grpc_server {
    tonic::include_proto!("grpc_server"); // This macro includes the generated code.
}

use grpc_server::{
    prediction_service_server::{PredictionService, PredictionServiceServer},
    ServerLiveRequest, ServerLiveResponse,
    ServerReadyRequest, ServerReadyResponse,
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

        let reply = ServerLiveResponse {
            live: true,
        };

        Ok(Response::new(reply))
    }

    async fn server_ready(
        &self,
        request: Request<ServerReadyRequest>,
    ) -> Result<Response<ServerReadyResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = ServerReadyResponse {
            ready: true,
        };

        Ok(Response::new(reply))
    }
}

/// Builder for setting up the gRPC server
pub struct GrpcServerBuilder {
    address: String,
    service_impl: PredictionServiceImpl,
}

impl GrpcServerBuilder {
    pub fn new() -> Self {
        Self {
            address: "[::1]:50051".to_string(),
            service_impl: PredictionServiceImpl::default(),
        }
    }

    pub fn with_address(mut self, address: &str) -> Self {
        self.address = address.to_string();
        self
    }

    pub fn with_service(mut self, service: PredictionServiceImpl) -> Self {
        self.service_impl = service;
        self
    }

    pub async fn build(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.address.parse()?;

        println!("GRPC PredictionService server listening on {}", addr);

        Server::builder()
            .add_service(PredictionServiceServer::new(self.service_impl))
            .serve(addr)
            .await?;

        Ok(())
    }
}
