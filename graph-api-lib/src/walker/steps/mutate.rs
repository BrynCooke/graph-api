use crate::walker::builder::{EdgeWalkerBuilder, Mutable, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/mutate.md")]
    pub fn mutate<Callback>(mut self, callback: Callback) -> usize
    where
        Callback: Fn(&mut Walker::Graph, Graph::VertexId, &Walker::Context),
        Mutability: Mutable,
        'graph: 'graph,
    {
        let graph = self.graph.take_mut();
        let graph_copy: &Graph = unsafe { std::mem::transmute(&*graph) };
        let mut walker = self.walker;

        let mut contexts = Vec::new();
        while let Some(vertex_id) = walker.next(graph_copy) {
            let ctx = walker.ctx().clone();
            contexts.push((vertex_id, ctx));
        }

        let mut count = 0;
        for (vertex_id, ctx) in contexts {
            callback(graph, vertex_id, &ctx);
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
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/mutate.md")]
    pub fn mutate<Callback>(mut self, callback: Callback) -> usize
    where
        Callback: Fn(&mut Walker::Graph, Graph::EdgeId, &Walker::Context),
        Mutability: Mutable,
        'graph: 'graph,
    {
        let graph = self.graph.take_mut();
        let graph_copy: &Graph = unsafe { std::mem::transmute(&*graph) };
        let mut walker = self.walker;

        let mut contexts = Vec::new();
        while let Some(edge) = walker.next(graph_copy) {
            let ctx = walker.ctx().clone();
            contexts.push((edge, ctx));
        }

        let mut count = 0;
        for (edge_id, ctx) in contexts {
            callback(graph, edge_id, &ctx);
            count += 1;
        }
        count
    }
}