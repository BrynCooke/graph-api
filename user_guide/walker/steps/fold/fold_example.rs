use crate::standard_model::{Person, VertexExt, standard_populated_graph};
use graph_api_lib::{Graph, VertexReference, VertexSearch};

// ANCHOR: all
pub fn fold_example() {
    // Create a graph with standard test data
    let graph = standard_populated_graph();

    // Calculate the sum of ages of all people using fold
    let total_age = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .fold(0, |acc, vertex, _ctx| {
            // Add the person's age to the accumulator
            if let Some(person) = vertex.project::<Person<_>>() {
                acc + person.age() as u32
            } else {
                acc
            }
        });

    println!("Total age of all people: {}", total_age);

    // Example with context: Collect names of people older than the context age
    let initial_age_threshold = 30;
    let names_older_than_threshold = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .push_context(|_, _| initial_age_threshold) // Push the threshold as context
        .fold(Vec::new(), |mut names, vertex, ctx| {
            if let Some(person) = vertex.project::<Person<_>>() {
                // Use context (threshold) in the fold logic
                if person.age() as u32 > **ctx {
                    names.push(person.name().to_string());
                }
            }
            names
        });

    println!(
        "Names of people older than {}: {:?}",
        initial_age_threshold, names_older_than_threshold
    );
}
// ANCHOR_END: all
