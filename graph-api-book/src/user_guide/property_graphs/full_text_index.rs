use crate::standard_model::{VertexIndex, standard_populated_graph};
use graph_api_lib::Graph;

// ANCHOR: all
// ANCHOR: define_full_text_index
// Function explaining full-text indexes
pub fn define_full_text_index() {
    // Full-text indexes enable text search capabilities for string properties
    // They are defined using the #[index(full_text)] attribute

    // In the Vertex enum from standard_model.rs:
    // - Person::biography has a full-text index for text search
    //   This allows for partial matches and fuzzy search

    // Full-text indexes are ideal for:
    // - Long text fields (descriptions, biographies, articles)
    // - Search by keyword functionality
    // - Natural language search
}
// ANCHOR_END: define_full_text_index

// Example demonstrating full-text search capabilities
pub fn full_text_queries() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: full_text_queries
    // Find people with "developer" in their biography
    let developers = graph
        .walk()
        .vertices(VertexIndex::person_by_biography("developer"))
        .collect::<Vec<_>>();

    println!("Found {} people who are developers", developers.len());

    // Find people with "graph" in their biography
    let graph_enthusiasts = graph
        .walk()
        .vertices(VertexIndex::person_by_biography("graph"))
        .collect::<Vec<_>>();

    println!(
        "Found {} people interested in graphs",
        graph_enthusiasts.len()
    );

    // Find people with "network" in their biography
    let network_specialists = graph
        .walk()
        .vertices(VertexIndex::person_by_biography("network"))
        .collect::<Vec<_>>();

    println!("Found {} network specialists", network_specialists.len());

    // Full-text search is more flexible than exact matching
    // It can find partial matches, handle stemming, and more
    // For example, searching for "develop" would also match "developer"
    let develop_related = graph
        .walk()
        .vertices(VertexIndex::person_by_biography("develop"))
        .collect::<Vec<_>>();

    println!(
        "Found {} people with 'develop' in their bio",
        develop_related.len()
    );
    // ANCHOR_END: full_text_queries
}
// ANCHOR_END: all
