use graph_api_lib::Graph;
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, Vertex, VertexIndex, populate_graph};

pub fn dbg_example() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    // Using dbg to debug a complex traversal
    println!("Starting complex traversal with debug output:");
    
    let result = graph
        .walk()
        .vertices(VertexIndex::person())
        .dbg("Found people")
        .filter(|vertex, _| {
            if let Vertex::Person { age, .. } = vertex.weight() {
                *age > 30
            } else {
                false
            }
        })
        .dbg("After filtering by age > 30")
        .count();
        
    println!("Found {} people over age 30", result);
}