use crate::{assert_elements_eq, populate_graph, Edge, Vertex};
use graph_api_lib::{EdgeSearch, Graph};

pub fn test_vertices_context<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .push_context(|_vertex, _ctx| "hi".to_string())
        .push_context(|_vertex, ctx| assert_eq!(ctx.as_str(), "hi"))
        .collect::<Vec<_>>();

    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}

pub fn test_edges_context<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .push_context(|_edge, _ctx| "hi".to_string())
        .push_context(|_edge, ctx| assert_eq!(ctx.as_str(), "hi"))
        .collect::<Vec<_>>();

    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}
