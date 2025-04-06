use crate::graph::Graph;
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use crate::{EdgeReference, ElementId, VertexReference};
use include_doc::function_body;
use std::marker::PhantomData;
use std::ops::ControlFlow;

pub struct VertexControlFlow<'graph, Parent, Predicate> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    predicate: Predicate,
}

impl<Parent, Predicate> VertexControlFlow<'_, Parent, Predicate> {
    pub(crate) fn new(parent: Parent, predicate: Predicate) -> Self {
        Self {
            _phantom_data: Default::default(),
            parent,
            predicate,
        }
    }
}

impl<'graph, Parent, Predicate> Walker<'graph> for VertexControlFlow<'graph, Parent, Predicate>
where
    Self: 'graph,
    Parent: VertexWalker<'graph>,
    for<'a> Predicate: Fn(
        &'a <Parent::Graph as Graph>::VertexReference<'graph>,
        &mut Parent::Context,
    ) -> ControlFlow<
        Option<&'a <Parent::Graph as Graph>::VertexReference<'graph>>,
        Option<&'a <Parent::Graph as Graph>::VertexReference<'graph>>,
    >,
{
    type Graph = Parent::Graph;
    type Context = Parent::Context;

    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }

    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.parent.ctx_mut()
    }
}

impl<'graph, Parent, Predicate> VertexWalker<'graph>
    for VertexControlFlow<'graph, Parent, Predicate>
where
    Self: 'graph,
    Parent: VertexWalker<'graph>,
    for<'a> Predicate: Fn(
        &'a <Parent::Graph as Graph>::VertexReference<'graph>,
        &mut Parent::Context,
    ) -> ControlFlow<
        Option<&'a <Parent::Graph as Graph>::VertexReference<'graph>>,
        Option<&'a <Parent::Graph as Graph>::VertexReference<'graph>>,
    >,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        while let Some(next) = self.parent.next(graph) {
            if let Some(vertex) = graph.vertex(next) {
                match (self.predicate)(&vertex, self.parent.ctx_mut()) {
                    ControlFlow::Continue(Some(reference)) => return Some(reference.id()),
                    ControlFlow::Continue(None) => continue, // Skip this element
                    ControlFlow::Break(reference) => {
                        return reference.map(|reference| reference.id());
                    } // Break with optional final element
                }
            }
        }
        None
    }
}

pub struct EdgeControlFlow<'graph, Parent, Predicate> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    predicate: Predicate,
}

impl<Parent, Predicate> EdgeControlFlow<'_, Parent, Predicate> {
    pub(crate) fn new(parent: Parent, predicate: Predicate) -> Self {
        Self {
            _phantom_data: Default::default(),
            parent,
            predicate,
        }
    }
}

impl<'graph, Parent, Predicate> Walker<'graph> for EdgeControlFlow<'graph, Parent, Predicate>
where
    Self: 'graph,
    Parent: EdgeWalker<'graph>,
    for<'a> Predicate: Fn(
        &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
        &mut Parent::Context,
    ) -> ControlFlow<
        Option<&'a <Parent::Graph as Graph>::EdgeReference<'graph>>,
        Option<&'a <Parent::Graph as Graph>::EdgeReference<'graph>>,
    >,
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

