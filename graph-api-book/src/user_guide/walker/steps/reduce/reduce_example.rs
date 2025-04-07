// ANCHOR: all
use graph_api_lib::{Graph, VertexReference, VertexSearch};
use crate::standard_model::{standard_populated_graph, Person, VertexExt};

pub fn reduce_example() {
    // Create a graph with standard test data
    let graph = standard_populated_graph();

    // Find the oldest person in the graph using reduce
    let oldest = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .reduce(|acc, vertex, _ctx| {
            let acc_age = if let Some(person) = acc.project::<Person<_>>() { person.age() } else { 0 };
            let vertex_age = if let Some(person) = vertex.project::<Person<_>>() { person.age() } else { 0 };

            // Return the person with higher age
            if vertex_age > acc_age { vertex } else { acc }
        })
        .map(|vertex, _ctx| {
            if let Some(person) = vertex.project::<Person<_>>() {
                format!("The oldest person is {:?}, age {}", vertex.id(), person.age())
            } else {
                format!("Unexpected non-person vertex: {:?}", vertex.id())
            }
        })
        .next()
        .expect("Should find at least one person");

    println!("{}", oldest);
}
// ANCHOR_END: all
