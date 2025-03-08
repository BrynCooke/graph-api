use crate::graph::{EdgeReference, VertexReference};
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::steps::{ContextRef, DefaultEdgeContext, DefaultVertexContext, EdgeContext, VertexContext};
use crate::walker::{EdgeWalker, VertexWalker};

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/default_context.md")]
    pub fn push_default_context(
        self,
    ) -> VertexWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        VertexContext<
            'graph,
            Walker,
            impl Fn(
                &Graph::VertexReference<'_>,
                &Walker::Context,
            )
                -> ContextRef<DefaultVertexContext<Graph::VertexId, Graph::Vertex>, Walker::Context>,
            ContextRef<DefaultVertexContext<Graph::VertexId, Graph::Vertex>, Walker::Context>,
        >,
    >
    where
        Graph::Vertex: Clone + 'static,
    {
        self.push_context(|vertex, _context| DefaultVertexContext {
            vertex_id: vertex.id(),
            vertex: vertex.weight().clone(),
        })
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
    <Walker as crate::walker::Walker<'graph>>::Context: Clone + 'static,
{
    pub fn push_default_context(
        self,
    ) -> EdgeWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        EdgeContext<
            'graph,
            Walker,
            impl Fn(
                &Graph::EdgeReference<'_>,
                &Walker::Context,
            )
                -> ContextRef<DefaultEdgeContext<Graph::EdgeId, Graph::Edge>, Walker::Context>,
            ContextRef<DefaultEdgeContext<Graph::EdgeId, Graph::Edge>, Walker::Context>,
        >,
    >
    where
        Graph::Edge: Clone + 'static,
    {
        self.push_context(|edge, _context| DefaultEdgeContext {
            edge_id: edge.id(),
            edge: edge.weight().clone(),
        })
    }
}