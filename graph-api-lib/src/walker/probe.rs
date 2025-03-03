use crate::graph::{EdgeReference, Graph};
use crate::walker::{EdgeWalker,  VertexWalker, Walker};
use std::marker::PhantomData;
use crate::ElementId;

pub struct VertexProbe<'graph, Parent, Callback> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    callback: Callback,
}

impl<Parent, Callback> VertexProbe<'_, Parent, Callback> {
    pub(crate) fn new(parent: Parent, callback: Callback) -> Self {
        VertexProbe {
            _phantom_data: Default::default(),
            parent,
            callback,
        }
    }
}

impl<'graph, Parent, Callback> Walker<'graph> for VertexProbe<'graph, Parent, Callback>
where
    Parent: VertexWalker<'graph>,
    Callback: FnMut(&<Parent::Graph as Graph>::VertexReference<'_>),
{
    type Graph = Parent::Graph;
    type Context = Parent::Context;

    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
        self.next(graph).map(ElementId::Vertex)
    }

    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent, Callback> VertexWalker<'graph> for VertexProbe<'graph, Parent, Callback>
where
    Parent: VertexWalker<'graph>,
    Callback: FnMut(&<Parent::Graph as Graph>::VertexReference<'_>),
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        let next = self.parent.next(graph);
        if let Some(id) = next {
            if let Some(vertex) = graph.vertex(id) {
                (self.callback)(&vertex);
            }
        }
        next
    }
}

pub struct EdgeProbe<'graph, Parent, Callback> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    callback: Callback,
}

impl<Parent, Callback> EdgeProbe<'_, Parent, Callback> {
    pub(crate) fn new(parent: Parent, callback: Callback) -> Self {
        EdgeProbe {
            _phantom_data: Default::default(),
            parent,
            callback,
        }
    }
}

impl<'graph, Parent, Callback> Walker<'graph> for EdgeProbe<'graph, Parent, Callback>
where
    Parent: EdgeWalker<'graph>,
    Callback: FnMut(&<Parent::Graph as Graph>::EdgeReference<'_>),
{
    type Graph = Parent::Graph;
    type Context = Parent::Context;

    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
        self.next(graph).map(|e| ElementId::Edge(e.id()))
    }

    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent, Callback> EdgeWalker<'graph> for EdgeProbe<'graph, Parent, Callback>
where
    Parent: EdgeWalker<'graph>,
    Callback: FnMut(&<Parent::Graph as Graph>::EdgeReference<'_>),
{
    fn next(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<<Self::Graph as Graph>::EdgeReference<'graph>> {
        let next = self.parent.next(graph);
        if let Some(ref edge) = next {
            (self.callback)(edge);
        }
        next
    }
}
