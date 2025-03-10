use crate::graph::{EdgeReference, Graph};
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use crate::{EdgeSearch, ElementId};
use include_doc::function_body;

// ================ EDGES IMPLEMENTATION ================

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
    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Edge)
    }
    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.parent.ctx_mut()
    }
}

impl<'graph, Parent> EdgeWalker<'graph> for Edges<'_, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::EdgeId> {
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

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Edges Step
    ///
    /// The `edges` step allows you to traverse to the edges in a graph.
    /// It moves the traversal position from vertices to their connected edges based on the provided search criteria.
    ///
    /// ## Visual Diagram
    ///
    /// Before edges step (traversal position on vertices):
    /// ```text
    ///   [Person A]* --- knows ---> [Person B] --- created ---> [Project]
    ///    ^                                         
    ///    |                                         
    ///   owns                                       
    ///    |                                         
    ///   [Company C]                                        
    /// ```
    ///
    /// After edges step with outgoing direction (traversal position moves to edges):
    /// ```text
    ///   [Person A] --- knows --->* [Person B] --- created ---> [Project]
    ///    ^                                         
    ///    |                                         
    ///   owns -*                                        
    ///    |                                         
    ///   [Company C]                                        
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `search`: An `EdgeSearch` that defines which edges to include. This can filter by label, direction, and other criteria.
    ///
    /// ## Return Value
    ///
    /// A new walker where the traversal position is on the edges matching the search criteria.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/edges.rs", example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The edges step changes the traversal position from vertices to edges
    /// - To get back to vertices after an edges step, use `head()` or `tail()`
    /// - The search direction matters: `.outgoing()` finds edges where the current vertex is the source,
    ///   `.incoming()` finds edges where the current vertex is the target, and `.bidirectional()` finds both
    /// - The edges step can filter by label and other properties through the EdgeSearch parameter
    pub fn edges<'a, T: Into<EdgeSearch<'a, Graph>>>(
        self,
        search: T,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, Edges<'a, 'graph, Walker>> {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.edges(search.into()),
            graph: self.graph,
        }
    }
}
