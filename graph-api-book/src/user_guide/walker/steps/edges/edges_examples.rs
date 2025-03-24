use crate::standard_model::{EdgeIndex, VertexIndex, standard_populated_graph};
use graph_api_lib::{EdgeSearch, Graph};
// ANCHOR: all
// Function demonstrating the edges step
pub fn edges_step_example() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();
    let bryn = graph
        .walk()
        .vertices(VertexIndex::person_by_username("bryn123"))
        .first()
        .expect("bryn");

    // ANCHOR: all_edges
    // Get all edges (both incoming and outgoing) from a vertex
    let all_connected_edges = graph
        .walk()
        .vertices_by_id([bryn])
        .edges(EdgeSearch::scan())
        .collect::<Vec<_>>();

    println!(
        "Found {} total edges connected to Bryn",
        all_connected_edges.len()
    );
    // ANCHOR_END: all_edges

    // ANCHOR: directional
    // Get only outgoing edges from a vertex
    let outgoing_edges = graph
        .walk()
        .vertices_by_id([bryn])
        .edges(EdgeSearch::scan().outgoing())
        .collect::<Vec<_>>();

    println!("Found {} outgoing edges from Bryn", outgoing_edges.len());

    // Get only incoming edges to a vertex
    let incoming_edges = graph
        .walk()
        .vertices_by_id([bryn])
        .edges(EdgeSearch::scan().incoming())
        .collect::<Vec<_>>();

    println!("Found {} incoming edges to Bryn", incoming_edges.len());
    // ANCHOR_END: directional

    // ANCHOR: label_filter
    // Get only edges with a specific label
    // Using the label index is more efficient
    let created_edges = graph
        .walk()
        .vertices_by_id([bryn])
        .edges(EdgeIndex::created())
        .collect::<Vec<_>>();

    println!("Found {} 'Created' edges for Bryn", created_edges.len());
    // ANCHOR_END: label_filter

    // ANCHOR: combined_filter
    // Combine direction and label filtering
    let outgoing_follows_edges = graph
        .walk()
        .vertices_by_id([bryn])
        .edges(EdgeIndex::follows().outgoing())
        .collect::<Vec<_>>();

    println!("Bryn follows {} people", outgoing_follows_edges.len());

    // Find incoming follows edges (people who follow Bryn)
    let incoming_follows_edges = graph
        .walk()
        .vertices_by_id([bryn])
        .edges(EdgeIndex::follows().incoming())
        .collect::<Vec<_>>();

    println!("{} people follow Bryn", incoming_follows_edges.len());
    // ANCHOR_END: combined_filter
}
// ANCHOR_END: all
