use crate::ElementId;
use crate::graph::Graph;
use crate::walker::{VertexWalker, Walker};
use std::marker::PhantomData;

/// # Empty Walker
///
/// An empty walker that doesn't produce any elements.
/// This is a starting point for building more complex walkers.
///
/// ## Visual Diagram
///
/// ```text
/// [Empty] -> []
/// ```
///
/// The empty walker produces no elements and has no context beyond an empty tuple.
/// This serves as the foundation for building traversals that start with fixed elements
/// like vertices_by_id or that pull elements from non-graph sources.
pub struct Empty<Graph, Context> {
    _phantom: PhantomData<Graph>,
    context: Context,
}

impl<Graph> Default for Empty<Graph, ()> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
            context: (),
        }
    }
}

impl<Graph, Context> Empty<Graph, Context> {
    pub(crate) fn with_context(context: Context) -> Self {
        Self {
            _phantom: PhantomData,
            context,
        }
    }
}

impl<'graph, Graph: crate::Graph, Context> Walker<'graph> for Empty<Graph, Context>
where
    Context: Clone + 'static,
{
    type Graph = Graph;
    type Context = Context;

    fn next_element(&mut self, _graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        None
    }

    fn ctx(&self) -> &Self::Context {
        &self.context
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        &mut self.context
    }
}

impl<'graph, G: Graph, Context> VertexWalker<'graph> for Empty<G, Context>
where
    Context: Clone + 'static,
{
    fn next(&mut self, _graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        None
    }
}
