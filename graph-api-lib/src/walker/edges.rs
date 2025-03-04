use crate::graph::{EdgeReference, Graph};
use crate::walker::{EdgeWalker,  VertexWalker, Walker};
use crate::{EdgeSearch, ElementId};

pub struct Edges<'search, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    parent: Parent,
    current_iter: Option<<Parent::Graph as Graph>::EdgeIter<'search, 'graph>>,
    edge_search: EdgeSearch<'search, Parent::Graph>,
    current: Option<<Parent::Graph as Graph>::VertexId>,
}

impl<'a, 'graph, Parent> Edges<'a, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
{
    pub(crate) fn new(
        parent: Parent,
        search: EdgeSearch<'a, Parent::Graph>,
    ) -> Edges<'a, 'graph, Parent> {
        Self {
            parent,
            edge_search: search,
            current_iter: None,
            current: None,
        }
    }
}

impl<'search, 'graph, Parent> Walker<'graph> for Edges<'_, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
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
        self.next(graph).map(ElementId::Edge)
    }
    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }
}
impl<'graph, Parent> EdgeWalker<'graph> for Edges<'_, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    fn next(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<<Self::Graph as Graph>::EdgeId> {
        loop {
            if let Some(ref mut iter) = self.current_iter {
                if let Some(edge) = iter.next() {
                    return Some(edge.id());
                }
                self.current_iter = None;
            } else if let Some(vertex) = self.parent.next(graph) {
                self.current = Some(vertex);
                self.current_iter = Some(graph.edges(vertex, &self.edge_search));
            } else {
                return None;
            }
        }
    }
}
