use crate::standard_model::{Edge, Vertex, standard_populated_graph};
use graph_api_lib::Graph;

// ANCHOR: all
// ANCHOR: define_range_index
// Function explaining range indexes
pub fn define_range_index() {
    // Range indexes enable efficient range queries on numeric or comparable types
    // They are defined using the #[index(range)] attribute

    // In the Vertex enum from standard_model.rs:
    // - Person::age has an range index for range queries
    //   This allows efficient queries like "find all people between 25 and 35"

    // Range indexes are ideal for:
    // - Numeric ranges (age, price, dates)
    // - Time-based queries
    // - Any property where "less than" or "greater than" comparisons are meaningful
}
// ANCHOR_END: define_range_index

// ANCHOR: range_index_queries
// Example of querying with range indexes
pub fn range_index_queries() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // Example 1: Find young adults (25-30)
    let young_adults = graph
        .walk()
        .vertices(VertexIndex::person_by_age_range(25..31))
        .collect::<Vec<_>>();

    println!("Found {} people aged 25-30", young_adults.len());

    // Example 2: Find people in their 30s
    let thirties = graph
        .walk()
        .vertices(VertexIndex::person_by_age_range(30..40))
        .collect::<Vec<_>>();

    println!("Found {} people in their 30s", thirties.len());

    // Example 3: Find people who are 34 or older
    let older_folks = graph
        .walk()
        .vertices(VertexIndex::person_by_age_range(34..))
        .collect::<Vec<_>>();

    println!("Found {} people aged 34 or older", older_folks.len());

    // Example 4: Find people younger than 30
    let under_thirty = graph
        .walk()
        .vertices(VertexIndex::person_by_age_range(..30))
        .collect::<Vec<_>>();

    println!("Found {} people under 30", under_thirty.len());

    // Example 5: Find people aged exactly 31
    // Range indexes also support exact value queries
    let exactly_31 = graph
        .walk()
        .vertices(VertexIndex::person_by_age(31))
        .collect::<Vec<_>>();

    println!("Found {} people who are exactly 31", exactly_31.len());
}
// ANCHOR_END: range_index_queries
// ANCHOR_END: all
