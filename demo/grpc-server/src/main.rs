//! gRPC Server Demo with Adaptive Batching
//! 
//! This demo implements a high-performance gRPC server for ML inference
//! that showcases the adaptive batching library in a production-like environment.

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use axum::{routing::get, Router};
use clap::Parser;
use tokio::signal;
use tonic::transport::Server;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, warn, error, Level};

mod service;
mod metrics_server;

use service::MLInferenceServiceImpl;
use metrics_server::create_metrics_router;

/// gRPC Server CLI Arguments
#[derive(Parser)]
#[command(name = "grpc-server")]
#[command(about = "High-performance gRPC ML inference server with adaptive batching")]
struct Args {
    /// gRPC server port
    #[arg(long, default_value = "50051")]
    port: u16,
    
    /// HTTP metrics server port
    #[arg(long, default_value = "9090")]
    metrics_port: u16,
    
    /// Maximum batch size for adaptive batching
    #[arg(long, default_value = "16")]
    max_batch_size: usize,
    
    /// Maximum latency in milliseconds
    #[arg(long, default_value = "200")]
    max_latency_ms: u64,
    
    /// Number of worker threads
    #[arg(long, default_value = "4")]
    workers: usize,
    
    /// Enable detailed logging
    #[arg(long)]
    debug: bool,
    
    /// Enable metrics collection
    #[arg(long, default_value = "true")]
    enable_metrics: bool,
    
    /// Model to load (resnet18, resnet50, etc.)
    #[arg(long, default_value = "resnet18")]
    model: String,
    
    /// Configuration file path
    #[arg(long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize tracing
    let log_level = if args.debug { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .init();

    info!("Starting gRPC ML Inference Server");
    info!("Configuration:");
    info!("  gRPC Port: {}", args.port);
    info!("  Metrics Port: {}", args.metrics_port);
    info!("  Max Batch Size: {}", args.max_batch_size);
    info!("  Max Latency: {}ms", args.max_latency_ms);
    info!("  Workers: {}", args.workers);
    info!("  Model: {}", args.model);
    
    // Create ML inference service
    let ml_service = MLInferenceServiceImpl::new(
        args.max_batch_size,
        args.max_latency_ms,
        args.enable_metrics,
        &args.model,
    ).await?;
    
    // gRPC server address
    let grpc_addr: SocketAddr = format!("[::1]:{}", args.port).parse()?;
    
    // Start metrics server
    let metrics_handle = if args.enable_metrics {
        let metrics_addr: SocketAddr = format!("0.0.0.0:{}", args.metrics_port).parse()?;
        info!("📊 Starting metrics server on http://{}", metrics_addr);
        
        let metrics_router = create_metrics_router();
        Some(tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(metrics_addr).await
                .expect("Failed to bind metrics server");
            axum::serve(listener, metrics_router).await
                .expect("Metrics server failed");
        }))
    } else {
        None
    };
    
    // Create gRPC service
    use service::ml_inference_service_server::MlInferenceServiceServer;
    let grpc_service = MlInferenceServiceServer::new(ml_service);
    
    info!("🎯 Starting gRPC server on {}", grpc_addr);
    
    // Start gRPC server with graceful shutdown
    let server_future = Server::builder()
        .add_service(grpc_service)
        .serve_with_shutdown(grpc_addr, shutdown_signal());
    
    // Run server and handle shutdown
    tokio::select! {
        result = server_future => {
            if let Err(e) = result {
                error!("gRPC server error: {}", e);
            }
        }
        _ = async {
            if let Some(handle) = metrics_handle {
                let _ = handle.await;
            }
        } => {
            info!("Metrics server shutdown");
        }
    }
    
    info!("✅ Server shutdown complete");
    Ok(())
}

/// Wait for shutdown signal (Ctrl+C or SIGTERM)
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal");
        },
        _ = terminate => {
            info!("Received SIGTERM signal");
        },
    }
    
    info!("Initiating graceful shutdown...");
}
