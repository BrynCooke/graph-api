use criterion::{BenchmarkGroup, Throughput, measurement::WallTime};
use graph_api_derive::{EdgeExt, VertexExt};
#[cfg(feature = "element-removal")]
use graph_api_lib::SupportsElementRemoval;
use graph_api_lib::{Graph, VertexSearch};
use std::fmt::Debug;
use uuid::Uuid;

// Define a model with no indexes for baseline performance
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        name: String,
        age: u64,
        unique_id: Uuid,
        username: String,
        biography: String,
    },
    Project {
        name: String,
    },
    Rust,
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows { since: i32 },
    Created,
    Language { name: String },
}

// Generate test data with the NoIndexVertex and NoIndexEdge models
pub fn generate_test_data<G>(graph: &mut G) -> (G::VertexId, G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    let src = graph.add_vertex(Vertex::Person {
        name: "Source".to_string(),
        age: 30,
        unique_id: Uuid::new_v4(),
        username: "source_user".to_string(),
        biography: "Source vertex".to_string(),
    });

    let dst = graph.add_vertex(Vertex::Person {
        name: "Target".to_string(),
        age: 35,
        unique_id: Uuid::new_v4(),
        username: "target_user".to_string(),
        biography: "Target vertex".to_string(),
    });

    (src, dst)
}

// Individual benchmark functions

pub fn bench_vertex_insertion<G>(
    group: &mut BenchmarkGroup<WallTime>,
    setup: impl Fn() -> G + Clone,
) where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Benchmark vertex insertion without any indexes
    group.throughput(Throughput::Elements(1));
    group.bench_function("vertex_insertion_no_index", |b| {
        let mut counter = 0;
        let mut graph = setup();
        b.iter(|| {
            counter += 1;
            graph.add_vertex(Vertex::Person {
                name: format!("Person{}", counter),
                age: 30,
                unique_id: Uuid::new_v4(),
                username: format!("noindex_user{}", counter),
                biography: "Test biography without index".to_string(),
            });
        })
    });
}

pub fn bench_edge_insertion<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Benchmark edge insertion without indexes
    group.bench_function("edge_insertion_no_index", |b| {
        // Setup: Create graph with minimal data for adding edges
        let mut graph = setup();
        let (src, dst) = generate_test_data(&mut graph);

        b.iter(|| graph.add_edge(src, dst, Edge::Knows { since: 2023 }))
    });
}

pub fn bench_vertex_scan<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Benchmark vertex scan (the non-indexed operation)
    group.bench_function("vertex_scan_no_index", |b| {
        // Setup: Create graph with random data
        let mut graph = setup();

        // Add some vertices for scanning
        for i in 0..100 {
            graph.add_vertex(Vertex::Person {
                name: format!("Person{}", i),
                age: 25 + (i % 50) as u64,
                unique_id: Uuid::new_v4(),
                username: format!("user{}", i),
                biography: format!("Bio for person {}", i),
            });
        }

        b.iter(|| {
            graph
                .walk()
                .vertices(VertexSearch::scan())
                .take(10)
                .collect::<Vec<_>>()
        })
    });
}

#[cfg(feature = "element-removal")]
pub fn bench_vertex_removal<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsElementRemoval,
{
    // Benchmark vertex removal without indexes
    group.bench_function("vertex_removal_no_index", |b| {
        // Setup: Create a new graph for each iteration
        b.iter_with_setup(
            || {
                let mut graph = setup();
                let vertex_id = graph.add_vertex(Vertex::Person {
                    name: "NoIndexRemoveMe".to_string(),
                    age: 25,
                    unique_id: Uuid::new_v4(),
                    username: "noindex_remove_user".to_string(),
                    biography: "To be removed without index".to_string(),
                });
                (graph, vertex_id)
            },
            |(mut graph, vertex_id)| {
                graph.remove_vertex(vertex_id);
            },
        )
    });
}

#[cfg(not(feature = "element-removal"))]
pub fn bench_vertex_removal<G>(
    _group: &mut BenchmarkGroup<WallTime>,
    _setup: impl Fn() -> G + Clone,
) where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // No-op when feature is disabled
}

#[cfg(feature = "element-removal")]
pub fn bench_edge_removal<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsElementRemoval,
{
    // Benchmark edge removal without indexes
    group.bench_function("edge_removal_no_index", |b| {
        // Setup: Create a new graph with an edge for each iteration
        b.iter_with_setup(
            || {
                let mut graph = setup();
                let (src, dst) = generate_test_data(&mut graph);
                let edge_id = graph.add_edge(src, dst, Edge::Knows { since: 2020 });
                (graph, edge_id)
            },
            |(mut graph, edge_id)| {
                graph.remove_edge(edge_id);
            },
        )
    });
}

#[cfg(not(feature = "element-removal"))]
pub fn bench_edge_removal<G>(_group: &mut BenchmarkGroup<WallTime>, _setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // No-op when feature is disabled
}
