use crate::ElementId;
use crate::graph::Graph;
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
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
    Parent: VertexWalker<'graph>,
    Predicate: Fn(
        &<Parent::Graph as Graph>::VertexReference<'_>,
        &mut Parent::Context,
    ) -> ControlFlow<(), ()>,
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
    Parent: VertexWalker<'graph>,
    Predicate: Fn(
        &<Parent::Graph as Graph>::VertexReference<'_>,
        &mut Parent::Context,
    ) -> ControlFlow<(), ()>,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        while let Some(next) = self.parent.next(graph) {
            if let Some(vertex) = graph.vertex(next) {
                match (self.predicate)(&vertex, self.parent.ctx_mut()) {
                    ControlFlow::Continue(()) => return Some(next),
                    ControlFlow::Break(()) => return None,
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
    Parent: EdgeWalker<'graph>,
    Predicate: Fn(
        &<Parent::Graph as Graph>::EdgeReference<'_>,
        &mut Parent::Context,
    ) -> ControlFlow<(), ()>,
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
    Parent: EdgeWalker<'graph>,
    Predicate: Fn(
        &<Parent::Graph as Graph>::EdgeReference<'_>,
        &mut Parent::Context,
    ) -> ControlFlow<(), ()>,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::EdgeId> {
        if let Some(next) = self.parent.next(graph) {
            let edge = graph.edge(next).expect("edge must exist");
            match (self.predicate)(&edge, self.parent.ctx_mut()) {
                ControlFlow::Continue(()) => return Some(next),
                ControlFlow::Break(()) => return None,
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
    /// `std::ops::ControlFlow` value. This gives precise control over traversal - you can either continue
    /// processing the current element (ControlFlow::Continue) or stop the traversal completely (ControlFlow::Break).
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
    /// After control_flow step with a condition that breaks when encountering [B]:
    /// ```text
    ///   [A]* --- edge1 ---> [B] --- edge2 ---> [C]  
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
    ///   and returns a `std::ops::ControlFlow<(), ()>` value. Return `ControlFlow::Continue(())` to keep
    ///   processing the current element and continue traversal, or `ControlFlow::Break(())` to stop traversal.
    ///
    /// ## Return Value
    ///
    /// A new walker that applies the control flow logic to the traversal.
    ///
    /// ## Example
    ///
    /// ```rust
    /// // Example will be provided once implemented
    /// ```
    ///
    /// ## Notes
    ///
    /// - This step is similar to `filter()` but with the added ability to stop all traversal at any point
    /// - The predicate receives a mutable reference to the context, allowing you to update state during traversal
    /// - Use this step when you need to conditionally stop traversal, such as when finding a specific condition
    /// - Only elements that return `ControlFlow::Continue(())` will be included in the traversal
    /// - When `ControlFlow::Break(())` is returned, the entire traversal stops immediately
    pub fn control_flow<Predicate>(
        self,
        predicate: Predicate,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexControlFlow<'graph, Walker, Predicate>>
    where
        Predicate: Fn(&Graph::VertexReference<'_>, &mut Walker::Context) -> ControlFlow<(), ()>,
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
    /// `std::ops::ControlFlow` value. This gives precise control over traversal - you can either continue
    /// processing the current element (ControlFlow::Continue) or stop the traversal completely (ControlFlow::Break).
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
    /// After control_flow step that breaks on "knows" edges:
    /// ```text
    ///   [Person A] --- knows ---> [Person B] --- created ---> [Project]
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
    ///   and returns a `std::ops::ControlFlow<(), ()>` value. Return `ControlFlow::Continue(())` to keep
    ///   processing the current element and continue traversal, or `ControlFlow::Break(())` to stop traversal.
    ///
    /// ## Return Value
    ///
    /// A new walker that applies the control flow logic to the traversal.
    ///
    /// ## Example
    ///
    /// ```rust
    /// // Example will be provided once implemented
    /// ```
    ///
    /// ## Notes
    ///
    /// - This step is similar to `filter()` but with the added ability to stop all traversal at any point
    /// - The predicate receives a mutable reference to the context, allowing you to update state during traversal
    /// - Use this step when you need to conditionally stop traversal, such as when encountering a specific edge type
    /// - Only elements that return `ControlFlow::Continue(())` will be included in the traversal
    /// - When `ControlFlow::Break(())` is returned, the entire traversal stops immediately
    pub fn control_flow<Predicate>(
        self,
        predicate: Predicate,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeControlFlow<'graph, Walker, Predicate>>
    where
        Predicate: Fn(&Graph::EdgeReference<'_>, &mut Walker::Context) -> ControlFlow<(), ()>,
    {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: EdgeControlFlow::new(self.walker, predicate),
            graph: self.graph,
        }
    }
}
