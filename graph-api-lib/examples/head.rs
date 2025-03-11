use graph_api_lib::{EdgeSearch, Graph, Supported};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, EdgeIndex, Vertex, VertexIndex, populate_graph};

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);
    example(graph, refs.graph_api);
}

fn example<G>(graph: G, graph_api_id: G::VertexId)
where
    G: Graph<
            Vertex = Vertex,
            Edge = Edge,
            SupportsVertexLabelIndex = Supported,
            SupportsEdgeLabelIndex = Supported,
        >,
{
    // Find all people who created projects
    // The head() step returns the source vertices of edges
    let creators = graph
        .walk()
        .vertices(VertexIndex::project())
        .edges(EdgeIndex::created().incoming())
        .head()
        .collect::<Vec<_>>();

    // There should be at least one creator
    assert!(!creators.is_empty());

    // Find vertices connected to graph_api project
    let connected_to_graph_api = graph
        .walk()
        .vertices_by_id(vec![graph_api_id])
        .edges(EdgeSearch::scan().incoming())
        .head()
        .collect::<Vec<_>>();

    // There should be at least one vertex connected to graph_api
    assert!(!connected_to_graph_api.is_empty());
}
