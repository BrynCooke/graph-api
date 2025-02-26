use crate::{Edge, Vertex};
use crate::bench::generators::{generate_random_graph, GraphSize};
use criterion::{BenchmarkGroup, Throughput, measurement::WallTime};
use graph_api_lib::{EdgeSearch, Graph,  VertexSearch};

/// Run all scaling benchmarks
pub fn run_benchmarks<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    bench_scale_vertex_count(group, setup.clone());
    bench_scale_edge_traversal(group, setup.clone());
}

/// Benchmark how performance scales with vertex count
fn bench_scale_vertex_count<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    // Test with different graph sizes
    for size in &[GraphSize::Small, GraphSize::Medium, GraphSize::Large, GraphSize::Huge] {
        let vertex_count = size.vertex_count();
        
        group.throughput(Throughput::Elements(vertex_count as u64));
        group.bench_function(&format!("scale_vertex_count_{}", vertex_count), |b| {
            // Setup: Create graph with given size
            let mut graph = setup();
            generate_random_graph(&mut graph, *size, 42);
            
            b.iter(|| {
                // Count all vertices
                graph.walk()
                    .vertices(VertexSearch::scan())
                    .count()
            })
        });
    }
}

/// Benchmark how traversal performance scales with edge count
fn bench_scale_edge_traversal<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    // Test with different graph sizes to see how edge density affects traversal
    for size in &[GraphSize::Small, GraphSize::Medium] {
        let vertex_count = size.vertex_count();
        
        group.throughput(Throughput::Elements(vertex_count as u64));
        group.bench_function(&format!("scale_edge_traversal_{}", vertex_count), |b| {
            // Setup: Create graph with given size
            let mut graph = setup();
            let vertex_ids = generate_random_graph(&mut graph, *size, 42);
            let start_id = vertex_ids[0]; // Start with the first vertex
            
            b.iter(|| {
                // Traverse outgoing edges from a vertex
                graph.walk()
                    .vertices_by_id(vec![start_id])
                    .edges(EdgeSearch::scan())
                    .head()
                    .collect::<Vec<_>>()
            })
        });
    }
}