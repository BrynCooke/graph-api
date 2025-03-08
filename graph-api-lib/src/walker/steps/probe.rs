use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use crate::graph::Graph;
use crate::ElementId;
use std::marker::PhantomData;

// ================ PROBE IMPLEMENTATION ================

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
    ) -> Option<ElementId<Self::Graph>> {
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
    ) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Edge)
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
    ) -> Option<<Self::Graph as Graph>::EdgeId> {
        let next = self.parent.next(graph);
        if let Some(next) = next {
            let edge = graph.edge(next).expect("edge must exist");
            (self.callback)(&edge);
        }
        next
    }
}

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/probe.md")]
    pub fn probe<Callback>(
        self,
        callback: Callback,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexProbe<'graph, Walker, Callback>>
    where
        Callback: FnMut(&Graph::VertexReference<'_>),
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: VertexProbe::new(self.walker, callback),
            graph: self.graph,
        }
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    #[doc = include_str!("../../../../graph-api-book/src/user_guide/walker/steps/probe.md")]
    pub fn probe<Callback>(
        self,
        callback: Callback,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeProbe<'graph, Walker, Callback>>
    where
        Callback: FnMut(&Graph::EdgeReference<'_>),
    {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: EdgeProbe::new(self.walker, callback),
            graph: self.graph,
        }
    }
}