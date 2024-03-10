use crate::graph::{EdgeReference, Graph};
use crate::walker::{EdgeWalker, Element, VertexWalker, Walker};
use std::marker::PhantomData;

pub struct VertexFilter<'graph, Parent, Predicate> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    predicate: Predicate,
}

impl<Parent, Predicate> VertexFilter<'_, Parent, Predicate> {
    pub(crate) fn new(parent: Parent, predicate: Predicate) -> Self {
        VertexFilter {
            _phantom_data: Default::default(),
            parent,
            predicate,
        }
    }
}

impl<'graph, Parent, Predicate> Walker<'graph> for VertexFilter<'graph, Parent, Predicate>
where
    Parent: VertexWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::VertexReference<'_>) -> bool,
{
    type Graph = Parent::Graph;

    type Context = Parent::Context;
    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<Element<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
        self.next(graph).map(Element::Vertex)
    }
    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent, Predicate> VertexWalker<'graph> for VertexFilter<'graph, Parent, Predicate>
where
    Parent: VertexWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::VertexReference<'_>) -> bool,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        while let Some(next) = self.parent.next(graph) {
            if let Some(vertex) = graph.vertex(next) {
                if (self.predicate)(&vertex) {
                    return Some(next);
                }
            }
        }
        None
    }
}

pub struct EdgeFilter<'graph, Parent, Predicate> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    predicate: Predicate,
}

impl<Parent, Predicate> EdgeFilter<'_, Parent, Predicate> {
    pub(crate) fn new(parent: Parent, predicate: Predicate) -> Self {
        EdgeFilter {
            _phantom_data: Default::default(),
            parent,
            predicate,
        }
    }
}

impl<'graph, Parent, Predicate> Walker<'graph> for EdgeFilter<'graph, Parent, Predicate>
where
    Parent: EdgeWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::EdgeReference<'_>) -> bool,
{
    type Graph = Parent::Graph;

    type Context = Parent::Context;
    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<Element<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
        self.next(graph).map(|e| Element::Edge(e.id()))
    }
    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent, Predicate> EdgeWalker<'graph> for EdgeFilter<'graph, Parent, Predicate>
where
    Parent: EdgeWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::EdgeReference<'_>) -> bool,
{
    fn next(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<<Self::Graph as Graph>::EdgeReference<'graph>> {
        while let Some(next) = self.parent.next(graph) {
            if (self.predicate)(&next) {
                return Some(next);
            }
        }
        None
    }
}
