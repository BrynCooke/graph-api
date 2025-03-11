use crate::bench::generators::{
    GraphSize, generate_project_graph, generate_random_graph, generate_social_graph,
};
use crate::{Edge, Vertex};
use criterion::{BenchmarkGroup, Throughput, measurement::WallTime};
use graph_api_lib::Graph;

/// Run all construction benchmarks
pub fn run_benchmarks<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    bench_construction_small(group, setup.clone());
    bench_construction_medium(group, setup.clone());
    bench_construction_social(group, setup.clone());
    bench_construction_project(group, setup.clone());
    bench_construction_batch(group, setup.clone());
}

/// Benchmark creating a small graph
fn bench_construction_small<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    let size = GraphSize::Small;
    let vertex_count = size.vertex_count();
    let edge_count = vertex_count * size.edge_multiplier();

    group.throughput(Throughput::Elements((vertex_count + edge_count) as u64));
    group.bench_function("construction_small", |b| {
        b.iter_batched(
            || setup(),
            |mut graph| {
                generate_random_graph(&mut graph, size, 42);
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

/// Benchmark creating a medium graph
fn bench_construction_medium<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    let size = GraphSize::Medium;
    let vertex_count = size.vertex_count();
    let edge_count = vertex_count * size.edge_multiplier();

    group.throughput(Throughput::Elements((vertex_count + edge_count) as u64));
    group.bench_function("construction_medium", |b| {
        b.iter_batched(
            || setup(),
            |mut graph| {
                generate_random_graph(&mut graph, size, 42);
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

/// Benchmark creating a social network graph
fn bench_construction_social<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    let size = GraphSize::Small;
    let vertex_count = size.vertex_count();
    let edge_count = vertex_count * size.edge_multiplier();

    group.throughput(Throughput::Elements((vertex_count + edge_count) as u64));
    group.bench_function("construction_social", |b| {
        b.iter_batched(
            || setup(),
            |mut graph| {
                generate_social_graph(&mut graph, size, 42);
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

/// Benchmark creating a project dependency graph
fn bench_construction_project<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    let size = GraphSize::Small;
    let vertex_count = size.vertex_count();
    let edge_count = vertex_count * size.edge_multiplier();

    group.throughput(Throughput::Elements((vertex_count + edge_count) as u64));
    group.bench_function("construction_project", |b| {
        b.iter_batched(
            || setup(),
            |mut graph| {
                generate_project_graph(&mut graph, size, 42);
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

/// Benchmark batch vertex and edge insertion
fn bench_construction_batch<G: Graph<Vertex = Vertex, Edge = Edge>>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) {
    const BATCH_SIZE: usize = 100;

    group.throughput(Throughput::Elements(BATCH_SIZE as u64));
    group.bench_function("construction_batch_vertex", |b| {
        b.iter_batched(
            || setup(),
            |mut graph| {
                // Create a batch of vertices
                for i in 0..BATCH_SIZE {
                    graph.add_vertex(Vertex::Person {
                        name: format!("BatchPerson-{}", i),
                        age: 30 + (i % 50) as u64,
                        unique_id: uuid::Uuid::new_v4(),
                        username: format!("batch_user_{}", i),
                        biography: format!("Batch created person {}", i),
                    });
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
}
