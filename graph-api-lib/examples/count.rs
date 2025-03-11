use graph_api_lib::Supported;
use graph_api_lib::{EdgeSearch, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, EdgeIndex, Vertex, VertexExt, VertexIndex, populate_graph};

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);
    example(graph);
}

fn example<Graph>(graph: Graph)
where
    Graph: graph_api_lib::Graph<
            Vertex = Vertex,
            Edge = Edge,
            SupportsVertexIndex = Supported,
            SupportsVertexLabelIndex = Supported,
            SupportsEdgeLabelIndex = Supported,
        >,
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
    let project_count = graph.walk().vertices(VertexIndex::project()).count();

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
        .edges(EdgeIndex::created())
        .count();

    assert!(created_edge_count > 0);
}
