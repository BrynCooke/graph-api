use graph_api_lib::Graph;
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, Vertex, VertexIndex, populate_graph};

pub fn probe_example() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    // Use probe to inspect vertices during traversal
    let person_count = graph
        .walk()
        .vertices(VertexIndex::person())
        .probe(|vertex, _| {
            // Extract and print information about each person
            if let Vertex::Person { name, age, .. } = vertex.weight() {
                println!("Traversing person: {} (age {})", name, age);
            }
        })
        .count();

    println!("Total people traversed: {}", person_count);
}