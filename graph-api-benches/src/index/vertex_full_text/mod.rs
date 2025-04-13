use criterion::{BenchmarkGroup, measurement::WallTime};

use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::Graph;
use std::fmt::Debug;
use uuid::Uuid;

// Define a model with ONLY full-text indexes (no hash or range indexes)
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        name: String,
        age: u64,
        unique_id: Uuid,
        username: String,
        #[index(full_text)]
        biography: String,
    },
    Project {
        name: String,
        #[index(full_text)]
        description: String,
    },
    Rust,
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows { since: i32 },
    Created,
    Language { name: String },
}

// Generate test data with the FullTextVertex and FullTextEdge models
pub fn populate_test_data<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Create specific text content we'll search for
    graph.add_vertex(Vertex::Person {
        name: "Bryn".to_string(),
        age: 45,
        unique_id: Uuid::new_v4(),
        username: "bryn".to_string(),
        biography: "Experienced in graph database technologies and implementations".to_string(),
    });

    // Add some other vertices with various biography texts
    let texts = [
        "Loves working with data structures",
        "Expert in graph algorithms and data models",
        "Interested in database performance optimization",
        "Has experience with various graph query languages",
        "Works on graph visualization techniques",
    ];

    for i in 0..100 {
        let text_idx = i % texts.len();
        graph.add_vertex(Vertex::Person {
            name: format!("Person{}", i),
            age: 25 + (i % 50) as u64,
            unique_id: Uuid::new_v4(),
            username: format!("user{}", i),
            biography: format!("{} and other skills", texts[text_idx]),
        });
    }

    // Add project vertices with descriptions
    graph.add_vertex(Vertex::Project {
        name: "GraphAPI".to_string(),
        description: "A Rust graph database API with support for various index types".to_string(),
    });

    graph.add_vertex(Vertex::Project {
        name: "OtherProject".to_string(),
        description: "Another project with different focus".to_string(),
    });

    graph.add_vertex(Vertex::Rust);
}

#[cfg(feature = "vertex-full-text-index")]
pub fn run_benchmarks<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsVertexFullTextIndex,
{
    // Benchmark full-text index lookup for biography
    group.throughput(criterion::Throughput::Elements(1));
    group.bench_function("vertex_fulltext_biography_lookup", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        populate_test_data(&mut graph);

        b.iter(|| {
            // Query by biography text containing "graph"
            let results = graph
                .walk()
                .vertices(Vertex::person_by_biography("graph"))
                .collect::<Vec<_>>();

            // Should find matches
            results
        })
    });

    // Benchmark full-text index lookup for project description
    group.bench_function("vertex_fulltext_description_lookup", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        populate_test_data(&mut graph);

        b.iter(|| {
            // Query by project description containing "database"
            let results = graph
                .walk()
                .vertices(Vertex::project_by_description("database"))
                .collect::<Vec<_>>();

            // Should find matches
            results
        })
    });

    // Benchmark insertion with full-text index
    group.bench_function("vertex_fulltext_insertion", |b| {
        let mut graph = setup();
        let mut counter = 0;

        b.iter(|| {
            counter += 1;

            graph.add_vertex(Vertex::Person {
                name: format!("FullText{}", counter),
                age: 30,
                unique_id: Uuid::new_v4(),
                username: format!("fulltext_user{}", counter),
                biography: format!("This is a test biography for fulltext indexing {}", counter),
            })
        })
    });

    // Benchmark removal with full-text index
    group.bench_function("vertex_fulltext_removal", |b| {
        // Setup: Create a new graph for each iteration
        b.iter_with_setup(
            || {
                let mut graph = setup();
                let vertex_id = graph.add_vertex(Vertex::Person {
                    name: "FullTextRemoveMe".to_string(),
                    age: 25,
                    unique_id: Uuid::new_v4(),
                    username: "fulltext_remove_user".to_string(),
                    biography: "To be removed from fulltext index with unique text".to_string(),
                });
                (graph, vertex_id)
            },
            |(mut graph, vertex_id)| {
                graph.remove_vertex(vertex_id);
            },
        )
    });
}

#[cfg(not(feature = "vertex-full-text-index"))]
pub fn run_benchmarks<G>(_group: &mut BenchmarkGroup<WallTime>, _setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // No-op when feature is disabled
}
