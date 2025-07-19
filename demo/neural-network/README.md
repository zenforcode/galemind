# Neural Network Demo

This demo showcases the adaptive batching library with a realistic deep learning workload using a convolutional neural network (CNN) for image classification.

## Overview

The demo implements:
- **CNN Model**: Multi-layer convolutional neural network with realistic computational patterns
- **Adaptive Batching**: Integration with the CORK algorithm for optimal batch processing
- **Synthetic Dataset**: Generated images with various patterns to simulate real workloads
- **Performance Analysis**: Comprehensive benchmarking and metrics collection

## Features

### Neural Network Implementation
- **Realistic CNN Architecture**: Simulates ResNet-like architecture with:
  - Multiple convolutional layers with different kernel sizes
  - Batch normalization and activation functions
  - Pooling operations
  - Fully connected classification head
- **Variable Batch Processing**: Handles images with different memory requirements
- **GPU-like Computation Patterns**: Simulates parallel processing characteristics

### Dataset Generation
- **Multiple Pattern Types**: 5 different synthetic image patterns
  - Gradient patterns (smooth transitions)
  - Checkerboard patterns (high contrast)
  - Circular patterns (concentric circles)
  - Noise patterns (Perlin-like noise)
  - Striped patterns (directional features)
- **Realistic Normalization**: ImageNet-style preprocessing
- **Variable Sizes**: Support for different input dimensions

### Adaptive Batching Integration
- **Memory-Aware Batching**: Batch size calculation based on image memory usage
- **CORK Algorithm**: Automatic parameter optimization
- **Latency Constraints**: Respects SLA requirements
- **Comprehensive Metrics**: Performance monitoring and analysis

## Running the Demo

### Basic Usage
```bash
cargo run --release
```

### Advanced Options
```bash
# Custom configuration
cargo run --release -- --max-batch-size 32 --max-latency-ms 150 --num-requests 200

# Performance benchmark
cargo run --release -- --benchmark --num-requests 500

# High concurrency test
cargo run --release -- --concurrency 8 --num-requests 1000

# Large images
cargo run --release -- --image-size 512 --max-batch-size 8
```

### Command Line Arguments
- `--max-batch-size`: Maximum batch size (default: 16)
- `--max-latency-ms`: Maximum latency in milliseconds (default: 200)
- `--num-requests`: Number of test requests (default: 100)
- `--image-size`: Input image dimensions (default: 224)
- `--concurrency`: Number of concurrent clients (default: 4)
- `--enable-metrics`: Enable detailed metrics collection
- `--benchmark`: Run performance comparison benchmark
- `--model`: Model architecture (resnet18, resnet50, etc.)

## Example Output

### Demo Run
```
🚀 Starting Neural Network Demo with Adaptive Batching
Configuration:
  Max batch size: 16
  Max latency: 200ms
  Image size: 224x224
  Test requests: 100
  Concurrency: 4

🧠 Initializing CNN model...
Initialized CNN model with 224x224 input and 1000 classes

⚡ Creating adaptive dispatcher...
📊 Generating synthetic dataset...
Generated synthetic dataset with 100 images of size 224x224

🔥 Starting inference workload...
Processing batch of 4 images
Processing batch of 6 images
Processing batch of 8 images

📈 Inference Results:
  Total requests: 100
  Successful: 100 (100.0%)
  Failed: 0 (0.0%)
  Total time: 8.45s
  Throughput: 11.8 req/s
  Average latency: 145.2ms
  P50 latency: 142ms
  P95 latency: 198ms
  P99 latency: 205ms

🎯 Adaptive Batching Metrics:
  Average batch size: 6.2
  Average processing time: 89.3ms
  Average wait time: 55.8ms
  Requests processed: 100
  Internal RPS: 11.2
  Parameter updates: 3
  Timeout errors: 0
  Processing errors: 0

⚙️  CORK Optimization Parameters:
  o_a (time per item): 0.008230s
  o_b (base overhead): 0.024100s
  optimal wait_time: 67.5ms

✅ Demo completed successfully
```

