#[cfg(any(
    feature = "edge-label-index",
    feature = "vertex-full-text-index",
    feature = "vertex-hash-index",
    feature = "vertex-label-index",
    feature = "vertex-range-index"
))]
use crate::generators::{GraphSize, generate_random_graph, generate_test_graph};
use criterion::{BenchmarkGroup, measurement::WallTime};

#[cfg(any(
    feature = "edge-label-index",
    feature = "vertex-full-text-index",
    feature = "vertex-hash-index",
    feature = "vertex-label-index",
    feature = "vertex-range-index"
))]
use criterion::Throughput;
#[cfg(feature = "edge-label-index")]
use graph_api_lib::SupportsEdgeLabelIndex;
#[cfg(feature = "vertex-full-text-index")]
use graph_api_lib::SupportsVertexFullTextIndex;
#[cfg(feature = "vertex-hash-index")]
use graph_api_lib::SupportsVertexHashIndex;
#[cfg(feature = "vertex-label-index")]
use graph_api_lib::SupportsVertexLabelIndex;
#[cfg(feature = "vertex-range-index")]
use graph_api_lib::SupportsVertexRangeIndex;

#[cfg(any(
    feature = "edge-label-index",
    feature = "vertex-full-text-index",
    feature = "vertex-hash-index",
    feature = "vertex-label-index",
    feature = "vertex-range-index"
))]
use graph_api_lib::{Graph, VertexSearch};
#[cfg(any(
    feature = "edge-label-index",
    feature = "vertex-full-text-index",
    feature = "vertex-hash-index",
    feature = "vertex-label-index",
    feature = "vertex-range-index"
))]
use graph_api_test::{Edge, EdgeIndex, Vertex, VertexIndex};

/// Benchmark vertex label index lookup
#[cfg(feature = "vertex-label-index")]
pub fn bench_vertex_label_index<G>(
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

#[cfg(not(feature = "vertex-label-index"))]
pub fn bench_vertex_label_index<G>(
    _group: &mut BenchmarkGroup<WallTime>,
    _setup: impl Fn() -> G + Clone,
) {
}

/// Benchmark vertex property index lookup
#[cfg(feature = "vertex-hash-index")]
pub fn bench_vertex_hash_index<G>(
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
#[cfg(not(feature = "vertex-hash-index"))]
pub fn bench_vertex_hash_index<G>(
    _group: &mut BenchmarkGroup<WallTime>,
    _setup: impl Fn() -> G + Clone,
) {
}

/// Benchmark vertex full-text index lookup
#[cfg(feature = "vertex-full-text-index")]
pub fn bench_vertex_fulltext_index<G>(
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

#[cfg(not(feature = "vertex-full-text-index"))]
pub fn bench_vertex_fulltext_index<G>(
    _group: &mut BenchmarkGroup<WallTime>,
    _setup: impl Fn() -> G + Clone,
) {
}

/// Benchmark vertex range index range queries
#[cfg(feature = "vertex-range-index")]
pub fn bench_vertex_range_index<G>(
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

#[cfg(not(feature = "vertex-range-index"))]
pub fn bench_vertex_range_index<G>(
    _group: &mut BenchmarkGroup<WallTime>,
    _setup: impl Fn() -> G + Clone,
) {
}

/// Benchmark edge label index lookup
#[cfg(feature = "edge-label-index")]
pub fn bench_edge_label_index<G>(
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

#[cfg(not(feature = "edge-label-index"))]
pub fn bench_edge_label_index<G>(
    _group: &mut BenchmarkGroup<WallTime>,
    _setup: impl Fn() -> G + Clone,
) {
}
