use crate::graph::Graph;
use crate::walker::{VertexWalker, Walker};
use crate::ElementId;
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
pub struct Empty<G> {
    _phantom: PhantomData<G>,
    context: (),
}

impl<G> Default for Empty<G> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
            context: (),
        }
    }
}

impl<'graph, G: Graph> Walker<'graph> for Empty<G> {
    type Graph = G;
    type Context = ();

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

impl<'graph, G: Graph> VertexWalker<'graph> for Empty<G> {
    fn next(&mut self, _graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        None
    }
}
