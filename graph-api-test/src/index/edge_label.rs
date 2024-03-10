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
        .out_edges(EdgeIndex::knows())
        .collect::<Vec<_>>();

    assert_elements_eq!(graph, collected, vec![refs.bryn_knows_julia])
}
