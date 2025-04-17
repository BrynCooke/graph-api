// ANCHOR: all
use crate::standard_model::{Vertex, VertexExt, standard_populated_graph};
use graph_api_lib::Graph;

// Function demonstrating how to use the dbg step
pub fn dbg_example() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: dbg_pipeline
    // Create a pipeline with multiple dbg steps to see how data transforms
    let result = graph
        .walk()
        .vertices(Vertex::person()) // Start with all people
        .dbg("all-people") // Debug all people vertices
        .filter_by_person(|person, _| {
            // Find people over 30
            person.age() > 30
        })
        .dbg("people-over-30") // Debug filtered results
        .count();

    println!("Found {} people over 30", result);
    // ANCHOR_END: dbg_pipeline
}
// ANCHOR_END: all
