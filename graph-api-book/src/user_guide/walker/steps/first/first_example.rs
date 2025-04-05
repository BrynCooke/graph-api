use crate::standard_model::{Vertex, VertexExt, standard_populated_graph};
use graph_api_lib::Graph;

// ANCHOR: all
// Function demonstrating the first step
pub fn first_example() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: basic_usage
    // Get the first person in the graph (if any)
    let first_person = graph.walk().vertices(Vertex::person()).first();

    match first_person {
        Some(person) => println!("Found a person: {:?}", person),
        None => println!("No people in the graph"),
    }
    // ANCHOR_END: basic_usage

    // ANCHOR: with_filter
    // Get the first person with a specific name
    let first_bryn = graph
        .walk()
        .vertices(Vertex::person()) // Get all Person vertices
        .filter_by_person(|person, _| {
            // Using the typed projection with accessor methods
            person.name().contains("Bryn")
        })
        .first();

    match first_bryn {
        Some(bryn) => println!("Found Bryn: {:?}", bryn),
        None => println!("No one named Bryn in the graph"),
    }
    // ANCHOR_END: with_filter

    // ANCHOR: existence_check
    // Use first to check if any element matches a condition
    let has_young_person = graph
        .walk()
        .vertices(Vertex::person())
        .filter_by_person(|person, _| {
            // Using type-safe accessor methods
            person.age() < 30
        })
        .first()
        .is_some();

    println!(
        "Graph {} people under 30",
        if has_young_person {
            "contains"
        } else {
            "doesn't contain"
        }
    );
    // ANCHOR_END: existence_check
}
// ANCHOR_END: all