impl<'graph, Parent, Predicate> EdgeWalker<'graph> for EdgeControlFlow<'graph, Parent, Predicate>
where
    Self: 'graph,
    Parent: EdgeWalker<'graph>,
    for<'a> Predicate: Fn(
        &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
        &mut Parent::Context,
    ) -> ControlFlow<
        Option<&'a <Parent::Graph as Graph>::EdgeReference<'graph>>,
        Option<&'a <Parent::Graph as Graph>::EdgeReference<'graph>>,
    >,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::EdgeId> {
        while let Some(next) = self.parent.next(graph) {
            if let Some(edge) = graph.edge(next) {
                match (self.predicate)(&edge, self.parent.ctx_mut()) {
                    ControlFlow::Continue(Some(reference)) => return Some(reference.id()),
                    ControlFlow::Continue(None) => continue, // Skip this element
                    ControlFlow::Break(reference) => {
                        return reference.map(|reference| reference.id());
                    } // Break with optional final element
                }
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
    /// # ControlFlow Step
    ///
    /// The `control_flow` step allows you to evaluate each vertex with a predicate function that returns a
    /// `std::ops::ControlFlow` value. This gives precise control over traversal - you can either:
    /// - Continue and include the element (ControlFlow::Continue(Some(id)))
    /// - Continue but skip the element (ControlFlow::Continue(None))
    /// - Stop traversal with an optional final element (ControlFlow::Break(option))
    ///
    /// ## Visual Diagram
    ///
    /// Before control_flow step (all vertices in traversal):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*
    ///    ^
    ///    |
    ///   edge3
    ///    |
    ///   [D]*
    /// ```
    ///
    /// After control_flow step that only includes projects and breaks on "Graph" projects:
    /// ```text
    ///   [A] --- edge1 ---> [B]* --- edge2 ---> [C]*
    ///    ^
    ///    |
    ///   edge3
    ///    |
    ///   [D]
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `predicate`: A function that takes a reference to a vertex and a mutable reference to its context,
    ///   and returns a `std::ops::ControlFlow<Option<VertexId>, Option<VertexId>>` value:
    ///   - Return `ControlFlow::Continue(Some(vertex.id()))` to include the vertex and continue
    ///   - Return `ControlFlow::Continue(None)` to skip the vertex and continue
    ///   - Return `ControlFlow::Break(Some(vertex.id()))` to include the vertex and stop traversal
    ///   - Return `ControlFlow::Break(None)` to stop traversal without including the vertex
    ///
    /// ## Return Value
    ///
    /// A new walker that applies the control flow logic to the traversal.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/control_flow.rs", vertex_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - This step is more powerful than `filter()` as it can both filter elements and control traversal flow
    /// - The predicate receives a mutable reference to the context, allowing you to update state during traversal
    /// - Use this step when you need a combination of filtering and conditional stopping of traversal
    /// - Only elements where the predicate returns `Some` will be included in the traversal
    /// - When `ControlFlow::Break` is returned, the entire traversal stops immediately
    pub fn control_flow<Predicate>(
        self,
        predicate: Predicate,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexControlFlow<'graph, Walker, Predicate>>
    where
        Walker: 'graph,
        for<'a> Predicate: Fn(
                &'a Graph::VertexReference<'graph>,
                &mut Walker::Context,
            ) -> ControlFlow<
                Option<&'a Graph::VertexReference<'graph>>,
                Option<&'a Graph::VertexReference<'graph>>,
            > + 'graph,
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: VertexControlFlow::new(self.walker, predicate),
            graph: self.graph,
        }
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// # ControlFlow Step
    ///
    /// The `control_flow` step allows you to evaluate each edge with a predicate function that returns a
    /// `std::ops::ControlFlow` value. This gives precise control over traversal - you can either:
    /// - Continue and include the element (ControlFlow::Continue(Some(id)))
    /// - Continue but skip the element (ControlFlow::Continue(None))
    /// - Stop traversal with an optional final element (ControlFlow::Break(option))
    ///
    /// ## Visual Diagram
    ///
    /// Before control_flow step (all edges in traversal):
    /// ```text
    ///   [Person A] --- knows* ---> [Person B] --- created* ---> [Project]
    ///    ^
    ///    |
    ///   owns*
    ///    |
    ///   [Company]
    /// ```
    ///
    /// After control_flow step that only includes "knows" edges and breaks on old connections:
    /// ```text
    ///   [Person A] --- knows* ---> [Person B] --- created ---> [Project]
    ///    ^
    ///    |
    ///   owns
    ///    |
    ///   [Company]
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `predicate`: A function that takes a reference to an edge and a mutable reference to its context,
    ///   and returns a `std::ops::ControlFlow<Option<EdgeId>, Option<EdgeId>>` value:
    ///   - Return `ControlFlow::Continue(Some(edge.id()))` to include the edge and continue
    ///   - Return `ControlFlow::Continue(None)` to skip the edge and continue
    ///   - Return `ControlFlow::Break(Some(edge.id()))` to include the edge and stop traversal
    ///   - Return `ControlFlow::Break(None)` to stop traversal without including the edge
    ///
    /// ## Return Value
    ///
    /// A new walker that applies the control flow logic to the traversal.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/control_flow.rs", edge_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - This step is more powerful than `filter()` as it can both filter elements and control traversal flow
    /// - The predicate receives a mutable reference to the context, allowing you to update state during traversal
    /// - Use this step when you need a combination of filtering and conditional stopping of traversal
    /// - Only elements where the predicate returns `Some` will be included in the traversal
    /// - When `ControlFlow::Break` is returned, the entire traversal stops immediately
    pub fn control_flow<Predicate>(
        self,
        predicate: Predicate,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeControlFlow<'graph, Walker, Predicate>>
    where
        Walker: 'graph,
        for<'a> Predicate: Fn(
                &'a Graph::EdgeReference<'graph>,
                &mut Walker::Context,
            ) -> ControlFlow<
                Option<&'a Graph::EdgeReference<'graph>>,
                Option<&'a Graph::EdgeReference<'graph>>,
            > + 'graph,
    {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: EdgeControlFlow::new(self.walker, predicate),
            graph: self.graph,
        }
    }
}
