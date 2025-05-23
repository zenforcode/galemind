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
    grpc_server::{HealthCheckService, HealthCheckServer},
    ServerLiveRequest, ServerLiveResponse,
    ServerReadyRequest, ServerReadyResponse,
};

#[derive(Debug, Default)]
pub struct HealthCheckServiceImpl;

#[tonic::async_trait]
impl HealthCheckService for HealthCheckServiceImpl {
    async fn server_live(
        &self,
        request: Request<ServerLiveRequest>,
    ) -> Result<Response<ServerLiveResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = ServerLiveResponse {
            message: "Server is live".into(),
        };

        Ok(Response::new(reply))
    }

    async fn server_ready(
        &self,
        request: Request<ServerReadyRequest>,
    ) -> Result<Response<ServerReadyResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = ServerReadyResponse {
            message: "Server is ready".into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let health_check_service = HealthCheckServiceImpl::default();

    println!("HealthCheck gRPC server listening on {}", addr);

    Server::builder()
        .add_service(HealthCheckServer::new(health_check_service))
        .serve(addr)
        .await?;

    Ok(())
}