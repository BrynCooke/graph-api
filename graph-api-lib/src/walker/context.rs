use crate::graph::{ Graph};
use crate::walker::{EdgeWalker,  VertexWalker, Walker};
use std::marker::PhantomData;
use std::ops::Deref;
use crate::ElementId;

#[derive(Clone, Debug)]
pub struct ContextRef<Current, Parent> {
    inner: Inner<Current, Parent>,
}

#[derive(Debug, Clone)]
struct Inner<Current, Parent> {
    parent: Parent,
    delegate: Current,
}

impl<Current, Parent> Deref for ContextRef<Current, Parent> {
    type Target = Current;

    fn deref(&self) -> &Self::Target {
        &self.inner.delegate
    }
}

impl<Current, Parent> ContextRef<Current, Parent> {
    pub(crate) fn new(delegate: Current, parent: Parent) -> ContextRef<Current, Parent> {
        Self {
            inner: Inner { parent, delegate },
        }
    }
    pub fn parent(&self) -> &Parent {
        &self.inner.parent
    }
}

#[derive(Clone, Debug)]
pub struct DefaultVertexContext<VertexId, Vertex> {
    pub(crate) vertex_id: VertexId,
    pub(crate) vertex: Vertex,
}

impl<VertexId, Vertex> DefaultVertexContext<VertexId, Vertex> {
    pub fn vertex(&self) -> &Vertex {
        &self.vertex
    }

    pub fn vertex_id(&self) -> &VertexId {
        &self.vertex_id
    }
}

#[derive(Clone, Debug)]
pub struct DefaultEdgeContext<EdgeId, Edge> {
    pub(crate) edge_id: EdgeId,
    pub(crate) edge: Edge,
}

impl<EdgeId, Edge> DefaultEdgeContext<EdgeId, Edge> {
    pub fn edge(&self) -> &Edge {
        &self.edge
    }

    pub fn edge_id(&self) -> &EdgeId {
        &self.edge_id
    }
}

pub struct VertexContext<'graph, Parent, Callback, Context>
where
    Parent: VertexWalker<'graph>,
    Callback: Fn(&<Parent::Graph as Graph>::VertexReference<'_>, &Parent::Context) -> Context,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    callback: Callback,
    context: Option<Context>,
}

impl<'graph, Parent, Callback, Context> VertexContext<'graph, Parent, Callback, Context>
where
    Parent: VertexWalker<'graph>,
    Callback: Fn(&<Parent::Graph as Graph>::VertexReference<'_>, &Parent::Context) -> Context,
{
    pub fn new(parent: Parent, callback: Callback) -> Self {
        VertexContext {
            _phantom_data: Default::default(),
            parent,
            callback,
            context: None,
        }
    }
}

impl<'graph, Parent, Predicate, Context> Walker<'graph>
    for VertexContext<'graph, Parent, Predicate, Context>
where
    Parent: VertexWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::VertexReference<'_>, &Parent::Context) -> Context,
    Context: Clone + 'static,
{
    type Graph = Parent::Graph;
    type Context = Context;

    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
        self.next(graph).map(ElementId::Vertex)
    }

    fn ctx(&self) -> &Self::Context {
        self.context
            .as_ref()
            .expect("context cannot be retrieved before call to next")
    }
}

impl<'graph, Parent, Predicate, Context> VertexWalker<'graph>
    for VertexContext<'graph, Parent, Predicate, Context>
where
    Parent: VertexWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::VertexReference<'_>, &Parent::Context) -> Context,
    Context: Clone + 'static,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        while let Some(next) = self.parent.next(graph) {
            if let Some(vertex) = graph.vertex(next) {
                self.context = Some((self.callback)(&vertex, self.parent.ctx()));
                return Some(next);
            }
        }
        None
    }
}

pub struct EdgeContext<'graph, Parent, Callback, Context>
where
    Parent: EdgeWalker<'graph>,
    Callback: Fn(&<Parent::Graph as Graph>::EdgeReference<'_>, &Parent::Context) -> Context,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    callback: Callback,
    context: Option<Context>,
}

impl<'graph, Parent, Callback, Context> EdgeContext<'graph, Parent, Callback, Context>
where
    Parent: EdgeWalker<'graph>,
    Callback: Fn(&<Parent::Graph as Graph>::EdgeReference<'_>, &Parent::Context) -> Context,
{
    pub fn new(parent: Parent, callback: Callback) -> Self {
        EdgeContext {
            _phantom_data: Default::default(),
            parent,
            callback,
            context: None,
        }
    }
}

impl<'graph, Parent, Predicate, Context> Walker<'graph>
    for EdgeContext<'graph, Parent, Predicate, Context>
where
    Parent: EdgeWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::EdgeReference<'_>, &Parent::Context) -> Context,
    Context: Clone + 'static,
{
    type Graph = Parent::Graph;

    type Context = Context;

    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
        self.next(graph).map(ElementId::Edge)
    }
    fn ctx(&self) -> &Self::Context {
        self.context
            .as_ref()
            .expect("context cannot be retrieved before call to next")
    }
}

impl<'graph, Parent, Predicate, Context> EdgeWalker<'graph>
    for EdgeContext<'graph, Parent, Predicate, Context>
where
    Parent: EdgeWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::EdgeReference<'_>, &Parent::Context) -> Context,
    Context: Clone + 'static,
{
    fn next(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<<Self::Graph as Graph>::EdgeId> {
        if let Some(next) = self.parent.next(graph) {
            if let Some(edge) = graph.edge(next) {
                self.context = Some((self.callback)(&edge, self.parent.ctx()));
                return Some(next);
            }
        }
        None
    }
}
