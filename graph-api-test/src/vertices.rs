use crate::{assert_elements_eq, populate_graph, Edge, Vertex};
use graph_api_lib::Graph;

pub fn test_head<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .out_edges(None)
        .head()
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.graph_api, refs.julia]);
}

pub fn test_tail<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .out_edges(None)
        .tail()
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}
