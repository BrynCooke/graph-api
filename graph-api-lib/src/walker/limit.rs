use crate::graph::{ Graph};
use crate::walker::{EdgeWalker,  VertexWalker, Walker};
use std::marker::PhantomData;
use crate::ElementId;

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
    ) -> Option<ElementId<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
        self.next(graph).map(ElementId::Vertex)
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
    ) -> Option<ElementId<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
        self.next(graph).map(ElementId::Edge)
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
    ) -> Option<<Self::Graph as Graph>::EdgeId> {
        if self.limit > 0 {
            self.limit -= 1;
            self.parent.next(graph)
        } else {
            None
        }
    }
}
