use crate::standard_model::{EdgeExt, Vertex, standard_populated_graph};
use graph_api_lib::{EdgeSearch, Graph};

// ANCHOR: all
pub fn head_example() {
    let graph = standard_populated_graph();

    // Find the first person vertex to start the traversal
    let Some(start_person_id) = graph.walk().vertices(Vertex::person()).first() else {
        println!("No person found in the graph to start traversal.");
        return;
    };

    // ANCHOR: head_example
    // Find the projects created by people followed by the starting person
    let projects: Vec<_> = graph
        .walk()
        .vertices_by_id([start_person_id]) // Start at a specific person
        .edges(EdgeSearch::scan().outgoing()) // Follow outgoing edges
        .filter_follows() // Keep only 'Follows' edges
        .head() // Move to the target vertices (people followed by the start person)
        .edges(EdgeSearch::scan().outgoing()) // Follow outgoing edges from these people
        .filter_created() // Keep only 'Created' edges
        .head() // Move to the target vertices (projects created by known people)
        .collect();

    println!(
        "Projects created by people followed by the starting person ({:?}):",
        projects
    );
    // ANCHOR_END: head_example
}
// ANCHOR_END: all
