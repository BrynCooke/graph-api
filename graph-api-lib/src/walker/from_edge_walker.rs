use crate::graph::EdgeReference;
use crate::graph::Graph;
use std::collections::HashSet;

pub trait FromEdgeWalker<'graph, EdgeWalker>
where
    EdgeWalker: crate::walker::EdgeWalker<'graph>,
{
    fn from_edge_walker(edge_walker: EdgeWalker, graph: &'graph EdgeWalker::Graph) -> Self;
}

impl<'graph, EdgeWalker> FromEdgeWalker<'graph, EdgeWalker>
    for Vec<<EdgeWalker::Graph as Graph>::EdgeId>
where
    EdgeWalker: crate::walker::EdgeWalker<'graph>,
{
    fn from_edge_walker(mut edge_walker: EdgeWalker, graph: &'graph EdgeWalker::Graph) -> Self {
        let mut vec = Vec::new();
        while let Some(edge_reference) = edge_walker.next(graph) {
            vec.push(edge_reference.id());
        }
        vec
    }
}

impl<'graph, EdgeWalker> FromEdgeWalker<'graph, EdgeWalker>
    for HashSet<<EdgeWalker::Graph as Graph>::EdgeId>
where
    EdgeWalker: crate::walker::EdgeWalker<'graph>,
{
    fn from_edge_walker(mut edge_walker: EdgeWalker, graph: &'graph EdgeWalker::Graph) -> Self {
        let mut set = HashSet::new();
        while let Some(edge_reference) = edge_walker.next(graph) {
            set.insert(edge_reference.id());
        }
        set
    }
}
