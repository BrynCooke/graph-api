use graph_api_lib::{Graph, VertexReference};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{populate_graph, Vertex};

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);
    example(graph, refs.bryn, refs.julia);
}

fn example<G>(graph: G, bryn_id: G::VertexId, julia_id: G::VertexId)
where
    G: Graph<Vertex = Vertex>,
{
    // The map() step transforms elements into a new value
    // Here we extract names from Person vertices
    let names: Vec<String> = graph
        .walk()
        .vertices_by_id(vec![bryn_id, julia_id])
        .map(|vertex, _ctx| match vertex.weight() {
            Vertex::Person { name, .. } => name.clone(),
            _ => "Unknown".to_string(),
        })
        .collect();

    // Should have mapped to person names
    assert!(!names.is_empty());
    assert!(names.contains(&"Bryn".to_string()));
}
