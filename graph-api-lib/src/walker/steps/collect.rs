use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/collect.md")]
    pub fn collect<T: FromIterator<Graph::VertexId>>(self) -> T
    where
        Walker: VertexWalker<'graph>,
    {
        self.into_iter().collect()
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    pub fn collect<T: FromIterator<Graph::EdgeId>>(self) -> T
    where
        Walker: EdgeWalker<'graph>,
    {
        self.into_iter().collect()
    }
}