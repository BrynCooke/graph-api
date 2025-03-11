use crate::bench::generators::{GraphSize, generate_random_graph, generate_test_graph};
use crate::{Edge, Person, PersonMut, Vertex};
use criterion::{BenchmarkGroup, Throughput, measurement::WallTime};
use graph_api_lib::{Graph, VertexReference, VertexReferenceMut};
use uuid::Uuid;

/// Run all vertex operation benchmarks
pub fn run_benchmarks<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    bench_vertex_add(group, setup.clone());
    bench_vertex_retrieve(group, setup.clone());
    bench_vertex_remove(group, setup.clone());
    bench_vertex_property_access(group, setup.clone());
    bench_vertex_property_update(group, setup.clone());
}

/// Benchmark adding a vertex
fn bench_vertex_add<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("vertex_add", |b| {
        b.iter_batched(
            || setup(),
            |mut graph| {
                graph.add_vertex(Vertex::Person {
                    name: "Benchmark Person".to_string(),
                    age: 30,
                    unique_id: Uuid::new_v4(),
                    username: "benchmark_user".to_string(),
                    biography: "Created for benchmarking".to_string(),
                })
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

/// Benchmark retrieving a vertex by ID
fn bench_vertex_retrieve<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("vertex_retrieve", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        let refs = generate_test_graph(&mut graph);

        b.iter(|| graph.vertex(refs.bryn))
    });
}

/// Benchmark removing a vertex
fn bench_vertex_remove<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("vertex_remove", |b| {
        b.iter_batched(
            || {
                // Setup: Create graph with random data
                let mut graph = setup();
                let vertex_ids = generate_random_graph(&mut graph, GraphSize::Small, 42);
                (graph, vertex_ids.first().cloned().unwrap())
            },
            |(mut graph, vertex_id)| graph.remove_vertex(vertex_id),
            criterion::BatchSize::SmallInput,
        )
    });
}

/// Benchmark accessing vertex properties
fn bench_vertex_property_access<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("vertex_property_access", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        let refs = generate_test_graph(&mut graph);

        b.iter(|| {
            let vertex = graph.vertex(refs.bryn).unwrap();
            vertex.project::<Person<_>>().expect("person").name();
        })
    });
}

/// Benchmark updating vertex properties
fn bench_vertex_property_update<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    group.throughput(Throughput::Elements(1));
    group.bench_function("vertex_property_update", |b| {
        b.iter_batched(
            || {
                // Setup: Create graph with test data
                let mut graph = setup();
                let refs = generate_test_graph(&mut graph);
                (graph, refs.bryn)
            },
            |(mut graph, vertex_id)| {
                let mut vertex = graph.vertex_mut(vertex_id).unwrap();
                let mut person = vertex.project_mut::<PersonMut<_, _>>().expect("person");
                person.set_age(100);
            },
            criterion::BatchSize::SmallInput,
        )
    });
}
