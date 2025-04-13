use crate::standard_model::VertexExt as VertexExt2;
use crate::standard_model::{Vertex, standard_populated_graph};
use graph_api_derive::VertexExt;
use graph_api_lib::{Graph, VertexSearch};
// ANCHOR: model_definition
#[derive(Debug, VertexExt)]
pub enum IndexedVertex {
    // Person vertex with various properties
    Person {
        name: String, // Not indexed

        #[index(hash)] // Hash index for exact lookups
        username: String,
    },
}
// ANCHOR_END: model_definition

// ANCHOR: all
// Example demonstrating hash index search capabilities
pub fn hash_index_example() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: hash_index_queries
    // Find a person by their username (using hash index)
    let julia = graph
        .walk()
        .vertices(Vertex::person_by_username("julia456"))
        .first();

    println!("Found Julia: {}", julia.is_some());
    // ANCHOR_END: hash_index_queries

    // ANCHOR: hash_vs_scan
    // Comparison: Hash index vs. full scan

    // 1. Using hash index (efficient)
    let username = "eve789";
    let by_index = graph
        .walk()
        .vertices(Vertex::person_by_username(username))
        .collect::<Vec<_>>();

    // 2. Using scan (inefficient)
    let by_scan = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_by_person(|person, _| person.username() == username)
        .collect::<Vec<_>>();

    // Both approaches find the same vertices
    assert_eq!(by_index.len(), by_scan.len());
    // But the hash index is much more efficient for large graphs
    // ANCHOR_END: hash_vs_scan
}
// ANCHOR_END: all
