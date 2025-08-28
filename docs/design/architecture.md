# Architecture Documentation

## System Architecture Overview

This document provides a detailed architectural overview of the adaptive batching library, including component interactions, data flows, and design decisions.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Client Applications                           │
└─────────────────────────┬───────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                        Public API Layer                                │
├─────────────────────────────────────────────────────────────────────────┤
│  BatchConfig  │  CorkDispatcher  │  Result<T>  │  BatchError           │
└─────────────────────────┬───────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                      Core Dispatcher Layer                             │
├─────────────────────────────────────────────────────────────────────────┤
│              Request Queue Management                                   │
│              Batch Formation Logic                                      │
│              Async Task Coordination                                    │
│              Training Phase Control                                     │
└─────────────────────────┬───────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                    Optimization Layer                                  │
├─────────────────────────────────────────────────────────────────────────┤
│  Parameter Learning  │  Least Squares  │  Constraint Validation        │
└─────────────────────────┬───────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                    Infrastructure Layer                                │
├─────────────────────────────────────────────────────────────────────────┤
│   Async Runtime   │   Channels   │   Metrics   │   Error Handling      │
│    (Tokio)        │ (Crossbeam)  │ Collection  │    (thiserror)        │
└─────────────────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. Public API Layer

#### BatchConfig
```rust
pub struct BatchConfig {
    max_batch_size: usize,
    max_latency_ms: u64,
    enable_metrics: bool,
    // Internal optimization parameters
}
```
**Purpose**: Configuration management and validation
**Key Features**:
- Builder pattern for ergonomic configuration
- Validation of parameter constraints
- Default values based on best practices

#### CorkDispatcher
```rust
pub struct CorkDispatcher<T, F, R> 
where 
    T: Batchable + Send + Sync + 'static,
    F: Fn(Vec<T>) -> Future<Output = Result<Vec<R>, BatchError>>,
{
    // Internal state
}
```
**Purpose**: Main entry point for batching operations
**Key Features**:
- Generic over request/response types
- Async processing function integration
- Lifecycle management (start/shutdown)

### 2. Core Dispatcher Implementation

#### Request Flow Architecture
```
┌─────────────┐    ┌──────────────┐    ┌─────────────────┐
│   Client    │───▶│  Dispatcher  │───▶│ Request Queue   │
│  Request    │    │   dispatch() │    │                 │
└─────────────┘    └──────────────┘    └─────────────────┘
                          │                      │
                          ▼                      ▼
                   ┌──────────────┐    ┌─────────────────┐
                   │   oneshot    │    │ Batch Formation │
                   │  Response    │    │     Logic       │
                   │   Channel    │    └─────────────────┘
                   └──────────────┘             │
                          ▲                     ▼
                          │            ┌─────────────────┐
                          └────────────│   Processor     │
                                       │   Function      │
                                       └─────────────────┘
```

#### Controller Loop State Machine
```rust
enum DispatcherState {
    Training {
        samples_collected: usize,
        training_data: Vec<(f64, f64)>,
    },
    Optimized {
        current_params: OptimizationParams,
        last_update: Instant,
    },
    Shutdown,
}
```

### 3. Optimization Layer Architecture

#### Data Collection Pipeline
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ Batch Execution │───▶│ Metrics Capture │───▶│ Training Data   │
│                 │    │                 │    │   Storage       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                                       │
                                                       ▼
                               ┌─────────────────────────────────────┐
                               │     Least Squares Optimization     │
                               │                                     │
                               │  Minimize: Σ(predicted - actual)²  │
                               │                                     │
                               │  Subject to: parameter constraints  │
                               └─────────────────┬───────────────────┘
                                                 │
                                                 ▼
                               ┌─────────────────────────────────────┐
                               │      Parameter Update & Apply       │
                               └─────────────────────────────────────┘
```

#### Optimization Mathematics
The optimizer solves the linear system:
```
[ n      Σx_i    ] [ o_b ]   [ Σy_i      ]
[ Σx_i   Σx_i²   ] [ o_a ] = [ Σ(x_i×y_i) ]
```

Where:
- `n` = number of observations
- `x_i` = batch size for observation i
- `y_i` = processing time for observation i

### 4. Infrastructure Layer

#### Async Runtime Integration
```rust
// Channel types used throughout the system
type RequestSender<T> = mpsc::UnboundedSender<RequestMessage<T>>;
type RequestReceiver<T> = mpsc::UnboundedReceiver<RequestMessage<T>>;
type ResponseSender<R> = oneshot::Sender<Result<R, BatchError>>;

