use crate::bench::generators::{GraphSize, generate_random_graph, generate_test_graph};
use crate::{Edge, EdgeIndex, Vertex, VertexIndex};
use criterion::{BenchmarkGroup, Throughput, measurement::WallTime};
use graph_api_lib::{Graph, Supported, VertexSearch};

/// Run all index operation benchmarks
pub fn run_benchmarks<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) where
    G: Graph<SupportsVertexLabelIndex = Supported>,
    G: Graph<SupportsVertexIndex = Supported>,
    G: Graph<SupportsEdgeLabelIndex = Supported>,
    G: Graph<SupportsVertexFullTextIndex = Supported>,
    G: Graph<SupportsVertexOrderedIndex = Supported>,
    G: Graph<SupportsVertexFullTextIndex = Supported>,
{
    bench_index_vertex_label(group, setup.clone());
    bench_index_vertex_property(group, setup.clone());
    bench_index_vertex_fulltext(group, setup.clone());
    bench_index_vertex_ordered_range(group, setup.clone());
    bench_index_edge_label(group, setup.clone());
}

/// Benchmark vertex label index lookup
fn bench_index_vertex_label<
    G: Graph<Vertex = Vertex, Edge = Edge, SupportsVertexLabelIndex = Supported>,
>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("index_vertex_label", |b| {
        // Setup: Create graph with random data
        let mut graph = setup();
        generate_random_graph(&mut graph, GraphSize::Small, 42);

        b.iter(|| {
            graph
                .walk()
                .vertices(VertexIndex::person())
                .limit(10)
                .collect::<Vec<_>>()
        })
    });
}

/// Benchmark vertex property index lookup
fn bench_index_vertex_property<
    G: Graph<Vertex = Vertex, Edge = Edge, SupportsVertexIndex = Supported>,
>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("index_vertex_property", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        generate_test_graph(&mut graph);

        b.iter(|| {
            graph
                .walk()
                .vertices(VertexIndex::person_by_name("Bryn"))
                .collect::<Vec<_>>()
        })
    });
}

/// Benchmark vertex full-text index lookup
fn bench_index_vertex_fulltext<
    G: Graph<Vertex = Vertex, Edge = Edge, SupportsVertexFullTextIndex = Supported>,
>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("index_vertex_fulltext", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        generate_test_graph(&mut graph);

        b.iter(|| {
            graph
                .walk()
                .vertices(VertexIndex::person_by_biography("graph"))
                .collect::<Vec<_>>()
        })
    });
}

/// Benchmark vertex ordered index range queries
fn bench_index_vertex_ordered_range<
    G: Graph<Vertex = Vertex, Edge = Edge, SupportsVertexOrderedIndex = Supported>,
>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("index_vertex_ordered_range", |b| {
        // Setup: Create graph with random data
        let mut graph = setup();
        generate_random_graph(&mut graph, GraphSize::Small, 42);

        b.iter(|| {
            graph
                .walk()
                .vertices(VertexIndex::person_by_age_range(30..50))
                .collect::<Vec<_>>()
        })
    });
}

/// Benchmark edge label index lookup
fn bench_index_edge_label<
    G: Graph<Vertex = Vertex, Edge = Edge, SupportsEdgeLabelIndex = Supported>,
>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("index_edge_label", |b| {
        // Setup: Create graph with random data
        let mut graph = setup();
        generate_random_graph(&mut graph, GraphSize::Small, 42);

        b.iter(|| {
            graph
                .walk()
                .vertices(VertexSearch::scan())
                .limit(1)
                .edges(EdgeIndex::knows())
                .collect::<Vec<_>>()
        })
    });
}
