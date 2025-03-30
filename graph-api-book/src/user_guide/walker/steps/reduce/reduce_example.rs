// ANCHOR: all
use graph_api_lib::{Graph, VertexReference, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Person, VertexExt, populate_graph};

pub fn reduce_example() {
    // Create a graph and populate it with test data
    let mut graph = SimpleGraph::new();
    let _refs = populate_graph(&mut graph);

    // Find the oldest person in the graph using reduce
    let oldest = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .reduce(|acc, vertex, _ctx| {
            let acc_age = acc.project::<Person<_>>().unwrap().age();
            let vertex_age = vertex.project::<Person<_>>().unwrap().age();

            // Return the person with higher age
            if vertex_age > acc_age { vertex } else { acc }
        })
        .map(|vertex, _ctx| {
            let age = vertex.project::<Person<_>>().unwrap().age();
            format!("The oldest person is {:?}, age {}", vertex.id(), age)
        })
        .next()
        .expect("Should find at least one person");

    println!("{}", oldest);
}
// ANCHOR_END: all
