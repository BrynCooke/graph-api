use graph_api_lib::{EdgeSearch, Graph, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, Vertex, populate_graph};

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
    // Get at most 2 vertices from the graph
    let vertices = graph
        .walk()
        .vertices(VertexSearch::scan())
        .limit(2) // Only process two vertices, regardless of how many exist
        .collect::<Vec<_>>();

    // Verify we got at most 2 vertices
    assert!(vertices.len() <= 2);
    println!("Retrieved {} vertices (limited to 2)", vertices.len());
}

fn edge_example<G>(graph: &G, start_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Get at most 3 edges connected to a specific vertex
    let connections = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .edges(EdgeSearch::scan())
        .limit(3) // Only process three edges, even if there are more
        .collect::<Vec<_>>();

    // Verify we got at most 3 edges
    println!("Retrieved {} connections (limited to 3)", connections.len());
}
