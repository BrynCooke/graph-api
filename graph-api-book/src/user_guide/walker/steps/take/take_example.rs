use crate::standard_model::{VertexExt, VertexIndex, standard_populated_graph};
use graph_api_lib::{EdgeSearch, Graph};

// ANCHOR: all
// Function demonstrating the take step
pub fn take_example() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: basic_usage
    // Get the first 2 people in the graph
    println!("First 2 people in the graph:");
    let people = graph
        .walk()
        .vertices(VertexIndex::person())
        .take(2) // Only take the first 2 vertices
        .collect::<Vec<_>>();

    for person in &people {
        println!("  - {:?}", person);
    }
    println!("Found {} people (limited to 2)", people.len());
    // ANCHOR_END: basic_usage

    // ANCHOR: with_filter
    // Get up to 3 people over the age of 30
    println!("\nUp to 3 people over 30:");
    let older_people = graph
        .walk()
        .vertices(VertexIndex::person()) // Get all Person vertices
        .filter_by_person(|person, _| {
            // Using the typed projection with accessor methods
            person.age() > 30
        })
        .take(3) // Only take up to 3 matches
        .collect::<Vec<_>>();

    for person in &older_people {
        println!("  - {:?}", person);
    }
    println!("Found {} people (limited to 3)", older_people.len());
    // ANCHOR_END: with_filter

    // ANCHOR: edge_example
    // Find the first 2 connections from the first person
    if let Some(first_person) = graph.walk().vertices(VertexIndex::person()).first() {
        println!("\nFirst 2 connections from a person:");

        let connections = graph
            .walk()
            .vertices_by_id([first_person])
            .edges(EdgeSearch::scan()) // All edge types
            .take(2) // Only take the first 2 edges
            .collect::<Vec<_>>();

        for edge in &connections {
            println!("  - {:?}", edge);
        }
        println!("Found {} connections (limited to 2)", connections.len());
    }
    // ANCHOR_END: edge_example
}
// ANCHOR_END: all
