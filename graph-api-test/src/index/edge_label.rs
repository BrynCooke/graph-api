#[cfg(feature = "edge-label-index")]
use crate::{assert_elements_eq, populate_graph};

use crate::{Edge, Vertex};
use graph_api_lib::Graph;

#[cfg(feature = "edge-label-index")]
pub fn test_index<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsEdgeLabelIndex,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn])
        .edges(Edge::knows().outgoing())
        .collect::<Vec<_>>();

    assert_elements_eq!(graph, collected, vec![refs.bryn_knows_julia])
}

#[cfg(not(feature = "edge-label-index"))]
pub fn test_index<T>(_graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
}

#[cfg(feature = "edge-label-index")]
pub fn test_index_limit<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsEdgeLabelIndex,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id([refs.bryn])
        .edges(Edge::knows().outgoing().with_limit(1))
        .collect::<Vec<_>>();

    assert_elements_eq!(graph, collected, vec![refs.bryn_knows_julia])
}

#[cfg(not(feature = "edge-label-index"))]
pub fn test_index_limit<T>(_graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
}
