use crate::graph::{EdgeReference, Graph};
use crate::walker::{EdgeWalker, Element, VertexWalker, Walker};
use crate::EdgeSearch;

pub struct Edges<'a, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    parent: Parent,
    current_iter: Option<<Parent::Graph as Graph>::EdgeIter<'graph>>,
    search: EdgeSearch<'a, Parent::Graph>,
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
            search,
            current_iter: None,
            current: None,
        }
    }
}

impl<'graph, Parent> Walker<'graph> for Edges<'_, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
    <Parent::Graph as Graph>::EdgeIter<'graph>:
        Iterator<Item = <Parent::Graph as Graph>::EdgeReference<'graph>>,
{
    type Graph = Parent::Graph;

    type Context = Parent::Context;
    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<Element<<Self::Graph as Graph>::VertexId, <Self::Graph as Graph>::EdgeId>> {
        self.next(graph).map(|e| Element::Edge(e.id()))
    }
    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }
}
impl<'graph, Parent> EdgeWalker<'graph> for Edges<'_, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
    <Parent::Graph as Graph>::EdgeIter<'graph>:
        Iterator<Item = <Parent::Graph as Graph>::EdgeReference<'graph>>,
{
    fn next(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<<Self::Graph as Graph>::EdgeReference<'graph>> {
        loop {
            if let Some(ref mut iter) = self.current_iter {
                if let Some(edge) = iter.next() {
                    if self
                        .search
                        .evaluate(self.current.expect("source vertex must be set"), &edge)
                    {
                        return Some(edge);
                    }
                    // This edge didn't match the search
                    continue;
                }
                self.current_iter = None;
            } else if let Some(vertex) = self.parent.next(graph) {
                self.current = Some(vertex);
                self.current_iter = Some(graph.edges(vertex, &self.search));
            } else {
                return None;
            }
        }
    }
}
