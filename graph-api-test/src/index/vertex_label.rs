use crate::{assert_elements_eq, populate_graph, Edge, Vertex, VertexIndex};
use graph_api_lib::{Graph, Supported};

pub fn test_index<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge, SupportsVertexLabelIndex = Supported>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices(VertexIndex::person())
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn, refs.julia]);
}
