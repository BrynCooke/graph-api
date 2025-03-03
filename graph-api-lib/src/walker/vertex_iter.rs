use crate::graph::Graph;
use crate::walker::{ VertexWalker, Walker};
use std::marker::PhantomData;
use crate::ElementId;

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
    ) -> Option<ElementId<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
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
