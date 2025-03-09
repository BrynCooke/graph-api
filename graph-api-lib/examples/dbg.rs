use graph_api_lib::Graph;
use graph_api_lib::VertexSearch;
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::populate_graph;
use graph_api_test::Vertex;

fn main() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _ = populate_graph(&mut graph);

    basic_example(graph);
}

fn basic_example<G>(graph: G)
where
    G: Graph,
{
    // Debug vertices in a traversal (this will print to console)
    let vertex_count = graph
        .walk()
        .vertices(VertexSearch::scan())
        .dbg("vertices") // Print vertices to console with tag "vertices"
        .count();

    // There should be at least 4 vertices in the graph
    assert!(vertex_count >= 4);
    println!("Found {} vertices in total", vertex_count);
}
