use graph_api_lib::{
    EdgeSearch, SupportsEdgeLabelIndex, SupportsVertexHashIndex, SupportsVertexLabelIndex,
    VertexSearch,
};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, Vertex, VertexExt, populate_graph};

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);
    example(graph);
}

fn example<Graph>(graph: Graph)
where
    Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>
        + SupportsVertexHashIndex
        + SupportsVertexLabelIndex
        + SupportsEdgeLabelIndex,
{
    // Count all vertices in the graph
    let vertex_count = graph.walk().vertices(VertexSearch::scan()).count();

    assert!(vertex_count >= 4); // At least bryn, julia, graph_api, rust

    // Count only Person vertices
    let person_count = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .count();

    assert_eq!(person_count, 2); // bryn and julia

    // Count Project vertices
    let project_count = graph.walk().vertices(Vertex::project()).count();

    assert_eq!(project_count, 2); // graph_api and rust

    // Count edges between vertices
    let edge_count = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(EdgeSearch::scan())
        .count();

    assert!(edge_count > 0);

    // Count created edges
    let created_edge_count = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(Edge::created())
        .count();

    assert!(created_edge_count > 0);
}
