use criterion::{BenchmarkGroup, measurement::WallTime};
use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::Graph;
use std::fmt::Debug;
use uuid::Uuid;

// Define a model with ONLY range indexes (no hash or fulltext indexes)
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        name: String,
        #[index(range)]
        age: u64,
        unique_id: Uuid,
        #[index(range)]
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

// Generate test data with the RangeVertex and RangeEdge models
pub fn populate_test_data<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Create vertices with a range of ages
    for i in 0..100 {
        let age = 20 + i % 60; // Ages from 20 to 79
        graph.add_vertex(Vertex::Person {
            name: format!("Person{}", i),
            age,
            unique_id: Uuid::new_v4(),
            username: format!("user{}", i),
            biography: format!("Bio for person {}", i),
        });
    }

    // Add some non-person vertices
    graph.add_vertex(Vertex::Project {
        name: "TestProject".to_string(),
    });

    graph.add_vertex(Vertex::Rust);
}

#[cfg(feature = "vertex-range-index")]
pub fn run_benchmarks<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsVertexRangeIndex,
{
    // Benchmark range property index lookup
    group.throughput(criterion::Throughput::Elements(1));
    group.bench_function("vertex_range_age_lookup", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        populate_test_data(&mut graph);

        b.iter(|| {
            // Query by age range (30..50)
            let results = graph
                .walk()
                .vertices(Vertex::person_by_age_range(30..50))
                .collect::<Vec<_>>();

            assert!(!results.is_empty());
            results
        })
    });

    // Benchmark range property index lookup by username range
    group.bench_function("vertex_range_username_lookup", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        populate_test_data(&mut graph);

        b.iter(|| {
            // Query by username range
            let results = graph
                .walk()
                .vertices(Vertex::person_by_username_range("user1".."user5"))
                .collect::<Vec<_>>();

            // This might be empty depending on usernames
            results
        })
    });

    // Benchmark insertion with range index
    group.bench_function("vertex_range_insertion", |b| {
        let mut graph = setup();
        let mut age = 25;

        b.iter(|| {
            age = (age + 1) % 100; // Cycle through ages 25-99

            graph.add_vertex(Vertex::Person {
                name: format!("Range{}", age),
                age,
                unique_id: Uuid::new_v4(),
                username: format!("range_user{}", age),
                biography: "Test biography for range index".to_string(),
            })
        })
    });

    // Benchmark removal with range index
    group.bench_function("vertex_range_removal", |b| {
        // Setup: Create a new graph for each iteration
        b.iter_with_setup(
            || {
                let mut graph = setup();
                let vertex_id = graph.add_vertex(Vertex::Person {
                    name: "RangeRemoveMe".to_string(),
                    age: 42,
                    unique_id: Uuid::new_v4(),
                    username: "range_remove_user".to_string(),
                    biography: "To be removed from range index".to_string(),
                });
                (graph, vertex_id)
            },
            |(mut graph, vertex_id)| {
                graph.remove_vertex(vertex_id);
            },
        )
    });
}

#[cfg(not(feature = "vertex-range-index"))]
pub fn run_benchmarks<G>(_group: &mut BenchmarkGroup<WallTime>, _setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // No-op when feature is disabled
}
