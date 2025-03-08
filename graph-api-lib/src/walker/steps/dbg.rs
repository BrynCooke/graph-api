use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::steps::{EdgeProbe, VertexProbe};
use crate::walker::{EdgeWalker, VertexWalker};

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/dbg.md")]
    pub fn dbg(
        self,
        tag: &'static str
    ) -> VertexWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        VertexProbe<'graph, Walker, impl FnMut(&Graph::VertexReference<'_>)>,
    > {
        let callback = move |vertex: &Graph::VertexReference<'_>| {
            println!("{}: {:?}", tag, vertex);
        };
        self.probe(callback)
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/dbg.md")]
    pub fn dbg(
        self,
        tag: &'static str
    ) -> EdgeWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        EdgeProbe<'graph, Walker, impl FnMut(&Graph::EdgeReference<'_>)>,
    > {
        let callback = move |edge: &Graph::EdgeReference<'_>| {
            println!("{}: {:?}", tag, edge);
        };
        self.probe(callback)
    }
}