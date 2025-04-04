# Benchmarking

Benchmarking is essential for measuring and comparing the performance of your graph implementation. This chapter covers how to set up and run benchmarks, interpret results, and use benchmarking to guide optimization.

## Setting Up Benchmarks

The Graph API provides built-in benchmarking tools to help you evaluate your implementation's performance.

### Adding Benchmarking Dependencies

First, add the necessary dependencies to your `Cargo.toml`:

```toml
[dev-dependencies]
criterion = "0.5"
graph-api-benches = { path = "../graph-api-benches" }

[[bench]]
name = "my_graph_benchmarks"
harness = false
```

### Creating a Basic Benchmark Suite

Create a benchmark file in your project's `benches` directory:

```rust
// benches/my_graph_benchmarks.rs
use criterion::{criterion_group, criterion_main, Criterion};
use graph_api_benches::bench_suite;
use my_graph::MyGraph;

fn criterion_benchmark(c: &mut Criterion) {
    bench_suite!(c, || MyGraph::new());
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

The `bench_suite!` macro runs a standardized set of benchmarks against your graph implementation.

## Standard Benchmark Suite

The standard benchmark suite in `graph-api-benches` tests several aspects of graph performance:

### 1. Construction Benchmarks

Measures the performance of creating graph elements:

- Creating vertices
- Creating edges
- Creating graphs of different sizes

### 2. Query Benchmarks

Evaluates lookup and traversal performance:

- Vertex retrieval by ID
- Vertex retrieval by index
- Edge traversal
- Path finding

### 3. Mutation Benchmarks

Tests the efficiency of modifying the graph:

- Adding vertices and edges
- Removing vertices and edges
- Modifying vertex and edge properties

### 4. Traversal Benchmarks

Measures the performance of walking the graph:

- Simple steps (vertices, edges)
- Filter operations
- Map operations
- Complex traversals

### 5. Scale Benchmarks

Assesses how performance scales with graph size:

- Small graphs (10s of vertices)
- Medium graphs (100s of vertices)
- Large graphs (1000s of vertices)
- Huge graphs (10,000s of vertices)

## Running Benchmarks

To run the benchmarks:

```bash
# Run all benchmarks
cargo bench

# Run a specific benchmark
cargo bench -- vertex_insertion

# Run benchmarks with more iterations for higher precision
CRITERION_SAMPLE_SIZE=100 cargo bench
```

## Custom Benchmarks

While the standard suite covers many scenarios, you should also create custom benchmarks for your implementation's specific features.

### Example: Custom Benchmark

```rust
fn custom_benchmarks(c: &mut Criterion) {
    // Benchmark for a specialized lookup method
    c.bench_function("custom_lookup", |b| {
        let mut graph = MyGraph::new();
        
        // Setup: Create a graph with test data
        let vertices = setup_test_graph(&mut graph);
        
        b.iter(|| {
            // Benchmark your custom operation
            for vertex_id in &vertices {
                graph.my_custom_lookup(*vertex_id);
            }
        })
    });
    
    // Benchmark for batch operations
    c.bench_function("batch_add_vertices", |b| {
        b.iter(|| {
            let mut graph = MyGraph::new();
            
            // Create test data
            let vertices = create_test_vertices(1000);
            
            // Benchmark the batch add operation
            graph.add_vertices_batch(vertices);
        })
    });
}

criterion_group!(custom_benches, custom_benchmarks);
```

## Comparative Benchmarking

To compare your implementation against other graph implementations:

```rust
fn comparison_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("vertex_insertion");
    
    // Benchmark your implementation
    group.bench_function("my_graph", |b| {
        b.iter(|| {
            let mut graph = MyGraph::new();
            for i in 0..1000 {
                graph.add_vertex(create_test_vertex(i));
            }
        })
    });
    
    // Benchmark SimpleGraph
    group.bench_function("simple_graph", |b| {
        b.iter(|| {
            let mut graph = SimpleGraph::new();
            for i in 0..1000 {
                graph.add_vertex(create_test_vertex(i));
            }
        })
    });
    
    // Benchmark PetGraph
    group.bench_function("pet_graph", |b| {
        b.iter(|| {
            let mut graph = PetGraph::new();
            for i in 0..1000 {
                graph.add_vertex(create_test_vertex(i));
            }
        })
    });
    
    group.finish();
}

criterion_group!(comparison_benches, comparison_benchmarks);
```

## Interpreting Benchmark Results

Criterion provides detailed statistics for each benchmark:

- **Mean time**: Average execution time
- **Median time**: Middle value of all measurements
- **Standard deviation**: Variation in measurements
- **Throughput**: Operations per second (for parameterized benchmarks)

### Sample Output Analysis

```
Benchmarking vertex_insertion: Collecting 100 samples in estimated 5.2684 s (10k iterations)
vertex_insertion           time:   [524.44 µs 525.30 µs 526.33 µs]
                           thrpt:  [1.9000 Melem/s 1.9037 Melem/s 1.9068 Melem/s]
```

This output shows:
- The benchmark took around 525 microseconds per iteration
- It processes about 1.9 million elements per second

### Comparing Results

When comparing benchmark results:

1. **Focus on relative differences**: A 5-10% difference might be noise; look for larger gaps
2. **Consider operation complexity**: Some operations naturally scale differently
3. **Check variance**: High standard deviation suggests inconsistent performance
4. **Look at throughput**: For scale benchmarks, throughput may be more meaningful than raw time

## Advanced Benchmarking Techniques

### Parameterized Benchmarks

Test how performance scales with input size:

```rust
fn parameterized_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("vertex_count_scaling");
    
    for size in [10, 100, 1000, 10000].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        group.bench_with_input(format!("vertices_{}", size), size, |b, &size| {
            b.iter(|| {
                let mut graph = MyGraph::new();
                for i in 0..size {
                    graph.add_vertex(create_test_vertex(i));
                }
            });
        });
    }
    
    group.finish();
}
```

### Profiling During Benchmarks

Use profiling tools with benchmarks:

```bash
# Generate a flame graph during benchmark execution
cargo flamegraph --bench my_graph_benchmarks -- --bench vertex_insertion

