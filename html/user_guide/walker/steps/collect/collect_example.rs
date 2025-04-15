use crate::standard_model::{Person, Vertex, VertexExt, standard_populated_graph};
use graph_api_lib::{Graph, VertexReference};
use std::collections::{BTreeSet, HashSet};

// ANCHOR: all
// Function demonstrating how to use the collect step
pub fn collect_example() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: collect_vec
    // Collect results into a Vec
    // Use the person() index method to get all Person vertices
    let person_vertices: Vec<_> = graph
        .walk()
        .vertices(Vertex::person()) // Type-safe vertex lookup by label
        .collect::<Vec<_>>();

    println!("Found {} person vertices", person_vertices.len());
    // ANCHOR_END: collect_vec

    // ANCHOR: collect_hashset
    // Collect into a HashSet for unique elements
    let unique_names: HashSet<String> = graph
        .walk()
        .vertices(Vertex::person())
        // Use map to extract properties from each person
        .map(|person, _| {
            // Use projection to access Person methods in a type-safe way
            person
                .project::<Person<_>>() // Project to Person type
                .map(|p| p.name().to_string()) // Use accessor method from projection
                .unwrap_or_else(|| "Unknown".to_string())
        })
        .collect::<HashSet<String>>();

    println!("Found {} unique person names", unique_names.len());
    // ANCHOR_END: collect_hashset

    // ANCHOR: collect_btreeset
    // Collect into a BTreeSet for range unique elements
    let range_ages: BTreeSet<u8> = graph
        .walk()
        .vertices(Vertex::person())
        // Use filter_person() to work exclusively with Person vertices (no closure needed)
        .filter_person() // Label-based type filter with no closure
        // Extract age from each Person vertex
        .map(|person, _| {
            // Use projection to access Person methods in a type-safe way
            person
                .project::<Person<_>>() // Project to Person type
                .map(|p| p.age()) // Use age() accessor method
                .unwrap_or(0)
        })
        .collect::<BTreeSet<u8>>();

    // Print ages in ascending order (BTreeSet maintains order)
    println!("Person ages (range): {:?}", range_ages);
    // ANCHOR_END: collect_btreeset
}
// ANCHOR_END: all
