use crate::standard_model::{Vertex, VertexIndex, standard_populated_graph};
use graph_api_lib::{Graph, VertexReference, VertexSearch};

/* ANCHOR: all */
// ANCHOR: define_standard_index
// Function explaining standard indexes
pub fn define_standard_index() {
    // Standard indexes provide efficient lookups based on exact property values
    // They are defined using the #[index] attribute

    // In the Vertex enum from standard_model.rs:
    // - Person::username has a standard index for exact username lookups
    // - Biography uses a full-text index for fuzzy text search
    // - Age uses an ordered index for range queries

    // Properties without the #[index] attribute (e.g., Person::name, unique_id)
    // can only be searched via full scan
}
// ANCHOR_END: define_standard_index

// ANCHOR: standard_index_queries
// Example of querying with standard indexes
pub fn standard_index_queries() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // The VertexIndex enum is automatically generated from the Vertex enum
    // by the VertexExt derive macro. It provides methods for each indexed field.

    // EFFICIENT: Find a person by username (using standard index)
    // This uses the auto-generated index methods from the VertexExt derive macro
    let _alice = graph
        .walk()
        // Use the index
        .vertices(VertexIndex::person_by_username("alice123"))
        .first();

    // Also efficient: Find a person by age (using ordered index)
    let _people_34 = graph
        .walk()
        // Use the index
        .vertices(VertexIndex::person_by_age(34))
        .collect::<Vec<_>>();

    // Find people with a specific biography text (using full-text index)
    let _developers = graph
        .walk()
        // Use the full-text index
        .vertices(VertexIndex::person_by_biography("developer"))
        .collect::<Vec<_>>();

    // Note: No direct index for looking up by name
    // This would require a full scan with filtering
    let _people_named_bob = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter(|vertex, _| {
            // We need to manually check the type and fields
            // because there's no index for the name property
            if let Vertex::Person { name, .. } = vertex.weight() {
                name == "Bob"
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    // While the above scan works, it's much less efficient than using indexes
    // It will examine every vertex in the graph, rather than just
    // going directly to the relevant vertices
}
// ANCHOR_END: standard_index_queries
/* ANCHOR_END: all */
