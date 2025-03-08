use crate::graph::Graph;
use crate::walker::{VertexWalker, Walker};
use crate::ElementId;
use std::marker::PhantomData;

/// An empty walker that doesn't produce any elements.
///
/// This is a starting point for building more complex walkers.
pub struct Empty<G> {
    _phantom: PhantomData<G>,
}

impl<G> Default for Empty<G> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
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
        &()
    }
}

impl<'graph, G: Graph> VertexWalker<'graph> for Empty<G> {
    fn next(&mut self, _graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        None
    }
}