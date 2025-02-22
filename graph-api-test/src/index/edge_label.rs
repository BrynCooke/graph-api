use crate::{assert_elements_eq, populate_graph, Edge, EdgeIndex, Vertex};
use graph_api_lib::{Graph, Supported};

pub fn test_index<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge, SupportsEdgeLabelIndex = Supported>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn])
        .edges(EdgeIndex::knows().outgoing())
        .collect::<Vec<_>>();

    assert_elements_eq!(graph, collected, vec![refs.bryn_knows_julia])
}

pub fn test_index_limit<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge, SupportsEdgeLabelIndex = Supported>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn])
        .edges(EdgeIndex::knows().outgoing().limit(1))
        .collect::<Vec<_>>();

    assert_elements_eq!(graph, collected, vec![refs.bryn_knows_julia])
}
