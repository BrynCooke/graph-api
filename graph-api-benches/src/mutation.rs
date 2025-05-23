use crate::generators::{GraphSize, generate_random_graph, generate_test_graph};

use criterion::{BenchmarkGroup, Throughput, measurement::WallTime};
#[cfg(feature = "element-removal")]
use graph_api_lib::SupportsElementRemoval;
use graph_api_lib::{Graph, VertexReferenceMut, VertexSearch};
use graph_api_test::{Edge, PersonMut, Vertex, VertexExt};

/// Run all mutation operation benchmarks
pub fn run_benchmarks<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    bench_mutation_vertex_update(group, setup.clone());
    bench_mutation_edge_add(group, setup.clone());

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
    bench_mutation_edge_remove(group, setup.clone());
}

/// Benchmark updating vertices during traversal
fn bench_mutation_vertex_update<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(10));
    group.bench_function("mutation_vertex_update", |b| {
        b.iter_batched(
            || {
                // Setup: Create graph with random data
                let mut graph = setup();
                generate_random_graph(&mut graph, GraphSize::Small, 42);
                graph
            },
            |mut graph| {
                // Update age of first 10 Person vertices by incrementing it
                graph
                    .walk_mut()
                    .vertices(VertexSearch::scan())
                    .filter_person()
                    .take(10)
                    .mutate(|graph, vertex_id, _context| {
                        let mut vertex = graph.vertex_mut(vertex_id).expect("vertex");
                        let mut person = vertex.project_mut::<PersonMut<_, _>>().expect("person");
                        person.set_age(100)
                    })
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

/// Benchmark adding edges during traversal
fn bench_mutation_edge_add<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("mutation_edge_add", |b| {
        b.iter_batched(
            || {
                // Setup: Create graph with test data
                let mut graph = setup();
                let refs = generate_test_graph(&mut graph);
                (graph, refs.julia, refs.rust)
            },
            |(mut graph, source_id, target_id)| {
                // Add an edge during traversal
                graph
                    .walk_mut()
                    .vertices_by_id(vec![source_id])
                    .mutate(|graph, _, _| {
                        graph.add_edge(source_id, target_id, Edge::Created);
                    })
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

/// Benchmark removing edges during traversal
#[cfg(feature = "element-removal")]
fn bench_mutation_edge_remove<G: Graph<Vertex = Vertex, Edge = Edge> + SupportsElementRemoval>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("mutation_edge_remove", |b| {
        b.iter_batched(
            || {
                // Setup: Create graph with test data
                let mut graph = setup();
                let _refs = generate_test_graph(&mut graph);
                graph
            },
            |mut graph| {
                // Find and remove the first outgoing edge from Bryn
                let count = graph
                    .walk_mut()
                    .vertices(VertexSearch::scan())
                    .edges(EdgeSearch::scan().outgoing())
                    .take(1)
                    .mutate(|graph, edge_id, _| {
                        graph.remove_edge(edge_id);
                    });
                count
            },
            criterion::BatchSize::SmallInput,
        )
    });
}
