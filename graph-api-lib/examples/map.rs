use graph_api_lib::{EdgeReference, EdgeSearch, Graph, VertexReference, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{populate_graph, Edge, Vertex};

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);

    vertex_example(&graph);
    edge_example(&graph, refs.bryn);
}

fn vertex_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Map vertices to their names
    let project_names: Vec<String> = graph
        .walk()
        .vertices(VertexSearch::scan())
        .map(|vertex, _| {
            // Extract project names using pattern matching
            match vertex.weight() {
                Vertex::Project(project) => project.name.clone(),
                _ => "Not a project".to_string(),
            }
        })
        .filter(|name| name != "Not a project")
        .collect();

    // Print the project names
    println!("Projects in the graph:");
    for name in &project_names {
        println!("- {}", name);
    }
}

fn edge_example<G>(graph: &G, start_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Map edges to relationship information
    let relationships: Vec<String> = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .edges(EdgeSearch::scan())
        .map(|edge, _| {
            // Create a descriptive string for each relationship
            match edge.weight() {
                Edge::Knows { since } => format!("Knows since {}", since),
                Edge::Created => "Created".to_string(),
                Edge::Language(lang) => format!("Uses language {}", lang.name),
            }
        })
        .collect();

    // Print the relationships
    println!("\nRelationships from starting vertex:");
    for relationship in &relationships {
        println!("- {}", relationship);
    }
}
