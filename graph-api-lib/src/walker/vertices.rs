use crate::graph::Graph;
use crate::search::vertex::VertexSearch;
use crate::walker::{Element, VertexWalker, Walker};
use crate::VertexReference;
use std::marker::PhantomData;

pub struct Vertices<'a, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    current_iter: Option<<Parent::Graph as Graph>::VertexIter<'graph>>,
    vertex_search: VertexSearch<'a, Parent::Graph>,
}

impl<'a, 'graph, Parent> Vertices<'a, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    pub fn new(parent: Parent, vertex_search: VertexSearch<'a, Parent::Graph>) -> Self {
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
    ) -> Option<Element<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
        self.next(graph).map(Element::Vertex)
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
