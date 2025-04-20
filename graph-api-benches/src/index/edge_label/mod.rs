use criterion::{BenchmarkGroup, measurement::WallTime};
use graph_api_derive::{EdgeExt, VertexExt};
use graph_api_lib::Graph;
#[cfg(feature = "element-removal")]
use graph_api_lib::SupportsElementRemoval;

use std::fmt::Debug;
use uuid::Uuid;

// Define a model with edge label indexes
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

// Generate test data with the EdgeLabelVertex and EdgeLabelEdge models
pub fn populate_test_data<G>(graph: &mut G) -> (Vec<G::VertexId>, Vec<G::EdgeId>)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    let mut person_ids = vec![];
    let mut edge_ids = vec![];

    // Create person vertices
    for i in 0..20 {
        let id = graph.add_vertex(Vertex::Person {
            name: format!("Person{}", i),
            age: 25 + (i % 50) as u64,
            unique_id: Uuid::new_v4(),
            username: format!("user{}", i),
            biography: format!("Bio for person {}", i),
        });
        person_ids.push(id);
    }

    // Create edges of different types
    for i in 0..19 {
        // Create "knows" edges
        if i % 3 == 0 {
            let edge_id = graph.add_edge(
                person_ids[i],
                person_ids[i + 1],
                Edge::Knows {
                    since: 2020 + (i as i32),
                },
            );
            edge_ids.push(edge_id);
        }
    }

    // Add a project and created edges
    let project = graph.add_vertex(Vertex::Project {
        name: "TestProject".to_string(),
    });

    for id in person_ids.iter().take(5) {
        let edge_id = graph.add_edge(*id, project, Edge::Created);
        edge_ids.push(edge_id);
    }

    // Add rust and language edges
    let rust = graph.add_vertex(Vertex::Rust);

    let edge_id = graph.add_edge(
        project,
        rust,
        Edge::Language {
            name: "Rust".to_string(),
        },
    );
    edge_ids.push(edge_id);

    (person_ids, edge_ids)
}

// Individual benchmark functions

#[cfg(feature = "edge-label-index")]
pub fn bench_lookup<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsEdgeLabelIndex,
{
    // Benchmark edge label index lookup for "knows" edges
    group.throughput(criterion::Throughput::Elements(1));
    group.bench_function("edge_label_knows_lookup", |b| {
        // Setup: Create graph with test data
        let mut graph = setup();
        populate_test_data(&mut graph);

        b.iter(|| {
            // Query edges with "knows" label
            let results = graph
                .walk()
                .vertices(graph_api_lib::VertexSearch::scan())
                .take(5)
                .edges(Edge::knows())
                .collect::<Vec<_>>();

            results
        })
    });
}

#[cfg(not(feature = "edge-label-index"))]
pub fn bench_lookup<G>(_group: &mut BenchmarkGroup<WallTime>, _setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // No-op when feature is disabled
}

#[cfg(feature = "edge-label-index")]
pub fn bench_insertion<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsEdgeLabelIndex,
{
    // Benchmark insertion with edge label index
    group.bench_function("edge_label_insertion", |b| {
        // Setup: Create graph with minimal data for adding edges
        let mut graph = setup();
        let (person_ids, _) = populate_test_data(&mut graph);
        let src = person_ids[0];
        let dst = person_ids[1];

        b.iter(|| graph.add_edge(src, dst, Edge::Knows { since: 2023 }))
    });
}

#[cfg(not(feature = "edge-label-index"))]
pub fn bench_insertion<G>(_group: &mut BenchmarkGroup<WallTime>, _setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // No-op when feature is disabled
}

#[cfg(all(feature = "edge-label-index", feature = "element-removal"))]
pub fn bench_removal<G>(group: &mut BenchmarkGroup<WallTime>, setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>
        + graph_api_lib::SupportsEdgeLabelIndex
        + SupportsElementRemoval,
{
    // Benchmark removal with edge label index
    group.bench_function("edge_label_removal", |b| {
        // Setup: Create a new graph with an edge for each iteration
        b.iter_with_setup(
            || {
                let mut graph = setup();
                let (person_ids, _) = populate_test_data(&mut graph);
                let edge_id =
                    graph.add_edge(person_ids[0], person_ids[1], Edge::Knows { since: 2020 });
                (graph, edge_id)
            },
            |(mut graph, edge_id)| {
                graph.remove_edge(edge_id);
            },
        )
    });
}

#[cfg(not(all(feature = "edge-label-index", feature = "element-removal")))]
pub fn bench_removal<G>(_group: &mut BenchmarkGroup<WallTime>, _setup: impl Fn() -> G + Clone)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // No-op when features are disabled
}
