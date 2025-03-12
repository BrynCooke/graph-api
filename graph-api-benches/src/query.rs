use crate::generators::{GraphSize, generate_random_graph, generate_test_graph};
use crate::{Edge, Vertex};
use criterion::{BenchmarkGroup, Throughput, measurement::WallTime};
use graph_api_lib::{EdgeSearch, Graph, VertexSearch};

/// Run all query operation benchmarks
pub fn run_benchmarks<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    bench_query_count_vertices(group, setup.clone());
    bench_query_count_edges(group, setup.clone());
    bench_query_first(group, setup.clone());
    bench_query_collect(group, setup.clone());
    bench_count(group, setup.clone());
}

/// Benchmark counting vertices
fn bench_query_count_vertices<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("query_count_vertices", |b| {
        // Setup: Create graph with random data
        let mut graph = setup();
        generate_random_graph(&mut graph, GraphSize::Small, 42);

        b.iter(|| graph.walk().vertices(VertexSearch::scan()).count())
    });
}

/// Benchmark counting edges
fn bench_query_count_edges<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("query_count_edges", |b| {
        // Setup: Create graph with random data
        let mut graph = setup();
        generate_random_graph(&mut graph, GraphSize::Small, 42);

        b.iter(|| {
            graph
                .walk()
                .vertices(VertexSearch::scan())
                .edges(EdgeSearch::scan())
                .count()
        })
    });
}

/// Benchmark finding first vertex matching criteria
fn bench_query_first<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("query_first", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        generate_test_graph(&mut graph);

        b.iter(|| graph.walk().vertices(VertexSearch::scan()).first())
    });
}

/// Benchmark collecting results into a collection
fn bench_query_collect<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("query_collect", |b| {
        // Setup: Create graph with random data
        let mut graph = setup();
        generate_random_graph(&mut graph, GraphSize::Small, 42);

        b.iter(|| {
            graph
                .walk()
                .vertices(VertexSearch::scan())
                .limit(25)
                .collect::<Vec<_>>()
        })
    });
}

/// Benchmark checking raw iteration via count
fn bench_count<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("query_is_empty", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        generate_test_graph(&mut graph);

        b.iter(|| graph.walk().vertices(VertexSearch::scan()).limit(0).count())
    });
}