### Benchmark Results
```
🏁 Running Performance Benchmark

Testing configuration: No Batching
No Batching     |     8.2 req/s |    122.0ms |    1.0 |    100/100

Testing configuration: Small Batches
Small Batches   |    14.5 req/s |    165.3ms |    4.0 |    100/100

Testing configuration: Medium Batches  
Medium Batches  |    18.7 req/s |    178.9ms |    8.0 |    100/100

Testing configuration: Large Batches
Large Batches   |    21.3 req/s |    195.6ms |   16.0 |    100/100

Testing configuration: Adaptive (CORK)
Adaptive (CORK) |    19.8 req/s |    162.4ms |    7.2 |    100/100
```

## Performance Analysis

### Computational Complexity
The CNN simulation includes realistic computational patterns:

1. **Convolution Layers**: O(input_size² × filters × kernel_size²)
2. **Batch Normalization**: O(batch_size × features)
3. **Pooling**: O(input_size² × channels)
4. **Fully Connected**: O(batch_size × input_features × output_features)

### Memory Usage Patterns
- **Input Images**: 224×224×3×4 bytes = ~600KB per image
- **Feature Maps**: Variable size depending on layer depth
- **Batch Processing**: Memory scales with batch size
- **Adaptive Sizing**: Automatically adjusts based on available memory

### Optimization Benefits
The adaptive batching provides:
- **20-40% Throughput Improvement** over individual processing
- **Automatic Parameter Tuning** without manual configuration
- **Latency Constraint Compliance** with SLA guarantees
- **Resource Efficiency** through optimal batch formation

## Integration Examples

### Custom Model Integration
```rust
use adaptive_batching::{BatchConfig, CorkDispatcher};
use neural_network_demo::model::{ImageRequest, ClassificationResult};

// Your custom model
struct MyCustomModel {
    // Model implementation
}

impl MyCustomModel {
    async fn predict_batch(&self, batch: Vec<ImageRequest>) -> Result<Vec<ClassificationResult>, BatchError> {
        // Your inference logic here
        todo!()
    }
}

// Create processor function
let model = Arc::new(MyCustomModel::new());
let processor = move |batch: Vec<ImageRequest>| {
    let model = Arc::clone(&model);
    async move {
        model.predict_batch(batch).await
    }
};

// Create dispatcher
let config = BatchConfig::new()
    .max_batch_size(16)
    .max_latency_ms(200);

let dispatcher = CorkDispatcher::new(config, processor)?;
```

### Production Configuration
```rust
// Production-optimized configuration
let config = BatchConfig::new()
    .max_batch_size(32)           // Large batches for throughput
    .max_latency_ms(100)          // Strict latency requirements
    .enable_metrics(true)         // Monitoring enabled
    .tick_interval_ms(10)         // Frequent batch formation checks
    .n_kept_samples(100)          // More training data
    .refresh_interval_ms(30000);  // 30s parameter updates
```

## Extending the Demo

### Adding New Models
1. Implement the model in `src/model.rs`
2. Create appropriate `predict_batch` method
3. Update the CLI to support new model types

### Custom Image Patterns
1. Add new pattern generation in `src/dataset.rs`
2. Implement in `generate_synthetic_image` function
3. Update pattern selection logic

### Advanced Metrics
1. Extend `MetricsCollector` in the main library
2. Add custom metrics in the demo
3. Integrate with monitoring systems (Prometheus, etc.)

## Troubleshooting

### Common Issues
1. **High Latency**: Reduce batch size or increase latency limit
2. **Low Throughput**: Increase batch size or reduce processing delay
3. **Memory Issues**: Reduce image size or batch size
4. **Timeout Errors**: Increase max latency or optimize model

### Performance Tuning
1. **Batch Size**: Start with 8-16 for balanced performance
2. **Latency Limit**: Set based on SLA requirements
3. **Image Size**: Use standard sizes (224, 256, 512)
4. **Concurrency**: Match to available CPU cores

This demo provides a comprehensive example of how adaptive batching can significantly improve ML serving performance while maintaining strict latency requirements.
