use crate::{populate_graph, Edge, Vertex};
use graph_api_lib::{EdgeReference, EdgeSearch, Graph};

pub fn test_vertices_first<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    assert_eq!(
        graph
            .walk_mut()
            .vertices_by_id(vec![refs.bryn, refs.julia])
            .first(),
        Some(refs.bryn)
    );
}

pub fn test_edges_first<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    assert_eq!(
        graph
            .walk_mut()
            .vertices_by_id(vec![refs.julia, refs.graph_api])
            .edges(EdgeSearch::scan().incoming())
            .first()
            .map(|e| e.id()),
        Some(refs.bryn_knows_julia)
    );
}
