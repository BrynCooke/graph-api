use crate::{populate_graph, Edge, Vertex};
use graph_api_lib::Graph;

pub fn test_vertices_count<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    assert_eq!(
        graph
            .walk_mut()
            .vertices_by_id(vec![refs.bryn])
            .out_edges(None)
            .head()
            .count(),
        2
    );
}

pub fn test_edges_count<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    assert_eq!(
        graph
            .walk_mut()
            .vertices_by_id(vec![refs.bryn])
            .out_edges(None)
            .count(),
        2
    );
}
