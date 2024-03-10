use crate::graph::{EdgeReference, Graph};
use crate::walker::{EdgeWalker, Element, VertexWalker, Walker};
use std::marker::PhantomData;

pub struct VertexLimit<'graph, Parent> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    limit: usize,
}

impl<Parent> VertexLimit<'_, Parent> {
    pub fn new(parent: Parent, limit: usize) -> Self {
        Self {
            _phantom_data: Default::default(),
            parent,
            limit,
        }
    }
}

impl<'graph, Parent> Walker<'graph> for VertexLimit<'graph, Parent>
where
    Parent: VertexWalker<'graph>,
{
    type Graph = Parent::Graph;

    type Context = Parent::Context;
    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<Element<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
        self.next(graph).map(Element::Vertex)
    }

    fn ctx(&self) -> &Parent::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent> VertexWalker<'graph> for VertexLimit<'graph, Parent>
where
    Parent: VertexWalker<'graph>,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        if self.limit > 0 {
            self.limit -= 1;
            self.parent.next(graph)
        } else {
            None
        }
    }
}

pub struct EdgeLimit<'graph, Parent> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    limit: usize,
}

impl<Parent> EdgeLimit<'_, Parent> {
    pub fn new(parent: Parent, limit: usize) -> Self {
        Self {
            _phantom_data: Default::default(),
            parent,
            limit,
        }
    }
}

impl<'graph, Parent> Walker<'graph> for EdgeLimit<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
{
    type Graph = Parent::Graph;

    type Context = Parent::Context;
    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<Element<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
        self.next(graph).map(|e| Element::Edge(e.id()))
    }
    fn ctx(&self) -> &Parent::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent> EdgeWalker<'graph> for EdgeLimit<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
{
    fn next(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<<Self::Graph as Graph>::EdgeReference<'graph>> {
        if self.limit > 0 {
            self.limit -= 1;
            self.parent.next(graph)
        } else {
            None
        }
    }
}
