use graph_api_lib::Graph;
use graph_api_lib::{VertexSearch, VertexReference};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{populate_graph, Vertex};

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);
    example(graph);
}

fn example<G>(graph: G)
where
    G: Graph<Vertex = Vertex>
{
    // Basic iteration to collect IDs
    let vertex_ids = graph
        .walk()
        .vertices(VertexSearch::scan())
        .into_iter()
        .collect::<Vec<_>>();

    // There should be at least 4 vertices in the graph
    assert!(vertex_ids.len() >= 4);
    
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
    
    // Using .map() on the walker directly yields references with context
    let vertex_names = graph
        .walk()
        .vertices(VertexSearch::scan())
        .map(|vertex, _ctx| {
            match vertex.weight() {
                Vertex::Person { name, .. } => name.clone(),
                Vertex::Project(project) => project.name.clone(),
                // Use a regular string instead of tech.clone() since Technology is not a variant
                _ => "Unknown".to_string(),
            }
        })
        .collect::<Vec<_>>();
        
    assert!(vertex_names.contains(&"Bryn".to_string()));
    assert!(vertex_names.contains(&"Julia".to_string()));
    assert!(vertex_names.contains(&"GraphApi".to_string()));
    assert!(vertex_names.contains(&"Rust".to_string()));
}