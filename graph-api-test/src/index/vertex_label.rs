use crate::{Edge, Vertex, assert_elements_eq, populate_graph};
use graph_api_lib::{Graph, SupportsVertexLabelIndex};

pub fn test_index<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexLabelIndex,
{
    let refs = populate_graph(graph);
    let collected = graph.walk().vertices(Vertex::person()).collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn, refs.julia]);
}

pub fn test_index_limit<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexLabelIndex,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices(Vertex::person().with_limit(1))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}
