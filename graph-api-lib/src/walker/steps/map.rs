use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};
use crate::walker::steps::into_iter::{EdgeReferenceIterImpl, VertexReferenceIterImpl};

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/map.md")]
    pub fn map<R, M>(mut self, mut mapping: M) -> impl Iterator<Item=R> + 'graph
    where
        M: FnMut(Graph::VertexReference<'graph>, Walker::Context) -> R + 'graph,
        Walker: 'graph,
    {
        VertexReferenceIterImpl::new(self.graph.take(), self.walker).map(move |(reference, ctx)| mapping(reference, ctx))
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/map.md")]
    pub fn map<R, M>(mut self, mut mapping: M) -> impl Iterator<Item=R> + 'graph
    where
        M: FnMut(Graph::EdgeReference<'graph>, Walker::Context) -> R + 'graph,
        Walker: 'graph,
    {
        EdgeReferenceIterImpl::new(self.graph.take(), self.walker).map(move |(reference, ctx)| mapping(reference, ctx))
    }
}