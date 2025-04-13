use criterion::{BenchmarkGroup, measurement::WallTime};
use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::Graph;
use std::collections::HashSet;
use std::fmt::Debug;
use uuid::Uuid;

// Define a model with ONLY label index (no property indexes)
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    // No hash, range, or fulltext indices - only label (enum variant) is indexed
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

// Generate test data with the LabelVertex and LabelEdge models
pub fn populate_test_data<G>(graph: &mut G) -> (HashSet<G::VertexId>, G::VertexId, G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    let mut person_ids = HashSet::new();

    // Create person vertices for label queries
    for i in 0..100 {
        let id = graph.add_vertex(Vertex::Person {
            name: format!("Person{}", i),
            age: 25 + (i % 50) as u64,
            unique_id: Uuid::new_v4(),
            username: format!("user{}", i),
            biography: format!("Bio for person {}", i),
        });
        person_ids.insert(id);
    }

    // Add some non-person vertices
    let project = graph.add_vertex(Vertex::Project {
        name: "TestProject".to_string(),
    });

    let rust = graph.add_vertex(Vertex::Rust);

    (person_ids, project, rust)
}

#[cfg(feature = "vertex-label-index")]
pub fn run_benchmarks<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsVertexLabelIndex,
{
    // Benchmark vertex label index lookup
    group.throughput(criterion::Throughput::Elements(1));
    group.bench_function("vertex_label_lookup", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        populate_test_data(&mut graph);

        b.iter(|| {
            // Query by label (Person)
            let results = graph
                .walk()
                .vertices(Vertex::person())
                .take(10)
                .collect::<Vec<_>>();

            assert!(!results.is_empty());
            results
        })
    });

    // Benchmark insertion with label index
    group.bench_function("vertex_label_insertion", |b| {
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

    // Benchmark removal with label index
    group.bench_function("vertex_label_removal", |b| {
        // Setup: Create a new graph for each iteration
        b.iter_with_setup(
            || {
                let mut graph = setup();
                let vertex_id = graph.add_vertex(Vertex::Person {
                    name: "RemoveMe".to_string(),
                    age: 25,
                    unique_id: Uuid::new_v4(),
                    username: "remove_user".to_string(),
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

#[cfg(not(feature = "vertex-label-index"))]
pub fn run_benchmarks<G>(_group: &mut BenchmarkGroup<WallTime>, _setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // No-op when feature is disabled
}
