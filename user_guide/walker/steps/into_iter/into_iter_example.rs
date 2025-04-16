use crate::standard_model::Vertex;
use graph_api_lib::Graph;
use graph_api_lib::{VertexReference, VertexSearch};

// ANCHOR: all
pub fn into_iter_example<G>(graph: G)
where
    G: Graph<Vertex = Vertex>,
{
    // ANCHOR: basic_usage
    // Basic iteration to collect IDs
    let vertex_ids = graph
        .walk()
        .vertices(VertexSearch::scan())
        .into_iter()
        .collect::<Vec<_>>();

    // There should be at least 4 vertices in the graph
    assert!(vertex_ids.len() >= 4);
    // ANCHOR_END: basic_usage

    // ANCHOR: filtering
    // We can use standard iterator operations like filtering
    let filtered_vertices = graph
        .walk()
        .vertices(VertexSearch::scan())
        .into_iter()
        .filter(|vertex_id| {
            // Get the vertex reference from the ID
            if let Some(vertex) = graph.vertex(*vertex_id) {
                // Check if it's a Person
                matches!(vertex.weight(), Vertex::Person { .. })
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    // There should be exactly 2 Person vertices (bryn and julia)
    assert_eq!(filtered_vertices.len(), 2);
    // ANCHOR_END: filtering

    // ANCHOR: comparison
    // Using .map() on the walker directly yields references with context
    let vertex_names = graph
        .walk()
        .vertices(VertexSearch::scan())
        .map(|vertex, _ctx| match vertex.weight() {
            Vertex::Person { name, .. } => name.clone(),
            Vertex::Project { name } => name.clone(),
            _ => "Unknown".to_string(),
        })
        .collect::<Vec<_>>();

    assert!(vertex_names.contains(&"Bryn".to_string()));
    assert!(vertex_names.contains(&"Julia".to_string()));
    assert!(vertex_names.contains(&"GraphApi".to_string()));
    assert!(vertex_names.contains(&"Rust".to_string()));
    // ANCHOR_END: comparison
}
// ANCHOR_END: all