// Message envelope for internal communication
struct RequestMessage<T> {
    request: T,
    response_tx: ResponseSender<T::Response>,
    received_at: Instant,
}
```

#### Error Handling Strategy
```rust
#[derive(thiserror::Error, Debug)]
pub enum BatchError {
    #[error("Request timed out after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
    
    #[error("Service unavailable: {reason}")]
    ServiceUnavailable { reason: String },
    
    #[error("Processing failed: {source}")]
    ProcessingError { source: Box<dyn std::error::Error + Send + Sync> },
    
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
}
```

## Data Flow Patterns

### 1. Request Processing Flow
```
Request → Queue → Batch Formation → Processing → Response Delivery
   ↓         ↓           ↓             ↓            ↓
Validate  Timeout    Size/Time     Function     Error/Success
         Check      Threshold      Execute       Handling
```

### 2. Training Data Flow
```
Batch Processing → Timing Measurement → Data Collection → Analysis
        ↓                   ↓                ↓            ↓
   Execute Batch      Record Metrics    Store Sample   Regression
                                                           ↓
Parameter Update ← Validation ← Calculation ← Sufficient Data?
```

### 3. Metrics Collection Flow
```
Operation Events → Measurement → Aggregation → Reporting
       ↓              ↓            ↓           ↓
   Timestamps    Calculate      Update      External
   Batch Sizes   Latencies     Histograms   Monitoring
```

## Design Patterns and Principles

### 1. Async/Await Patterns
- **Single Threaded Async**: All operations use Tokio's single-threaded async model
- **Channel Communication**: No shared mutable state, only message passing
- **Graceful Cancellation**: Proper cleanup on shutdown signals

### 2. Type Safety
- **Generic Programming**: Flexible types while maintaining compile-time safety
- **Trait Bounds**: Clear contracts for batchable types
- **Error Types**: Structured error handling with context

### 3. Resource Management
- **RAII**: Automatic resource cleanup through Drop traits
- **Arc/Mutex**: Minimal shared state with clear ownership
- **Channel Cleanup**: Proper channel closure and cleanup

## Performance Characteristics

### 1. Memory Usage
- **Request Buffering**: Bounded by max_batch_size configuration
- **Metrics Storage**: Rolling window approach for bounded memory
- **Channel Buffers**: Unbounded channels for request ingestion

### 2. CPU Utilization
- **Async Efficiency**: Single-threaded async reduces context switching
- **Batch Processing**: Amortizes fixed costs across multiple requests
- **Optimization Overhead**: Minimal computational cost for parameter updates

### 3. Latency Profile
- **Request Queuing**: Variable based on arrival patterns
- **Batch Formation**: Bounded by wait_time parameter
- **Processing Time**: Optimized through parameter learning

## Scalability Considerations

### 1. Vertical Scaling
- **Memory Bounds**: Configurable limits prevent unbounded growth
- **CPU Efficiency**: Async design maximizes single-core utilization
- **Throughput**: Scales with batch processing efficiency

### 2. Horizontal Scaling
- **Stateless Design**: Each dispatcher instance is independent
- **Load Distribution**: External load balancing required
- **Metrics Aggregation**: Per-instance metrics can be combined

### 3. Resource Limits
- **Back Pressure**: Natural back pressure through channel capacity
- **Circuit Breaker**: Error handling prevents cascade failures
- **Graceful Degradation**: Maintains service under stress

## Security Considerations

### 1. Memory Safety
- **Rust Guarantees**: No buffer overflows or use-after-free
- **Type Safety**: Compile-time prevention of common errors
- **Resource Leaks**: Automatic cleanup prevents resource exhaustion

### 2. Error Information
- **Controlled Disclosure**: Error messages don't leak sensitive data
- **Logging Safety**: Metrics don't expose request contents
- **Timeout Protection**: Prevents resource holding attacks

### 3. Input Validation
- **Configuration**: All parameters validated at startup
- **Request Validation**: User-provided validation through Batchable trait
- **Bounds Checking**: All array/vector accesses are bounds-checked

## Testing Strategy

### 1. Unit Testing
- **Component Isolation**: Each module tested independently
- **Mock Dependencies**: Async functions mocked for deterministic testing
- **Property Testing**: Invariant validation across parameter ranges

### 2. Integration Testing
- **End-to-End Flows**: Full request processing validation
- **Concurrency Testing**: Multi-threaded access patterns
- **Performance Testing**: Latency and throughput validation

### 3. Chaos Testing
- **Failure Injection**: Network timeouts and processing errors
- **Resource Exhaustion**: Memory and CPU stress testing
- **Recovery Testing**: Graceful recovery from failure states

This architecture provides a robust foundation for adaptive batching while maintaining the flexibility to evolve with changing requirements and workload patterns.
