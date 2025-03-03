use crate::graph::Graph;
use crate::search::vertex::VertexSearch;
use crate::walker::{ VertexWalker, Walker};
use crate::{ElementId, VertexReference};
use std::marker::PhantomData;

pub struct Vertices<'search, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    current_iter: Option<<Parent::Graph as Graph>::VertexIter<'search, 'graph>>,
    vertex_search: VertexSearch<'search, Parent::Graph>,
}

impl<'search, 'graph, Parent> Vertices<'search, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    pub fn new(parent: Parent, vertex_search: VertexSearch<'search, Parent::Graph>) -> Self {
        Self {
            _phantom_data: Default::default(),
            parent,
            current_iter: None,
            vertex_search,
        }
    }
}

impl<'graph, Parent> Walker<'graph> for Vertices<'_, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
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
impl<'graph, Parent> VertexWalker<'graph> for Vertices<'_, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        if self.current_iter.is_none() {
            self.current_iter = Some(graph.vertices(&self.vertex_search));
        }

        self.current_iter
            .as_mut()
            .expect("iterator must be populated")
            .next()
            .map(|next| next.id())
    }
}
