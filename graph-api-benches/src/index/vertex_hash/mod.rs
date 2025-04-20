use criterion::{BenchmarkGroup, measurement::WallTime};
use graph_api_derive::{EdgeExt, VertexExt};

use graph_api_lib::Graph;
#[cfg(feature = "element-removal")]
use graph_api_lib::SupportsElementRemoval;
use std::fmt::Debug;
use uuid::Uuid;

// Define a model with ONLY hash indexes (no label, range, or fulltext indexes)
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index(hash)]
        name: String,
        age: u64,
        unique_id: Uuid,
        username: String,
        biography: String,
    },
    Project {
        #[index(hash)]
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

// Generate test data with the HashVertex and HashEdge models
pub fn populate_test_data<G>(graph: &mut G) -> (G::VertexId, G::VertexId, Uuid)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Create a specific vertex we'll search for
    let bryn_id = Uuid::new_v4();
    let bryn = graph.add_vertex(Vertex::Person {
        name: "Bryn".to_string(),
        age: 45,
        unique_id: bryn_id,
        username: "bryn".to_string(),
        biography: "Test biography".to_string(),
    });

    // Add some other vertices
    for i in 0..100 {
        graph.add_vertex(Vertex::Person {
            name: format!("Person{}", i),
            age: 25 + (i % 50) as u64,
            unique_id: Uuid::new_v4(),
            username: format!("user{}", i),
            biography: format!("Bio for person {}", i),
        });
    }

    // Add a project vertex
    let project = graph.add_vertex(Vertex::Project {
        name: "TestProject".to_string(),
    });

    (bryn, project, bryn_id)
}

// Individual benchmark functions

#[cfg(feature = "vertex-hash-index")]
pub fn bench_lookup<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsVertexHashIndex,
{
    // Benchmark hash property index lookup
    group.throughput(criterion::Throughput::Elements(1));
    group.bench_function("vertex_hash_name_lookup", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        let (_, _, _) = populate_test_data(&mut graph);

        b.iter(|| {
            // Query by hash property (name)
            let results = graph
                .walk()
                .vertices(Vertex::person_by_name("Bryn"))
                .collect::<Vec<_>>();

            assert!(!results.is_empty());
            results
        })
    });
}

#[cfg(not(feature = "vertex-hash-index"))]
pub fn bench_lookup<G>(_group: &mut BenchmarkGroup<WallTime>, _setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // No-op when feature is disabled
}

#[cfg(feature = "vertex-hash-index")]
pub fn bench_insertion<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsVertexHashIndex,
{
    // Benchmark insertion with hash index
    group.bench_function("vertex_hash_insertion", |b| {
        let mut graph = setup();
        let mut counter = 0;

        b.iter(|| {
            counter += 1;

            graph.add_vertex(Vertex::Person {
                name: format!("Test{}", counter),
                age: 30,
                unique_id: Uuid::new_v4(),
                username: format!("test_user{}", counter),
                biography: "Test biography".to_string(),
            })
        })
    });
}

#[cfg(not(feature = "vertex-hash-index"))]
pub fn bench_insertion<G>(_group: &mut BenchmarkGroup<WallTime>, _setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // No-op when feature is disabled
}

#[cfg(all(feature = "vertex-hash-index", feature = "element-removal"))]
pub fn bench_removal<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>
        + graph_api_lib::SupportsVertexHashIndex
        + SupportsElementRemoval,
{
    // Benchmark removal with hash index
    group.bench_function("vertex_hash_removal", |b| {
        // Setup: Create a new graph for each iteration
        b.iter_with_setup(
            || {
                let mut graph = setup();
                let unique_id = Uuid::new_v4();
                let vertex_id = graph.add_vertex(Vertex::Person {
                    name: "HashRemoveMe".to_string(),
                    age: 25,
                    unique_id,
                    username: "hash_remove_user".to_string(),
                    biography: "To be removed".to_string(),
                });
                (graph, vertex_id)
            },
            |(mut graph, vertex_id)| {
                graph.remove_vertex(vertex_id);
            },
        )
    });
}

#[cfg(not(all(feature = "vertex-hash-index", feature = "element-removal")))]
pub fn bench_removal<G>(_group: &mut BenchmarkGroup<WallTime>, _setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // No-op when features are disabled
}
