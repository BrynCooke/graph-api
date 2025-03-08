use crate::walker::builder::VertexWalkerBuilder;
use crate::walker::{VertexWalker, Walker};
use crate::graph::Graph;
use crate::ElementId;
use std::marker::PhantomData;

// ================ VERTEX_ITER IMPLEMENTATION ================

pub struct VertexIter<'graph, Parent, Iter>
where
    Parent: VertexWalker<'graph>,
    Iter: Iterator<Item = <Parent::Graph as Graph>::VertexId>,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    start: Iter,
}

impl<'graph, Parent, Iter> VertexIter<'graph, Parent, Iter>
where
    Parent: VertexWalker<'graph>,
    Iter: Iterator<Item = <Parent::Graph as Graph>::VertexId>,
{
    pub fn new(parent: Parent, start: Iter) -> Self {
        Self {
            _phantom_data: Default::default(),
            parent,
            start,
        }
    }
}

impl<'graph, Parent, Iter> Walker<'graph> for VertexIter<'graph, Parent, Iter>
where
    Parent: VertexWalker<'graph>,
    Iter: Iterator<Item = <Parent::Graph as Graph>::VertexId>,
{
    type Graph = Parent::Graph;

    type Context = Parent::Context;
    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }
    fn ctx(&self) -> &Parent::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent, Iter> VertexWalker<'graph> for VertexIter<'graph, Parent, Iter>
where
    Parent: VertexWalker<'graph>,
    Iter: Iterator<Item = <Parent::Graph as Graph>::VertexId>,
{
    fn next(&mut self, _graph: &Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        self.start.next()
    }
}

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/vertices_by_id.md")]
    pub fn vertices_by_id<Iter>(
        self,
        vertex_ids: Iter,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexIter<'graph, Walker, Iter::IntoIter>>
    where
        Iter: IntoIterator<Item = Graph::VertexId>,
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.vertices_by_id(vertex_ids),
            graph: self.graph,
        }
    }
}