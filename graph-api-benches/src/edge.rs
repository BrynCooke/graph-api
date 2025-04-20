use crate::generators::generate_test_graph;

use criterion::{BenchmarkGroup, Throughput, measurement::WallTime};
#[cfg(feature = "element-removal")]
use graph_api_lib::SupportsElementRemoval;
use graph_api_lib::{EdgeReference, Graph};
use graph_api_test::{Edge, Knows, Vertex};

/// Run all edge operation benchmarks
pub fn run_benchmarks<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    bench_edge_add(group, setup.clone());
    bench_edge_retrieve(group, setup.clone());
    bench_edge_property_access(group, setup.clone());

    // Only run removal benchmarks if the feature is enabled and graph supports removal
    #[cfg(feature = "element-removal")]
    run_removal_benchmarks(group, setup);
}

/// Run removal-specific benchmarks
#[cfg(feature = "element-removal")]
fn run_removal_benchmarks<G: Graph<Vertex = Vertex, Edge = Edge> + SupportsElementRemoval>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    bench_edge_remove(group, setup.clone());
}

/// Benchmark adding an edge
fn bench_edge_add<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("edge_add", |b| {
        b.iter_batched(
            || {
                // Setup: Create graph with test data
                let mut graph = setup();
                let refs = generate_test_graph(&mut graph);
                (graph, refs.bryn, refs.julia)
            },
            |(mut graph, source, target)| {
                graph.add_edge(source, target, Edge::Knows { since: 2023 })
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

/// Benchmark retrieving an edge by ID
fn bench_edge_retrieve<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("edge_retrieve", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        let refs = generate_test_graph(&mut graph);

        b.iter(|| graph.edge(refs.bryn_knows_julia))
    });
}

/// Benchmark removing an edge
#[cfg(feature = "element-removal")]
fn bench_edge_remove<G: Graph<Vertex = Vertex, Edge = Edge> + SupportsElementRemoval>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("edge_remove", |b| {
        b.iter_batched(
            || {
                // Setup: Create graph with random data
                let mut graph = setup();
                let refs = generate_test_graph(&mut graph);
                // Add a new edge that we'll remove during the benchmark
                let edge_id = graph.add_edge(refs.bryn, refs.julia, Edge::Knows { since: 2023 });
                (graph, edge_id)
            },
            |(mut graph, edge_id)| graph.remove_edge(edge_id),
            criterion::BatchSize::SmallInput,
        )
    });
}

/// Benchmark accessing edge properties
fn bench_edge_property_access<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("edge_property_access", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        let refs = generate_test_graph(&mut graph);

        b.iter(|| {
            let edge = graph.edge(refs.bryn_knows_julia).unwrap();
            let knows = edge.project::<Knows<_>>().expect("knows");
            knows.since()
        })
    });
}
