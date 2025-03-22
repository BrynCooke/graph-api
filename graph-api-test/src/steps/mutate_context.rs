use crate::{Edge, Vertex, populate_graph};
use graph_api_lib::{EdgeSearch, Graph};

// Note: Here we're simply testing that the mutate_context step exists and is called for each element
pub fn test_vertex_mutate_context<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Use mutate_context to verify the callback is run for each element
    let ctx = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .push_context(|_, _| 0)
        .mutate_context(|_, ctx| {
            // Just track that the callback was called
            **ctx += 1;
        })
        .map(|_, ctx| *ctx)
        .next()
        .expect("Expected to process at least one vertex");

    assert_eq!(ctx, 1, "Should have processed one vertex");
}

pub fn test_edge_mutate_context<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Use mutate_context to verify the callback is run for each element
    let ctx = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .push_context(|_, _| 0)
        .edges(EdgeSearch::scan().outgoing())
        .mutate_context(|_, ctx| {
            // Just track that the callback was called
            **ctx += 1;
        })
        .map(|_, ctx| *ctx)
        .last()
        .expect("Expected to process at least one vertex");

    // Verify we processed at least one element
    assert_eq!(ctx, 2, "Should have processed two edges");
}
