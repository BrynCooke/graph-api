use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use crate::graph::Graph;
use crate::ElementId;
use include_doc::function_body;
use std::marker::PhantomData;

// ================ FILTER IMPLEMENTATION ================

pub struct VertexFilter<'graph, Parent, Predicate> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    predicate: Predicate,
}

impl<Parent, Predicate> VertexFilter<'_, Parent, Predicate> {
    pub(crate) fn new(parent: Parent, predicate: Predicate) -> Self {
        VertexFilter {
            _phantom_data: Default::default(),
            parent,
            predicate,
        }
    }
}

impl<'graph, Parent, Predicate> Walker<'graph> for VertexFilter<'graph, Parent, Predicate>
where
    Parent: VertexWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::VertexReference<'_>, &Parent::Context) -> bool,
{
    type Graph = Parent::Graph;

    type Context = Parent::Context;
    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }
    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent, Predicate> VertexWalker<'graph> for VertexFilter<'graph, Parent, Predicate>
where
    Parent: VertexWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::VertexReference<'_>, &Parent::Context) -> bool,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        while let Some(next) = self.parent.next(graph) {
            if let Some(vertex) = graph.vertex(next) {
                if (self.predicate)(&vertex, self.parent.ctx()) {
                    return Some(next);
                }
            }
        }
        None
    }
}

pub struct EdgeFilter<'graph, Parent, Predicate> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    predicate: Predicate,
}

impl<Parent, Predicate> EdgeFilter<'_, Parent, Predicate> {
    pub(crate) fn new(parent: Parent, predicate: Predicate) -> Self {
        EdgeFilter {
            _phantom_data: Default::default(),
            parent,
            predicate,
        }
    }
}

impl<'graph, Parent, Predicate> Walker<'graph> for EdgeFilter<'graph, Parent, Predicate>
where
    Parent: EdgeWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::EdgeReference<'_>, &Parent::Context) -> bool,
{
    type Graph = Parent::Graph;

    type Context = Parent::Context;
    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Edge)
    }
    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent, Predicate> EdgeWalker<'graph> for EdgeFilter<'graph, Parent, Predicate>
where
    Parent: EdgeWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::EdgeReference<'_>, &Parent::Context) -> bool,
{
    fn next(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<<Self::Graph as Graph>::EdgeId> {
        while let Some(next) = self.parent.next(graph) {
            let edge = graph.edge(next).expect("edge must exist");
            if (self.predicate)(&edge, self.parent.ctx()) {
                return Some(next);
            }
        }
        None
    }
}

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Filter Step
    ///
    /// The `filter` step allows you to keep only elements that match a predicate function. 
    /// Elements that don't match the predicate are excluded from further traversal.
    ///
    /// ## Visual Diagram
    ///
    /// Before filter step (all vertices in traversal):
    /// ```text
    ///   [Person A]* --- knows ---> [Person B]* --- created ---> [Project]*
    ///    ^                                         
    ///    |                                         
    ///   owns                                       
    ///    |                                         
    ///   [Company C]*                                        
    /// ```
    ///
    /// After filter(is_person) step (only Person vertices remain in traversal):
    /// ```text
    ///   [Person A]* --- knows ---> [Person B]* --- created ---> [Project]
    ///    ^                                         
    ///    |                                         
    ///   owns                                       
    ///    |                                         
    ///   [Company C]                                        
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `predicate`: A function that takes a reference to an element and returns a boolean. 
    ///   Only elements for which this function returns `true` will be included in the traversal.
    ///
    /// ## Return Value
    ///
    /// A new walker containing only the elements that matched the predicate.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/filter.rs", example, [])]
    /// ```
    ///
    /// For more examples, see the [filter example](https://github.com/yourusername/graph-api/blob/main/graph-api-lib/examples/filter.rs).
    ///
    /// ## Notes
    ///
    /// - The filter step does not modify the graph, only the traversal
    /// - For complex filtering logic, consider breaking into multiple filter steps for better readability
    /// - Use type projections when filtering to access type-specific methods and properties
    /// - The filter is applied lazily during traversal, not when the step is added to the walker
    pub fn filter<Predicate>(
        self,
        predicate: Predicate,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexFilter<'graph, Walker, Predicate>>
    where
        Predicate: Fn(&Graph::VertexReference<'_>, &Walker::Context) -> bool,
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.filter(predicate),
            graph: self.graph,
        }
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
    <Walker as crate::walker::Walker<'graph>>::Context: Clone + 'static,
{
    /// Filters edges from this traversal based on a predicate function.
    ///
    /// This allows you to keep only edges that match a condition, discarding the rest.
    ///
    /// See the documentation for [`VertexWalkerBuilder::filter`] for more details.
    pub fn filter<Predicate>(
        self,
        predicate: Predicate,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeFilter<'graph, Walker, Predicate>>
    where
        Predicate: Fn(&Graph::EdgeReference<'_>, &Walker::Context) -> bool,
    {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.filter(predicate),
            graph: self.graph,
        }
    }
}