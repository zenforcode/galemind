# Demo: Adaptive Batching with Deep Neural Networks

This demo showcases the adaptive batching library with realistic ML workloads, including a deep neural network inference engine and a high-performance gRPC server.

## Demo Structure

```
demo/
├── neural-network/        # Deep learning inference engine
│   ├── Cargo.toml        # Dependencies for ML workload
│   ├── src/
│   │   ├── main.rs       # Neural network demo
│   │   ├── model.rs      # CNN model implementation
│   │   └── dataset.rs    # Dataset utilities
│   └── README.md         # Neural network demo guide
├── grpc-server/          # High-performance gRPC server
│   ├── Cargo.toml        # gRPC server dependencies
│   ├── build.rs          # Proto compilation
│   ├── src/
│   │   ├── main.rs       # gRPC server implementation
│   │   ├── service.rs    # ML inference service
│   │   └── client.rs     # Test client
│   └── README.md         # gRPC server guide
├── proto/                # Protocol buffer definitions
│   └── ml_service.proto  # ML inference service definition
└── README.md            # This file
```

## Quick Start

### 1. Neural Network Demo
```bash
cd demo/neural-network/
cargo run --release
```
Demonstrates adaptive batching with a CNN performing image classification.

### 2. gRPC Server Demo
```bash
# Terminal 1: Start server
cd demo/grpc-server/
cargo run --bin server --release

# Terminal 2: Run client
cargo run --bin client --release
```

## Features Demonstrated

### Neural Network Demo
- **Convolutional Neural Network**: Multi-layer CNN for image classification
- **Variable Batch Sizes**: Images of different dimensions requiring different memory
- **Realistic ML Workload**: GPU-like computation patterns with memory constraints
- **Performance Metrics**: Training time vs batch size analysis

### gRPC Server Demo
- **Production-Ready Server**: High-performance gRPC service
- **Adaptive Batching Integration**: Real-world ML serving with CORK algorithm
- **Concurrent Clients**: Multiple clients sending requests simultaneously
- **Monitoring & Metrics**: Comprehensive performance monitoring
- **Load Testing**: Stress testing with various load patterns

## Performance Benefits

The demos showcase how adaptive batching provides:

1. **Improved Throughput**: 2-4x improvement over individual processing
2. **Latency Management**: Respects SLA constraints while optimizing batches
3. **Resource Efficiency**: Better GPU/CPU utilization through intelligent batching
4. **Automatic Adaptation**: No manual tuning required for different workloads

## Running the Demos

### Prerequisites
```bash
# Ensure you have the adaptive batching library built
cd ../../
cargo build --release

# Install additional dependencies for demos
rustup component add rustfmt clippy
```

### Neural Network Demo
```bash
cd demo/neural-network/
cargo run --release -- --help
```

Options:
- `--batch-size`: Maximum batch size (default: 16)
- `--max-latency`: Maximum latency in ms (default: 200)
- `--num-requests`: Number of test requests (default: 100)
- `--image-size`: Input image dimensions (default: 224x224)

### gRPC Server Demo
```bash
# Start server (Terminal 1)
cd demo/grpc-server/
cargo run --bin server --release -- --port 50051

# Run load test client (Terminal 2)
cargo run --bin client --release -- --server http://[::1]:50051 --requests 1000 --concurrency 10
```

## Architecture Overview

### Neural Network Architecture
```
Input Images → CNN Layers → Adaptive Batching → Batch Processing → Results
     ↓              ↓              ↓                    ↓              ↓
 Variable Size → Feature Maps → CORK Algorithm → Optimized Batches → Classifications
```

### gRPC Server Architecture
```
gRPC Clients → Request Queue → Adaptive Dispatcher → ML Model → Response Stream
     ↓              ↓                ↓                  ↓            ↓
 Concurrent → Batch Formation → Parameter Learning → Inference → Results
```

## Performance Analysis

The demos include built-in benchmarking to demonstrate:

1. **Batch Size Optimization**: How CORK finds optimal batch sizes
2. **Latency vs Throughput**: Trade-off analysis with real workloads
3. **Memory Efficiency**: Memory usage patterns with different batch strategies
4. **Adaptation Speed**: How quickly the algorithm adapts to changing loads

## Extending the Demos

### Adding New Models
1. Implement the `MLModel` trait in `neural-network/src/model.rs`
2. Define your model architecture and forward pass
3. Configure appropriate batch size calculation logic

### Custom gRPC Services
1. Modify `proto/ml_service.proto` for your service definition
2. Update `grpc-server/src/service.rs` with your inference logic
3. Rebuild with `cargo build`

## Monitoring and Observability

Both demos include comprehensive monitoring:

- **Request Latencies**: P50, P95, P99 latency tracking
- **Batch Size Distribution**: Histogram of batch sizes over time
- **Throughput Metrics**: Requests per second and processing rates
- **Error Rates**: Timeout and processing error tracking
- **Resource Usage**: Memory and CPU utilization monitoring

## Production Considerations

The demos illustrate production-ready patterns:

- **Error Handling**: Graceful handling of model errors and timeouts
- **Resource Management**: Memory bounds and cleanup
- **Configuration**: Environment-based configuration management
- **Logging**: Structured logging with tracing integration
- **Health Checks**: Server health and readiness endpoints

## Next Steps

After running the demos:

1. **Experiment with Parameters**: Try different batch sizes and latency limits
2. **Load Testing**: Increase client concurrency and request rates
3. **Custom Models**: Implement your own ML models using the patterns shown
4. **Production Deployment**: Use the gRPC server as a template for your services

These demos provide a solid foundation for understanding how adaptive batching can dramatically improve ML serving performance in real-world scenarios.
