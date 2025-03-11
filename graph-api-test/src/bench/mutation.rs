use crate::bench::generators::{generate_random_graph, generate_test_graph, GraphSize};
use crate::{Edge, PersonMut, Vertex};
use criterion::{measurement::WallTime, BenchmarkGroup, Throughput};
use graph_api_lib::{EdgeSearch, Graph, VertexReferenceMut, VertexSearch};

/// Run all mutation operation benchmarks
pub fn run_benchmarks<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    bench_mutation_vertex_update(group, setup.clone());
    bench_mutation_edge_add(group, setup.clone());
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
                    .limit(10)
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
fn bench_mutation_edge_remove<G: Graph<Vertex = Vertex, Edge = Edge>>(
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
                    .limit(1)
                    .mutate(|graph, edge_id, _| {
                        graph.remove_edge(edge_id);
                    });
                count
            },
            criterion::BatchSize::SmallInput,
        )
    });
}
