use crate::standard_model::{Vertex, VertexExt, standard_populated_graph};
use graph_api_lib::{Graph, VertexSearch};
// ANCHOR: model_definition
#[derive(VertexExt)]
pub enum IndexedVertex {
    // Person vertex with various properties
    Person {
        name: String, // Not indexed

        #[index(range)] // Range index for range lookups
        age: u8,
    },
}
// ANCHOR_END: model_definition

// ANCHOR: all
// Example demonstrating range index search capabilities
pub fn range_index_example() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: range_index_queries
    // Find people within a specific age range (29-35)
    let age_range_results = graph
        .walk()
        .vertices(Vertex::person_by_age_range(29..36))
        .collect::<Vec<_>>();
    println!("Found {} people aged 29-35", age_range_results.len());

    // ANCHOR_END: range_index_queries

    // ANCHOR: range_vs_scan
    // Comparison: Range index vs. full scan

    // 1. Using range index (efficient)
    let age_range = 25..35;
    let by_index = graph
        .walk()
        .vertices(Vertex::person_by_age_range(age_range.clone()))
        .collect::<Vec<_>>();

    // 2. Using scan (inefficient)
    let by_scan = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_by_person(|person, _| age_range.contains(&person.age()))
        .collect::<Vec<_>>();

    // Both approaches find the same vertices
    assert_eq!(by_index.len(), by_scan.len());
    // But the range index is much more efficient for large graphs
    // ANCHOR_END: range_vs_scan
}
// ANCHOR_END: all
