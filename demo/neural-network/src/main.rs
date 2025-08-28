//! Neural Network Demo using Adaptive Batching
//! 
//! This demo showcases a realistic deep learning workload with a convolutional
//! neural network performing image classification, integrated with the adaptive
//! batching library to optimize throughput while maintaining latency constraints.

use std::time::{Duration, Instant};
use std::sync::Arc;

use clap::Parser;
use tracing::{info, warn, error, Level};
use anyhow::Result;

use adaptive_batching::{BatchConfig, CorkDispatcher};

mod model;
mod dataset;

use model::{CNNModel, ImageRequest, ClassificationResult};
use dataset::{ImageDataset, generate_synthetic_dataset};

/// Neural Network Demo Arguments
#[derive(Parser)]
#[command(name = "neural-network")]
#[command(about = "Deep neural network demo with adaptive batching")]
struct Args {
    /// Maximum batch size for adaptive batching
    #[arg(long, default_value = "16")]
    max_batch_size: usize,
    
    /// Maximum latency in milliseconds
    #[arg(long, default_value = "200")]
    max_latency_ms: u64,
    
    /// Number of test requests to generate
    #[arg(long, default_value = "100")]
    num_requests: usize,
    
    /// Input image size (width and height)
    #[arg(long, default_value = "224")]
    image_size: u32,
    
    /// Number of concurrent clients
    #[arg(long, default_value = "4")]
    concurrency: usize,
    
    /// Enable metrics collection
    #[arg(long)]
    enable_metrics: bool,
    
    /// Run performance benchmark
    #[arg(long)]
    benchmark: bool,
    
    /// Model architecture to use
    #[arg(long, default_value = "resnet18")]
    model: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let args = Args::parse();
    
    info!("🚀 Starting Neural Network Demo with Adaptive Batching");
    info!("Configuration:");
    info!("  Max batch size: {}", args.max_batch_size);
    info!("  Max latency: {}ms", args.max_latency_ms);
    info!("  Image size: {}x{}", args.image_size, args.image_size);
    info!("  Test requests: {}", args.num_requests);
    info!("  Concurrency: {}", args.concurrency);
    
    if args.benchmark {
        run_benchmark(&args).await?;
    } else {
        run_demo(&args).await?;
    }
    
    Ok(())
}

/// Run the main demo
async fn run_demo(args: &Args) -> Result<()> {
    // Initialize the neural network model
    info!("Initializing CNN model...");
    let model = Arc::new(CNNModel::new(args.image_size, 1000)?); // 1000 classes (ImageNet-like)
    
    // Create adaptive batching configuration
    let config = BatchConfig::new()
        .max_batch_size(args.max_batch_size)
        .max_latency_ms(args.max_latency_ms)
        .enable_metrics(args.enable_metrics);
    
    // Create the processor function that uses the CNN model
    let model_clone = Arc::clone(&model);
    let processor = move |batch: Vec<ImageRequest>| {
        let model = Arc::clone(&model_clone);
        async move {
            model.predict_batch(batch).await
        }
    };
    
    // Create and start the adaptive dispatcher
    info!("⚡ Creating adaptive dispatcher...");
    let dispatcher = CorkDispatcher::new(config, processor)?;
    dispatcher.start().await?;
    
    // Generate synthetic dataset
    info!("📊 Generating synthetic dataset...");
    let dataset = generate_synthetic_dataset(args.num_requests, args.image_size)?;
    
    // Run inference workload
    info!("🔥 Starting inference workload...");
    let start_time = Instant::now();
    
    let results = run_concurrent_inference(
        &dispatcher,
        dataset,
        args.concurrency,
    ).await?;
    
    let total_time = start_time.elapsed();
    
    // Print results
    print_results(&results, total_time, &dispatcher).await;
    
    // Shutdown
    dispatcher.shutdown().await?;
    info!("✅ Demo completed successfully");
    
    Ok(())
}

