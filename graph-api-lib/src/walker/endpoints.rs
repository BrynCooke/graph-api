use crate::graph::{EdgeReference, Graph};
use crate::walker::{EdgeWalker,  VertexWalker, Walker};
use std::marker::PhantomData;
use crate::ElementId;

pub(crate) enum End {
    Head,
    Tail,
}

pub struct Endpoints<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    end: End,
}

impl<'graph, Parent> Endpoints<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
{
    pub(crate) fn new(parent: Parent, end: End) -> Endpoints<'graph, Parent> {
        Self {
            _phantom_data: Default::default(),
            parent,
            end,
        }
    }
}

impl<'search, 'graph, Parent> Walker<'graph> for Endpoints<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
    <Parent as Walker<'graph>>::Context: Clone + 'static,
    <Parent as Walker<'graph>>::Graph: 'graph,
    <Parent::Graph as Graph>::EdgeIter<'search, 'graph>:
        Iterator<Item = <Parent::Graph as Graph>::EdgeReference<'graph>>,
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

impl<'search, 'graph, Parent> VertexWalker<'graph> for Endpoints<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
    <Parent as Walker<'graph>>::Context: Clone + 'static,
    <Parent as Walker<'graph>>::Graph: 'graph,
    <Parent::Graph as Graph>::EdgeIter<'search, 'graph>:
        Iterator<Item = <Parent::Graph as Graph>::EdgeReference<'graph>>,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        self.parent.next(graph).map(|e| match &self.end {
            End::Head => e.head(),
            End::Tail => e.tail(),
        })
    }
}
