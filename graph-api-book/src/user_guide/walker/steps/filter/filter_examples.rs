use crate::standard_model::{Person, Vertex, VertexExt, standard_populated_graph};
use graph_api_lib::{Graph, VertexReference};

// ANCHOR: all
// Function demonstrating various ways to use the filter step
pub fn filter_examples() {
    // Create a graph with standard test data
    let graph = standard_populated_graph();

    // ANCHOR: basic_filter
    // Basic filter using a closure
    let adult_people = graph
        .walk()
        .vertices(Vertex::person())
        .filter_by_person(|person, _| person.age() >= 18)
        .collect::<Vec<_>>();

    println!("Found {} adults", adult_people.len());
    // ANCHOR_END: basic_filter

    // ANCHOR: type_specific_filter
    // Use the type-specific filter methods generated by the VertexExt derive macro
    let people_named_b = graph
        .walk()
        .vertices(Vertex::person())
        .filter_by_person(|person, _| {
            // This closure gets a strongly-typed view of the Person data
            person.name().starts_with('B')
        })
        .collect::<Vec<_>>();

    println!(
        "Found {} people whose names start with B",
        people_named_b.len()
    );
    // ANCHOR_END: type_specific_filter

    // ANCHOR: chained_filters
    // Chain multiple filters for complex conditions
    let specific_people = graph
        .walk()
        .vertices(Vertex::person())
        // First filter: age range
        .filter_by_person(|person, _| person.age() > 25 && person.age() < 40)
        // Second filter: name contains 'y'
        .filter_by_person(|person, _| person.name().contains('y'))
        .collect::<Vec<_>>();

    println!(
        "Found {} people aged 26-39 with 'y' in their name",
        specific_people.len()
    );
    // ANCHOR_END: chained_filters

    // ANCHOR: context_filter
    // Use filter with context
    let result = graph
        .walk()
        .vertices(Vertex::person())
        .push_context(|v, _| {
            // Store original vertex in context
            if let Some(person) = v.project::<Person<_>>() {
                person.name().to_string()
            } else {
                String::new()
            }
        })
        .filter(|_, ctx| {
            // Filter based on context
            ctx.len() > 3
        })
        .collect::<Vec<_>>();

    println!(
        "Found {} people with names longer than 3 characters",
        result.len()
    );
    // ANCHOR_END: context_filter
}
// ANCHOR_END: all
