use crate::generators::{GraphSize, generate_random_graph, generate_test_graph};

use criterion::{BenchmarkGroup, Throughput, measurement::WallTime};
use graph_api_lib::{
    Graph, VertexSearch, 
    SupportsEdgeLabelIndex, SupportsVertexFullTextIndex, SupportsVertexHashIndex,
    SupportsVertexLabelIndex, SupportsVertexRangeIndex
};
use graph_api_test::{Edge, EdgeIndex, Vertex, VertexIndex};

/// Run all index operation benchmarks
pub fn run_benchmarks<G>(
    #[allow(unused_variables)] group: &mut BenchmarkGroup<WallTime>,
    #[allow(unused_variables)] setup: impl Fn() -> G + Clone,
) where
    G: Graph<Vertex = Vertex, Edge = Edge>
        + SupportsVertexLabelIndex
        + SupportsVertexHashIndex 
        + SupportsVertexFullTextIndex
        + SupportsVertexRangeIndex
        + SupportsEdgeLabelIndex,
{
    #[cfg(feature = "vertex-label-index")]
    bench_index_vertex_label(group, setup.clone());
    #[cfg(feature = "vertex-index")]
    bench_index_vertex_property(group, setup.clone());
    #[cfg(feature = "vertex-full-text-index")]
    bench_index_vertex_fulltext(group, setup.clone());
    #[cfg(feature = "vertex-range-index")]
    bench_index_vertex_range_range(group, setup.clone());
    #[cfg(feature = "edge-label-index")]
    bench_index_edge_label(group, setup.clone());
}

/// Benchmark vertex label index lookup
#[allow(dead_code)]
fn bench_index_vertex_label<G>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexLabelIndex,
{
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
#[allow(dead_code)]
fn bench_index_vertex_property<G>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexHashIndex,
{
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
#[allow(dead_code)]
fn bench_index_vertex_fulltext<G>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexFullTextIndex,
{
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

/// Benchmark vertex range index range queries
#[allow(dead_code)]
fn bench_index_vertex_range_range<G>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexRangeIndex,
{
    group.throughput(Throughput::Elements(1));
    group.bench_function("index_vertex_range_range", |b| {
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
#[allow(dead_code)]
fn bench_index_edge_label<G>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
{
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
