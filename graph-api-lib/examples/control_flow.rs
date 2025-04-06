use graph_api_lib::{EdgeReference, EdgeSearch, Graph, VertexReference, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, Vertex, populate_graph};
use std::ops::ControlFlow;

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
    // Use control_flow to either skip a vertex (None), include it (Some), or stop traversal (Break)
    let project = graph
        .walk()
        .vertices(VertexSearch::scan())
        .control_flow(|vertex, _| {
            if let Vertex::Project(project) = vertex.weight() {
                // If we find a project with "Graph" in the name, stop traversal
                if project.name.contains("Graph") {
                    return ControlFlow::Break(Some(vertex));
                }
                // Include other project vertices
                return ControlFlow::Continue(Some(vertex));
            }
            // Skip non-project vertices
            ControlFlow::Continue(None)
        })
        .first();

    match project {
        Some(id) => println!("Found project with 'Graph' in the name: {:?}", id),
        None => println!("No projects with 'Graph' in the name"),
    }
}

fn edge_example<G>(graph: &G, start_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Use control_flow to skip edges (None), include them (Some), or stop traversal (Break)
    let early_connection = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .edges(EdgeSearch::scan())
        .control_flow(|edge, _| {
            if let Edge::Knows { since } = edge.weight() {
                // If we found a connection from before 2010, stop traversal
                if *since < 2010 {
                    println!(
                        "Found a very old connection ({}), stopping traversal",
                        since
                    );
                    return ControlFlow::Break(Some(edge));
                }
                // Include 'knows' edges
                return ControlFlow::Continue(Some(edge));
            }
            // Skip non-'knows' edges
            ControlFlow::Continue(None)
        })
        .first();

    match early_connection {
        Some(id) => println!("Old connection {:?}", id),
        None => println!("No connections found"),
    }
}
