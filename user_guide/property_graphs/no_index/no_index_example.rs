use crate::standard_model::{Vertex, VertexExt, standard_populated_graph};
use graph_api_lib::{Graph, VertexSearch};

// ANCHOR: all

// Example showing how to scan the entire graph without indexes
pub fn scan_example() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: scan_for_name
    // INEFFICIENT: Find all people named "Bryn" by scanning
    // Since name is not indexed, we must scan all vertices
    let bryn_vertices = graph
        .walk()
        .vertices(VertexSearch::scan()) // Must scan ALL vertices
        .filter_by_person(|person, _| person.name() == "Bryn")
        .collect::<Vec<_>>();

    println!("Found {} vertices for Bryn", bryn_vertices.len());
    // ANCHOR_END: scan_for_name

    // ANCHOR: scan_for_project
    // INEFFICIENT: Find all projects with a specific name by scanning
    // Since Project::name is not indexed, we must scan all vertices
    let graphapi_projects = graph
        .walk()
        .vertices(VertexSearch::scan()) // Must scan ALL vertices
        .filter_by_project(|project, _| project.name() == "GraphApi")
        .collect::<Vec<_>>();

    println!("Found {} GraphApi projects", graphapi_projects.len());
    // ANCHOR_END: scan_for_project

    // ANCHOR: comparison_scan
    // COMPARISON: Using an index vs. not using an index
    // 1. Inefficient: Find people with a specific username using a scan
    let julia_by_scan = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_by_person(|person, _| person.username() == "julia456")
        .collect::<Vec<_>>();

    println!("Found {} with username julia456", julia_by_scan.len());
    // ANCHOR_END: comparison_scan

    // ANCHOR: comparison_index
    // 2. Efficient: Find the same person using the username index
    let julia_by_index = graph
        .walk()
        .vertices(Vertex::person_by_username("julia456"))
        .collect::<Vec<_>>();

    println!("Found {} with username julia456", julia_by_index.len());
    // ANCHOR_END: comparison_index
}
// ANCHOR_END: all