# Use perf for detailed profiling
perf record -g cargo bench -- vertex_insertion
perf report
```

## Real-World Benchmark Examples

### Memory Usage Benchmarking

Measure memory consumption:

```rust
fn memory_benchmark(c: &mut Criterion) {
    c.bench_function("memory_usage_1M_vertices", |b| {
        b.iter_custom(|iters| {
            let mut total_duration = std::time::Duration::new(0, 0);
            
            for _ in 0..iters {
                // Record baseline memory
                let baseline = get_current_memory_usage();
                
                // Start timing
                let start = std::time::Instant::now();
                
                // Create a large graph
                let graph = create_large_graph(1_000_000);
                
                // Force graph to stay alive until measurement
                std::hint::black_box(&graph);
                
                // Measure memory increase
                let peak = get_current_memory_usage();
                let memory_used = peak - baseline;
                
                // Log memory usage (not part of timing)
                println!("Memory used: {} MB", memory_used / (1024 * 1024));
                
                // End timing
                total_duration += start.elapsed();
            }
            
            total_duration
        });
    });
}

// Helper function to get memory usage
fn get_current_memory_usage() -> usize {
    // Use platform-specific APIs to get memory usage
    // For example, using the `psutil` crate
    // ...
}
```

### Concurrent Access Benchmarking

For graph implementations supporting concurrency:

```rust
fn concurrent_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_access");
    
    for threads in [1, 2, 4, 8, 16].iter() {
        group.bench_function(format!("read_threads_{}", threads), |b| {
            // Create a shared graph
            let graph = Arc::new(MyGraph::new());
            populate_test_graph(&graph);
            
            b.iter(|| {
                let mut handles = Vec::new();
                
                // Spawn reader threads
                for _ in 0..*threads {
                    let graph_clone = Arc::clone(&graph);
                    handles.push(std::thread::spawn(move || {
                        // Perform concurrent read operations
                        for _ in 0..1000 {
                            let _ = graph_clone.walk()
                                .vertices(VertexSearch::scan())
                                .collect::<Vec<_>>();
                        }
                    }));
                }
                
                // Wait for all threads
                for handle in handles {
                    handle.join().unwrap();
                }
            });
        });
    }
    
    group.finish();
}
```

## Using Benchmarks to Guide Optimization

Benchmarks are most valuable when used as part of your optimization process:

1. **Establish a baseline**: Run benchmarks before making any changes
2. **Identify bottlenecks**: Use results to find which operations are slowest
3. **Make targeted improvements**: Change one thing at a time
4. **Measure impact**: Run benchmarks after each change
5. **Validate trade-offs**: Ensure optimizations don't degrade other aspects

### Example Optimization Workflow

```
1. Initial benchmark:
   vertex_insertion: 525.30 µs

2. Profiling shows excessive allocations in vertex insertion
   - Modify implementation to reduce allocations

3. After optimization:
   vertex_insertion: 320.15 µs (39% improvement)

4. Check if other operations are affected:
   vertex_removal: 212.40 µs (unchanged)
   edge_insertion: 187.30 µs (5% improvement)
```

## Benchmark-Driven Development

Consider adopting benchmark-driven development for performance-critical components:

1. **Write benchmarks first**: Define performance expectations
2. **Implement the feature**: Focus on correctness first
3. **Run benchmarks**: Measure initial performance
4. **Optimize iteratively**: Improve performance until targets are met
5. **Document results**: Record performance characteristics

## Common Pitfalls in Benchmarking

Watch out for these benchmarking issues:

1. **Measurement noise**: External factors affecting results
2. **Microbenchmark traps**: Optimizing operations that don't matter in real use
3. **Compiler optimizations**: Dead code elimination skewing results
4. **Benchmark bias**: Testing only favorable scenarios
5. **Invalid comparisons**: Comparing different operations as if they were equivalent

### Avoiding Benchmarking Mistakes

```rust
// BAD: Allows dead code elimination
c.bench_function("ineffective_bench", |b| {
    b.iter(|| {
        let mut graph = MyGraph::new();
        for i in 0..1000 {
            graph.add_vertex(create_test_vertex(i));
        }
        // Results never used, may be optimized away
    })
});

// GOOD: Prevents optimization with black_box
c.bench_function("effective_bench", |b| {
    b.iter(|| {
        let mut graph = MyGraph::new();
        for i in 0..1000 {
            graph.add_vertex(create_test_vertex(i));
        }
        // Force evaluation
        criterion::black_box(&graph);
    })
});
```

## Benchmarking Checklist

Use this checklist when creating benchmarks:

- [ ] Include representative operations
- [ ] Test a range of input sizes
- [ ] Include setup code outside timed sections
- [ ] Use `black_box` to prevent dead code elimination
- [ ] Run multiple iterations for statistical significance
- [ ] Test on the target hardware when possible
- [ ] Include comparison benchmarks against similar implementations
- [ ] Document benchmark configurations and results

## Conclusion

Effective benchmarking is crucial for developing high-performance graph implementations. By creating comprehensive benchmarks, accurately measuring performance, and using results to guide optimization, you can ensure your graph implementation meets its performance requirements.

Remember that benchmarks are most valuable when they reflect real-world usage patterns. Focus on benchmarking operations that will be common in actual applications, and be cautious about optimizing solely for benchmark performance at the expense of code clarity or maintainability.