use crate::graph::Graph;
use std::collections::HashSet;

pub trait FromVertexWalker<'graph, VertexWalker>
where
    VertexWalker: crate::walker::VertexWalker<'graph>,
{
    fn from_vertex_walker(vertex_walker: VertexWalker, graph: &'graph VertexWalker::Graph) -> Self;
}

impl<'graph, VertexWalker> FromVertexWalker<'graph, VertexWalker>
    for Vec<<VertexWalker::Graph as Graph>::VertexId>
where
    VertexWalker: crate::walker::VertexWalker<'graph>,
{
    fn from_vertex_walker(
        mut vertex_walker: VertexWalker,
        graph: &'graph VertexWalker::Graph,
    ) -> Self {
        let mut vec = Vec::new();
        while let Some(vertex_id) = vertex_walker.next(graph) {
            vec.push(vertex_id);
        }
        vec
    }
}

impl<'graph, VertexWalker> FromVertexWalker<'graph, VertexWalker>
    for HashSet<<VertexWalker::Graph as Graph>::VertexId>
where
    VertexWalker: crate::walker::VertexWalker<'graph>,
{
    fn from_vertex_walker(
        mut vertex_walker: VertexWalker,
        graph: &'graph VertexWalker::Graph,
    ) -> Self {
        let mut hash_set = HashSet::new();
        while let Some(vertex_id) = vertex_walker.next(graph) {
            hash_set.insert(vertex_id);
        }
        hash_set
    }
}
