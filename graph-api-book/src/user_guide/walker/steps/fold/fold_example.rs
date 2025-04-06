use graph_api_lib::{Graph, VertexReference, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Person, VertexExt, populate_graph};

// ANCHOR: all
pub fn fold_example() {
    // Create a graph and populate it with test data
    let mut graph = SimpleGraph::new();
    let _refs = populate_graph(&mut graph);

    // Calculate the sum of ages of all people using fold
    let total_age = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .fold(0, |acc, vertex, _ctx| {
            // Add the person's age to the accumulator
            acc + vertex.project::<Person<_>>().unwrap().age()
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
            let person = vertex.project::<Person<_>>().unwrap();
            // Use context (threshold) in the fold logic
            if person.age() > *ctx {
                names.push(person.name().to_string());
            }
            names
        });

    println!(
        "Names of people older than {}: {:?}",
        initial_age_threshold, names_older_than_threshold
    );
}
// ANCHOR_END: all
