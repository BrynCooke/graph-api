use graph_api_lib::{EdgeSearch, SupportsEdgeLabelIndex};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, Vertex, populate_graph};

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);
    example(graph, refs.bryn);
}

fn example<Graph>(graph: Graph, bryn_id: Graph::VertexId)
where
    Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
{
    // Get all edges from bryn
    let bryn_edges = graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .edges(EdgeSearch::scan())
        .collect::<Vec<_>>();

    // Bryn should have edges
    assert!(!bryn_edges.is_empty());

    // Get outgoing edges from bryn
    let bryn_outgoing_edges = graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .edges(EdgeSearch::scan().outgoing())
        .collect::<Vec<_>>();

    // Bryn should have outgoing edges
    assert!(!bryn_outgoing_edges.is_empty());

    // Get only 'Created' edges
    let bryn_created_edges = graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .edges(Edge::created())
        .collect::<Vec<_>>();

    // Bryn should have at least one `Created` edge
    assert!(!bryn_created_edges.is_empty());

    // Get outgoing 'Created' edges
    let bryn_outgoing_created_edges = graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .edges(Edge::created().outgoing())
        .collect::<Vec<_>>();

    // Bryn should have outgoing created edges
    assert!(!bryn_outgoing_created_edges.is_empty());

    // You can also use indexed edges when available
    let indexed_created_edges = graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .edges(Edge::created())
        .collect::<Vec<_>>();

    assert!(!indexed_created_edges.is_empty());
}
