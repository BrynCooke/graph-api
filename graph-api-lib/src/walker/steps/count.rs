use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    //#[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/count.md")]
    pub fn count(mut self) -> usize
    where
        'graph: 'graph,
    {
        let mut count = 0;
        let graph = self.graph.take();
        let mut walker = self.walker;
        while let Some(_vertex_id) = walker.next(graph) {
            count += 1;
        }
        count
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    //#[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/count.md")]
    pub fn count(mut self) -> usize
    where
        'graph: 'graph,
    {
        let mut count = 0;
        let graph = self.graph.take();
        let mut walker = self.walker;
        while let Some(_vertex_id) = walker.next(graph) {
            count += 1;
        }
        count
    }
}