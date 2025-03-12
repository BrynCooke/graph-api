use crate::{Edge, Vertex, assert_elements_eq, populate_graph};
use graph_api_lib::{EdgeSearch, Graph};

pub fn test_edges_collect<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan().outgoing())
        .collect::<Vec<_>>();
    assert_elements_eq!(
        graph,
        collected,
        vec![refs.bryn_knows_julia, refs.bryn_created_graph_api]
    );
}

pub fn test_vertices_collect<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn, refs.julia])
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn, refs.julia]);
}
