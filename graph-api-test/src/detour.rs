use crate::{populate_graph, Edge, Vertex};
use graph_api_lib::Graph;

pub fn test_vertices_detour<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn, refs.julia])
        .detour(|w| {
            w.out_edges(None)
                .push_default_context()
                .head()
                .push_default_context()
        })
        .push_default_context()
        .into_iter()
        .map(|(_, c)| {
            (
                *c.vertex_id(),
                *c.parent().parent().edge_id(),
                *c.parent().vertex_id(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(collected.len(), 3);
    assert!(collected.contains(&(refs.bryn, refs.bryn_knows_julia, refs.julia)));
    assert!(collected.contains(&(refs.julia, refs.julia_knows_bryn, refs.bryn)));
    assert!(collected.contains(&(refs.bryn, refs.bryn_created_graph_api, refs.graph_api)));
}
