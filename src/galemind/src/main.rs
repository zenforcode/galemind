use clap::{Arg, Command};
use foundation::{model::model_manager::{self, ModelManager}, InferenceServerBuilder, InferenceServerConfig};
use grpc_server::GrpcServerBuilder;
use rest_server::RestServerBuilder;
use std::{env, error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("galemind")
        .version("0.1")
        .author("Zenforcode Team <team@zenforcode.com>")
        .about("GaleMind ML Inference Server v0.1")
        .subcommand(
            Command::new("start")
                .about("Start the server")
                .arg(
                    Arg::new("rest-host")
                        .long("rest-host")
                        .default_value("0.0.0.0")
                        .help("REST server host"),
                )
                .arg(
                    Arg::new("rest-port")
                        .long("rest-port")
                        .default_value("8080")
                        .help("REST server port"),
                )
                .arg(
                    Arg::new("grpc-host")
                        .long("grpc-host")
                        .default_value("0.0.0.0")
                        .help("gRPC server host"),
                )
                .arg(
                    Arg::new("grpc-port")
                        .long("grpc-port")
                        .default_value("50051")
                        .help("gRPC server port"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("start", sub_matches)) => {
            println!("Starting servers...");

            let context = InferenceServerConfig {
                rest_hostname: sub_matches
                    .get_one::<String>("rest-host")
                    .unwrap()
                    .to_string(),
                rest_port: sub_matches
                    .get_one::<String>("rest-port")
                    .unwrap()
                    .parse()?,
                grpc_hostname: sub_matches
                    .get_one::<String>("grpc-host")
                    .unwrap()
                    .to_string(),
                grpc_port: sub_matches
                    .get_one::<String>("grpc-port")
                    .unwrap()
                    .parse()?,
            };
            let grpc_context = context.clone();

            // Instantiate Model Manager with CircularBuffer capacity of 32 for each model ID
            // TODO: Calculate optimal value or pass dynamically models_buffer_capacity !
            let model_manager = Arc::new(ModelManager::new(32));
            model_manager.load_models_from_dir(env::var("MODELS_DIR").expect("MODELS_DIR environment variable must be set!"))?;

            // Load contexts for REST and gRPC servers
            let rest_server = RestServerBuilder::configure(context, model_manager.clone());
            let grpc_server = GrpcServerBuilder::configure(grpc_context, model_manager.clone());

            // Start REST and gRPC servers
            let rest_handler = tokio::spawn(async move { rest_server.start().await });
            let grpc_handler = tokio::spawn(async move { grpc_server.start().await });

            let (rest_result, grpc_result) = tokio::join!(rest_handler, grpc_handler);

            // Check REST server result
            match rest_result {
                Ok(Ok(())) => println!("REST server exited cleanly."),
                Ok(Err(e)) => eprintln!("REST server error: {}", e),
                Err(e) => eprintln!("REST task panicked: {}", e),
            }

            // Check gRPC server result
            match grpc_result {
                Ok(Ok(())) => println!("gRPC server exited cleanly."),
                Ok(Err(e)) => eprintln!("gRPC server error: {}", e),
                Err(e) => eprintln!("gRPC task panicked: {}", e),
            }

        }
        _ => {
            println!("Use --help for usage.");
        }
    }
    Ok(())
}
