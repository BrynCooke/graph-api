use crate::standard_model::{Vertex, VertexExt, standard_populated_graph};
use graph_api_lib::{Graph, VertexSearch};

// ANCHOR: model_definition
#[derive(VertexExt)]
pub enum IndexedVertex {
    // Person vertex with various properties
    Person {
        name: String, // Not indexed

        #[index(full_text)] // Full-text index for text search
        biography: String,
    },
}
// ANCHOR_END: model_definition

// ANCHOR: all
// Example demonstrating full-text index search capabilities
pub fn full_text_index_example() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: full_text_queries
    // Find people with "developer" in their biography
    let developers = graph
        .walk()
        .vertices(Vertex::person_by_biography("developer"))
        .collect::<Vec<_>>();

    println!("Found {} people who are developers", developers.len());
    // ANCHOR_END: full_text_queries

    // ANCHOR: full_text_vs_scan
    // Comparison: Full-text index vs. manual string search

    // 1. Using full-text index (efficient)
    let search_term = "network";
    let by_index = graph
        .walk()
        .vertices(Vertex::person_by_biography(search_term))
        .collect::<Vec<_>>();

    // 2. Using scan (inefficient)
    let by_scan = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_by_person(|person, _| {
            person
                .biography()
                .to_lowercase()
                .contains(&search_term.to_lowercase())
        })
        .collect::<Vec<_>>();

    // Both approaches find matching vertices
    assert_eq!(by_index.len(), by_scan.len());
    // But the full-text index is much more efficient for large graphs
    // and provides additional features like relevance ranking
    // ANCHOR_END: full_text_vs_scan
}
// ANCHOR_END: all
