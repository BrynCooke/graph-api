use graph_api_lib::{EdgeReference, EdgeSearch, Graph, VertexReference, VertexSearch};
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
    // Filter vertices to keep only those of a specific type
    let projects = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter(|vertex, _| matches!(vertex.weight(), Vertex::Project(_)))
        .count();

    println!("Found {} project vertices", projects);
}

fn edge_example<G>(graph: &G, start_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Filter edges to find relationships created in a specific year
    let recent_connections = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .edges(EdgeSearch::scan())
        .filter(|edge, _| {
            if let Edge::Knows { since } = edge.weight() {
                *since >= 2020
            } else {
                false
            }
        })
        .count();

    println!("Found {} connections made since 2020", recent_connections);
}
