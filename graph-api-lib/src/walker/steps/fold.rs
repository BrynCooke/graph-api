use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/fold.md")]
    pub fn fold<Acc, F>(mut self, init: Acc, mut f: F) -> Acc 
    where
        F: FnMut(Acc, Graph::VertexReference<'graph>, &Walker::Context) -> Acc,
        'graph: 'graph,
    {
        let graph = self.graph.take();
        let mut walker = self.walker;
        let mut acc = init;
        
        while let Some(vertex_id) = walker.next(graph) {
            let vertex = graph.vertex(vertex_id).expect("vertex ID must resolve to vertex");
            acc = f(acc, vertex, walker.ctx());
        }
        
        acc
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/fold.md")]
    pub fn fold<Acc, F>(mut self, init: Acc, mut f: F) -> Acc 
    where
        F: FnMut(Acc, Graph::EdgeReference<'graph>, &Walker::Context) -> Acc,
        'graph: 'graph,
    {
        let graph = self.graph.take();
        let mut walker = self.walker;
        let mut acc = init;
        
        while let Some(edge_id) = walker.next(graph) {
            let edge = graph.edge(edge_id).expect("edge ID must resolve to edge");
            acc = f(acc, edge, walker.ctx());
        }
        
        acc
    }
}