use graph_api_lib::Graph;
use graph_api_lib::{EdgeSearch, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::populate_graph;

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);
    example(graph);
}

fn example(graph: impl Graph) {
    // Get at most 2 vertices from the graph
    let some_vertices = graph
        .walk()
        .vertices(VertexSearch::scan())
        .limit(2)
        .collect::<Vec<_>>();

    // Verify we got at most 2 vertices
    assert!(some_vertices.len() <= 2);

    // Get at most 3 edges
    let some_edges = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(EdgeSearch::scan())
        .limit(3)
        .collect::<Vec<_>>();

    assert!(some_edges.len() <= 3);
}