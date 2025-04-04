use graph_api_lib::{EdgeSearch, Graph, SupportsEdgeLabelIndex};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, Vertex, populate_graph};

fn main() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with some test data
    let refs = populate_graph(&mut graph);

    // Run both examples
    vertex_mutate_context_example(&graph, refs.bryn);
    edge_mutate_context_example(&graph, refs.bryn);
}

fn vertex_mutate_context_example<G>(graph: &G, start_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
{
    // Use mutate_context to track each vertex we've seen
    // Here we're using context as a way to observe the traversal
    let ctx = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .push_context(|_, _| 0)
        .mutate_context(|_, ctx| {
            // Update the current context
            **ctx += 1;
        })
        .map(|_, ctx| *ctx)
        .next()
        .expect("Expected to process one vertex");

    println!("Visited {} vertices", ctx);
    assert_eq!(ctx, 1, "Should have visited at least one vertex");
}

fn edge_mutate_context_example<G>(graph: &G, start_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
{
    // Use mutate_context to track each edge in the traversal
    let ctx = graph
        .walk()
        .vertices_by_id(vec![start_id])
        .push_context(|_, _| 0)
        .edges(EdgeSearch::scan().outgoing())
        .mutate_context(move |_, ctx| {
            // Count each edge we encounter
            **ctx += 1;
        })
        .map(|_, ctx| *ctx)
        .last()
        .expect("Expected to process at least one edge");

    println!("Total edges traversed: {}", ctx);
    assert_eq!(ctx, 2, "Should have traversed two edges");
}
