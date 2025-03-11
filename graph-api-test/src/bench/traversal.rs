use crate::bench::generators::{generate_social_graph, generate_test_graph, GraphSize};
use crate::{Edge, EdgeExt, Person, Vertex, VertexExt};
use criterion::{measurement::WallTime, BenchmarkGroup, Throughput};
use graph_api_lib::{EdgeSearch, Graph, VertexReference};

/// Run all traversal operation benchmarks
pub fn run_benchmarks<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    bench_traversal_outgoing(group, setup.clone());
    bench_traversal_incoming(group, setup.clone());
    bench_traversal_filtered(group, setup.clone());
    bench_traversal_complex(group, setup.clone());
    bench_traversal_deep(group, setup.clone());
}

/// Benchmark basic outgoing traversal
fn bench_traversal_outgoing<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("traversal_outgoing", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        let refs = generate_test_graph(&mut graph);

        b.iter(|| {
            graph
                .walk()
                .vertices_by_id(vec![refs.bryn])
                .edges(EdgeSearch::scan().outgoing())
                .head()
                .collect::<Vec<_>>()
        })
    });
}

/// Benchmark incoming traversal
fn bench_traversal_incoming<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("traversal_incoming", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        let refs = generate_test_graph(&mut graph);

        b.iter(|| {
            graph
                .walk()
                .vertices_by_id(vec![refs.bryn])
                .edges(EdgeSearch::scan().incoming())
                .head()
                .collect::<Vec<_>>()
        })
    });
}

/// Benchmark filtered traversal
fn bench_traversal_filtered<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("traversal_filtered", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        let refs = generate_test_graph(&mut graph);

        b.iter(|| {
            graph
                .walk()
                .vertices_by_id(vec![refs.bryn])
                .filter_by_person(|_, _| true)
                .edges(EdgeSearch::scan().outgoing())
                .filter_by_knows(|_, _| true)
                .head()
                .collect::<Vec<_>>()
        })
    });
}

/// Benchmark complex traversal combining multiple operations
fn bench_traversal_complex<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("traversal_complex", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        let refs = generate_test_graph(&mut graph);

        b.iter(|| {
            graph
                .walk()
                .vertices_by_id(vec![refs.bryn])
                .filter_by_person(|v, _| v.age() > 40)
                .push_context(|v, _ctx| v.project::<Person<_>>().unwrap().age())
                .edges(EdgeSearch::scan().outgoing())
                .limit(5)
                .head()
                .collect::<Vec<_>>()
        })
    });
}

/// Benchmark deep traversal (multiple hops)
fn bench_traversal_deep<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("traversal_deep", |b| {
        // Setup: Create a bigger graph for this test
        let mut graph = setup();
        let vertex_ids = generate_social_graph(&mut graph, GraphSize::Small, 42);
        let start_id = vertex_ids[0]; // Start with the first vertex

        b.iter(|| {
            // Friends of friends (2 hops)
            graph
                .walk()
                .vertices_by_id(vec![start_id])
                .edges(EdgeSearch::scan().outgoing())
                .head()
                .edges(EdgeSearch::scan().outgoing())
                .head()
                .collect::<Vec<_>>()
        })
    });
}