/// Run performance benchmark comparing different strategies
async fn run_benchmark(args: &Args) -> Result<()> {
    info!("🏁 Running Performance Benchmark");
    
    let model = Arc::new(CNNModel::new(args.image_size, 1000)?);
    let dataset = generate_synthetic_dataset(args.num_requests, args.image_size)?;
    
    // Test different configurations
    let configs = vec![
        ("No Batching", 1, args.max_latency_ms),
        ("Small Batches", 4, args.max_latency_ms),
        ("Medium Batches", 8, args.max_latency_ms),
        ("Large Batches", 16, args.max_latency_ms),
        ("Adaptive (CORK)", args.max_batch_size, args.max_latency_ms),
    ];
    
    for (name, batch_size, latency_ms) in configs {
        info!("Testing configuration: {}", name);
        
        let config = BatchConfig::new()
            .max_batch_size(batch_size)
            .max_latency_ms(latency_ms)
            .enable_metrics(true);
        
        let model_clone = Arc::clone(&model);
        let processor = move |batch: Vec<ImageRequest>| {
            let model = Arc::clone(&model_clone);
            async move {
                model.predict_batch(batch).await
            }
        };
        
        let dispatcher = CorkDispatcher::new(config, processor)?;
        dispatcher.start().await?;
        
        let start_time = Instant::now();
        let results = run_concurrent_inference(
            &dispatcher,
            dataset.clone(),
            args.concurrency,
        ).await?;
        let total_time = start_time.elapsed();
        
        print_benchmark_results(name, &results, total_time, &dispatcher).await;
        
        dispatcher.shutdown().await?;
        
        // Brief pause between tests
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    Ok(())
}

/// Run concurrent inference requests
async fn run_concurrent_inference(
    dispatcher: &CorkDispatcher<ImageRequest, ClassificationResult, _, _>,
    dataset: ImageDataset,
    concurrency: usize,
) -> Result<Vec<InferenceResult>> {
    let requests_per_client = dataset.images.len() / concurrency;
    let mut handles = Vec::new();
    
    for client_id in 0..concurrency {
        let start_idx = client_id * requests_per_client;
        let end_idx = if client_id == concurrency - 1 {
            dataset.images.len()
        } else {
            start_idx + requests_per_client
        };
        
        let client_requests: Vec<ImageRequest> = dataset.images[start_idx..end_idx]
            .iter()
            .enumerate()
            .map(|(i, img)| ImageRequest {
                id: (start_idx + i) as u64,
                image_data: img.clone(),
                width: dataset.width,
                height: dataset.height,
                channels: dataset.channels,
            })
            .collect();
        
        let dispatcher_ref = dispatcher;
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            
            for request in client_requests {
                let request_start = Instant::now();
                
                match dispatcher_ref.dispatch(request.clone()).await {
                    Ok(classification) => {
                        results.push(InferenceResult {
                            request_id: request.id,
                            success: true,
                            latency: request_start.elapsed(),
                            error_message: None,
                            predicted_class: Some(classification.class_id),
                            confidence: Some(classification.confidence),
                        });
                    }
                    Err(e) => {
                        results.push(InferenceResult {
                            request_id: request.id,
                            success: false,
                            latency: request_start.elapsed(),
                            error_message: Some(e.to_string()),
                            predicted_class: None,
                            confidence: None,
                        });
                    }
                }
                
                // Add some realistic delay between requests
                if client_id % 2 == 0 {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }
            
            results
        });
        
        handles.push(handle);
    }
    
    // Collect all results
    let mut all_results = Vec::new();
    for handle in handles {
        let client_results = handle.await?;
        all_results.extend(client_results);
    }
    
    Ok(all_results)
}

/// Structure to hold inference results
#[derive(Debug, Clone)]
struct InferenceResult {
    request_id: u64,
    success: bool,
    latency: Duration,
    error_message: Option<String>,
    predicted_class: Option<usize>,
    confidence: Option<f32>,
}

