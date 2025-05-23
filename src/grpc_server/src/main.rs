use tonic::{transport::Server, Request, Response, Status};

// Include the generated protobuf code
// The `my_service` module name comes from the `package mypackage;` in your .proto
// and the `tonic-build`'s default behavior.
// If you named your package `my_service` in the .proto, it would be `crate::my_service`.
// It's usually good practice to use `super::` or `crate::` for generated modules.
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let prediction_service = PredictionServiceImpl::default();

    println!("GRPC PredictionService server listening on {}", addr);

    Server::builder()
        .add_service(PredictionServiceServer::new(prediction_service))
        .serve(addr)
        .await?;

    Ok(())
}