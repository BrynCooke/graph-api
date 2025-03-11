use graph_api_lib::{Graph, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::populate_graph;

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);
    example(graph)
}

fn example(graph: impl Graph) {
    // Collect vertex IDs into a vector
    let vertex_ids = graph
        .walk()
        .vertices(VertexSearch::scan())
        .collect::<Vec<_>>();

    // Graph should have vertices
    assert!(!vertex_ids.is_empty());
}