/// Print demo results
async fn print_results(
    results: &[InferenceResult],
    total_time: Duration,
    dispatcher: &CorkDispatcher<ImageRequest, ClassificationResult, _, _>,
) {
    let successful = results.iter().filter(|r| r.success).count();
    let failed = results.len() - successful;
    
    let latencies: Vec<Duration> = results.iter()
        .filter(|r| r.success)
        .map(|r| r.latency)
        .collect();
    
    if !latencies.is_empty() {
        let mut sorted_latencies = latencies.clone();
        sorted_latencies.sort();
        
        let p50_idx = sorted_latencies.len() / 2;
        let p95_idx = (sorted_latencies.len() as f64 * 0.95) as usize;
        let p99_idx = (sorted_latencies.len() as f64 * 0.99) as usize;
        
        let avg_latency = latencies.iter().sum::<Duration>() / latencies.len() as u32;
        let p50_latency = sorted_latencies.get(p50_idx).unwrap_or(&Duration::ZERO);
        let p95_latency = sorted_latencies.get(p95_idx).unwrap_or(&Duration::ZERO);
        let p99_latency = sorted_latencies.get(p99_idx).unwrap_or(&Duration::ZERO);
        
        info!("📈 Inference Results:");
        info!("  Total requests: {}", results.len());
        info!("  Successful: {} ({:.1}%)", successful, (successful as f64 / results.len() as f64) * 100.0);
        info!("  Failed: {} ({:.1}%)", failed, (failed as f64 / results.len() as f64) * 100.0);
        info!("  Total time: {:.2}s", total_time.as_secs_f64());
        info!("  Throughput: {:.1} req/s", results.len() as f64 / total_time.as_secs_f64());
        info!("  Average latency: {:.1}ms", avg_latency.as_millis());
        info!("  P50 latency: {:.1}ms", p50_latency.as_millis());
        info!("  P95 latency: {:.1}ms", p95_latency.as_millis());
        info!("  P99 latency: {:.1}ms", p99_latency.as_millis());
    }
    
    // Print adaptive batching metrics
    if let Some(metrics) = dispatcher.metrics() {
        info!("🎯 Adaptive Batching Metrics:");
        info!("  Average batch size: {:.1}", metrics.avg_batch_size);
        info!("  Average processing time: {:.1}ms", metrics.avg_processing_time_ms);
        info!("  Average wait time: {:.1}ms", metrics.avg_wait_time_ms);
        info!("  Requests processed: {}", metrics.requests_processed);
        if let Some(rps) = metrics.requests_per_second {
            info!("  Internal RPS: {:.1}", rps);
        }
        info!("  Parameter updates: {}", metrics.parameter_updates);
        info!("  Timeout errors: {}", metrics.timeout_errors);
        info!("  Processing errors: {}", metrics.processing_errors);
    }
    
    // Print optimization parameters
    let (o_a, o_b, wait_time) = dispatcher.optimization_params().await;
    info!("⚙️  CORK Optimization Parameters:");
    info!("  o_a (time per item): {:.6}s", o_a);
    info!("  o_b (base overhead): {:.6}s", o_b);
    info!("  optimal wait_time: {:.1}ms", wait_time * 1000.0);
}

/// Print benchmark comparison results
async fn print_benchmark_results(
    config_name: &str,
    results: &[InferenceResult],
    total_time: Duration,
    dispatcher: &CorkDispatcher<ImageRequest, ClassificationResult, _, _>,
) {
    let successful = results.iter().filter(|r| r.success).count();
    let throughput = results.len() as f64 / total_time.as_secs_f64();
    
    let avg_latency = if successful > 0 {
        let latencies: Vec<Duration> = results.iter()
            .filter(|r| r.success)
            .map(|r| r.latency)
            .collect();
        latencies.iter().sum::<Duration>() / latencies.len() as u32
    } else {
        Duration::ZERO
    };
    
    let avg_batch_size = dispatcher.metrics()
        .map(|m| m.avg_batch_size)
        .unwrap_or(1.0);
    
    info!("{:15} | {:>8.1} req/s | {:>8.1}ms | {:>6.1} | {:>6}/{}", 
        config_name,
        throughput,
        avg_latency.as_millis(),
        avg_batch_size,
        successful,
        results.len()
    );
}
