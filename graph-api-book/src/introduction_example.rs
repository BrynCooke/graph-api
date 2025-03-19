use crate::standard_model::{Edge, Vertex, standard_populated_graph};
use graph_api_lib::Graph;

/* ANCHOR: all */
// ANCHOR: basic_example
// Basic example showing key Graph API features
pub fn basic_example() {
    // Use the standard graph model from standard_model.rs
    let graph = standard_populated_graph();

    // Find a person using the username index (exact match)
    let alice = graph
        .walk()
        .vertices(VertexIndex::person_by_username("alice123"))
        .first()
        .expect("Alice should exist in the graph");

    println!("Found person: {:?}", alice);

    // Find projects created by a person
    let projects_created_by_alice = graph
        .walk()
        .vertices_by_id(vec![alice.id()])
        .edges(EdgeIndex::created().outgoing())
        .vertices()
        .collect::<Vec<_>>();

    println!("Alice created {} projects", projects_created_by_alice.len());

    // Find all people followed by Alice
    let followed_by_alice = graph
        .walk()
        .vertices_by_id(vec![alice.id()])
        .edges(EdgeIndex::follows().outgoing())
        .vertices()
        .collect::<Vec<_>>();

    println!("Alice follows {} people", followed_by_alice.len());

    // Find all people with "graph" in their biography
    let graph_enthusiasts = graph
        .walk()
        .vertices(VertexIndex::person_by_biography_containing("graph"))
        .collect::<Vec<_>>();

    println!("Found {} graph enthusiasts", graph_enthusiasts.len());

    // Find people in a specific age range
    let people_in_30s = graph
        .walk()
        .vertices(VertexIndex::person_by_age_range(30..40))
        .collect::<Vec<_>>();

    println!("Found {} people in their 30s", people_in_30s.len());
}
// ANCHOR_END: basic_example
/* ANCHOR_END: all */
