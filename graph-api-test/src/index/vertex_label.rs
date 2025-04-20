#[cfg(feature = "vertex-label-index")]
use crate::{assert_elements_eq, populate_graph};

use crate::{Edge, Vertex};
use graph_api_lib::Graph;

#[cfg(feature = "vertex-label-index")]
pub fn test_index<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsVertexLabelIndex,
{
    let refs = populate_graph(graph);
    let collected = graph.walk().vertices(Vertex::person()).collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn, refs.julia]);
}

#[cfg(not(feature = "vertex-label-index"))]
pub fn test_index<T>(_graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
}

#[cfg(feature = "vertex-label-index")]
pub fn test_index_limit<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsVertexLabelIndex,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices(Vertex::person().with_limit(1))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}

#[cfg(not(feature = "vertex-label-index"))]
pub fn test_index_limit<T>(_graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
}
