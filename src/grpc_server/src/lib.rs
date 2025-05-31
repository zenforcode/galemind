use async_trait::async_trait;
use foundation::{InferenceServerBuilder, InferenceServerConfig};
use tonic::{Request, Response, Status, transport::Server};
// Include the generated protobuf code
pub mod grpc_server {
    tonic::include_proto!("grpc_server"); // This macro includes the generated code.
}

use grpc_server::{
    ServerLiveRequest, ServerLiveResponse, ServerReadyRequest, ServerReadyResponse,
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

        println!("GRPC PredictionService server listening on {}", addr);

        Server::builder()
            .add_service(PredictionServiceServer::new(self.service_impl))
            .serve(addr)
            .await?;
        Ok(())
    }
}
