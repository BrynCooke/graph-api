use graph_api_lib::{Graph, VertexReferenceMut};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{PersonMut, VertexIndex, populate_graph};

/* ANCHOR: all */
pub fn mutate_example() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    // ANCHOR: update_vertices
    // Update all people to increment their age by 1
    graph
        .walk_mut()
        .vertices(VertexIndex::person())
        .mutate(|graph, vertex, _| {
            let mut vertex = graph.vertex_mut(vertex).expect("Vertex not found");
            if let Some(mut person) = vertex.project_mut::<PersonMut<_, _>>() {
                let age = person.age() + 1;
                person.set_age(age);
                println!("Incremented age to {}", age);
            }
        });
    // ANCHOR_END: update_vertices
}
/* ANCHOR_END: all */
