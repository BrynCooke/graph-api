use crate::standard_model::{Edge, Vertex, standard_populated_graph};
use graph_api_lib::Graph;

// ANCHOR: all
// Basic example showing key Graph API features
pub fn basic_example() {
    // Use the standard graph model from standard_model.rs
    let graph = standard_populated_graph();
    // ANCHOR: basic_example
    // Find a person using the username index (exact match)
    let bryn = graph
        .walk()
        .vertices(Vertex::person_by_username("bryn123"))
        .first()
        .expect("Bryn should exist in the graph");

    println!("Found person: {:?}", bryn);

    // Find projects created by a person
    let projects_created_by_bryn = graph
        .walk()
        .vertices_by_id(vec![bryn])
        .edges(Edge::created().outgoing())
        .head()
        .collect::<Vec<_>>();

    println!("Bryn created {} projects", projects_created_by_bryn.len());

    // Find all people followed by Bryn
    let followed_by_bryn = graph
        .walk()
        .vertices_by_id(vec![bryn])
        .edges(Edge::follows().outgoing())
        .head()
        .collect::<Vec<_>>();

    println!("Bryn follows {} people", followed_by_bryn.len());

    // Find all people with "graph" in their biography
    let graph_enthusiasts = graph
        .walk()
        .vertices(Vertex::person_by_biography("graph"))
        .collect::<Vec<_>>();

    println!("Found {} graph enthusiasts", graph_enthusiasts.len());

    // Find people in a specific age range
    let people_in_30s = graph
        .walk()
        .vertices(Vertex::person_by_age_range(30..40))
        .collect::<Vec<_>>();

    println!("Found {} people in their 30s", people_in_30s.len());
    // ANCHOR_END: basic_example
}
// ANCHOR_END: all
