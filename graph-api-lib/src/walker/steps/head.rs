use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::steps::Endpoints;
use crate::walker::EdgeWalker;

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/head.md")]
    pub fn head(self) -> VertexWalkerBuilder<'graph, Mutability, Graph, Endpoints<'graph, Walker>> {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: Endpoints::new(self.walker, crate::walker::steps::End::Head),
            graph: self.graph,
        }
    }
}