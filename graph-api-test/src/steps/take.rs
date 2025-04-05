use crate::{Edge, Vertex, populate_graph};
use graph_api_lib::{EdgeSearch, Graph};

pub fn test_vertices_take<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    assert_eq!(
        graph
            .walk_mut()
            .vertices_by_id(vec![refs.bryn, refs.julia])
            .take(1)
            .count(),
        1
    );
}

pub fn test_edges_take<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    assert_eq!(
        graph
            .walk_mut()
            .vertices_by_id(vec![refs.bryn])
            .edges(EdgeSearch::scan().outgoing())
            .take(1)
            .count(),
        1
    );
}
