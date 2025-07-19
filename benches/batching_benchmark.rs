use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use adaptive_batching::{BatchConfig, CorkDispatcher};
use std::time::Duration;
use tokio::runtime::Runtime;

fn bench_basic_batching(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_batching");
    
    for batch_size in [1, 4, 8, 16, 32].iter() {
        group.bench_with_input(
            BenchmarkId::new("max_batch_size", batch_size),
            batch_size,
            |b, &batch_size| {
                let rt = Runtime::new().unwrap();
                
                b.to_async(&rt).iter(|| async {
                    let config = BatchConfig::new()
                        .max_batch_size(batch_size)
                        .max_latency_ms(100);
                    
                    let processor = |batch: Vec<i32>| async move {
                        let results: Vec<i32> = batch.into_iter().map(|x| x * 2).collect();
                        Ok(results)
                    };
                    
                    let dispatcher = CorkDispatcher::new(config, processor).unwrap();
                    dispatcher.start().await.unwrap();
                    
                    // Benchmark single request processing
                    let result = dispatcher.dispatch(black_box(42)).await.unwrap();
                    
                    dispatcher.shutdown().await.unwrap();
                    
                    black_box(result);
                });
            },
        );
    }
    
    group.finish();
}

fn bench_concurrent_requests(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_requests");
    
    for num_requests in [10, 50, 100, 200].iter() {
        group.bench_with_input(
            BenchmarkId::new("num_requests", num_requests),
            num_requests,
            |b, &num_requests| {
                let rt = Runtime::new().unwrap();
                
                b.to_async(&rt).iter(|| async {
                    let config = BatchConfig::new()
                        .max_batch_size(16)
                        .max_latency_ms(50);
                    
                    let processor = |batch: Vec<i32>| async move {
                        // Simulate some processing
                        tokio::time::sleep(Duration::from_micros(100)).await;
                        let results: Vec<i32> = batch.into_iter().map(|x| x + 1).collect();
                        Ok(results)
                    };
                    
                    let dispatcher = CorkDispatcher::new(config, processor).unwrap();
                    dispatcher.start().await.unwrap();
                    
                    // Send concurrent requests
                    let mut handles = Vec::new();
                    for i in 0..num_requests {
                        let disp = &dispatcher;
                        let handle = tokio::spawn(async move {
                            disp.dispatch(black_box(i)).await
                        });
                        handles.push(handle);
                    }
                    
                    // Wait for all to complete
                    for handle in handles {
                        let _ = handle.await.unwrap().unwrap();
                    }
                    
                    dispatcher.shutdown().await.unwrap();
                });
            },
        );
    }
    
    group.finish();
}

fn bench_optimization_overhead(c: &mut Criterion) {
    c.bench_function("optimization_overhead", |b| {
        let rt = Runtime::new().unwrap();
        
        b.to_async(&rt).iter(|| async {
            let config = BatchConfig::new()
                .max_batch_size(8)
                .max_latency_ms(100)
                .n_kept_samples(20)
                .enable_metrics(true);
            
            let processor = |batch: Vec<i32>| async move {
                // Variable processing time to trigger optimization
                let processing_time = Duration::from_micros(50 + batch.len() as u64 * 10);
                tokio::time::sleep(processing_time).await;
                let results: Vec<i32> = batch.into_iter().map(|x| x).collect();
                Ok(results)
            };
            
            let dispatcher = CorkDispatcher::new(config, processor).unwrap();
            dispatcher.start().await.unwrap();
            
            // Send requests to trigger optimization learning
            for i in 0..50 {
                let _ = dispatcher.dispatch(black_box(i)).await.unwrap();
            }
            
            dispatcher.shutdown().await.unwrap();
        });
    });
}

fn bench_large_batches(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_batches");
    
    for data_size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("data_size", data_size),
            data_size,
            |b, &data_size| {
                let rt = Runtime::new().unwrap();
                
                b.to_async(&rt).iter(|| async {
                    #[derive(Debug, Clone)]
                    struct LargeRequest {
                        data: Vec<f32>,
                    }
                    
                    impl adaptive_batching::container::Batchable for LargeRequest {
                        fn batch_size(&self) -> usize {
                            self.data.len()
                        }
                    }
                    
                    let config = BatchConfig::new()
                        .max_batch_size(50000) // Allow large batches based on data size
                        .max_latency_ms(200);
                    
                    let processor = |batch: Vec<LargeRequest>| async move {
                        // Simulate processing proportional to data size
                        let total_elements: usize = batch.iter().map(|r| r.data.len()).sum();
                        let processing_time = Duration::from_nanos(total_elements as u64);
                        tokio::time::sleep(processing_time).await;
                        
                        let results: Vec<usize> = batch.into_iter().map(|r| r.data.len()).collect();
                        Ok(results)
                    };
                    
                    let dispatcher = CorkDispatcher::new(config, processor).unwrap();
                    dispatcher.start().await.unwrap();
                    
                    // Create large request
                    let request = LargeRequest {
                        data: vec![1.0; data_size],
                    };
                    
                    let result = dispatcher.dispatch(black_box(request)).await.unwrap();
                    
                    dispatcher.shutdown().await.unwrap();
                    
                    black_box(result);
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_basic_batching,
    bench_concurrent_requests,
    bench_optimization_overhead,
    bench_large_batches
);
criterion_main!(benches);
