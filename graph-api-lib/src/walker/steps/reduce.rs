use crate::ElementId;
use crate::graph::{
    EdgeReference as GraphEdgeReference, Graph, VertexReference as GraphVertexReference,
};
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use include_doc::function_body;
use std::marker::PhantomData;
// ================ REDUCE IMPLEMENTATION ================

pub struct VertexReduce<'graph, Parent, Reducer>
where
    Parent: Walker<'graph>,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    reducer: Reducer,
}

impl<'graph, Parent, Reducer> VertexReduce<'graph, Parent, Reducer>
where
    Parent: Walker<'graph>,
{
    pub(crate) fn new(parent: Parent, reducer: Reducer) -> Self {
        VertexReduce {
            _phantom_data: Default::default(),
            parent,
            reducer,
        }
    }
}

impl<'graph, Parent, Reducer> Walker<'graph> for VertexReduce<'graph, Parent, Reducer>
where
    Parent: VertexWalker<'graph>,
    Parent::Graph: 'graph,
    Reducer: for<'a> Fn(
        &'a <Parent::Graph as Graph>::VertexReference<'graph>,
        &'a <Parent::Graph as Graph>::VertexReference<'graph>,
        &Parent::Context,
    ) -> &'a <Parent::Graph as Graph>::VertexReference<'graph>,
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

impl<'graph, Parent, Reducer> VertexWalker<'graph> for VertexReduce<'graph, Parent, Reducer>
where
    Parent: VertexWalker<'graph>,
    Parent::Graph: 'graph,
    Reducer: for<'a> Fn(
        &'a <Parent::Graph as Graph>::VertexReference<'graph>,
        &'a <Parent::Graph as Graph>::VertexReference<'graph>,
        &Parent::Context,
    ) -> &'a <Parent::Graph as Graph>::VertexReference<'graph>,
    Parent::Context: Clone,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        let mut acc_vertex = None;
        loop {
            if let Some(next) = self.parent.next(graph) {
                let vertex_reference = graph.vertex(next).expect("vertex must exist");
                if let Some(acc_vertex_ref) = &acc_vertex {
                    let result =
                        (self.reducer)(acc_vertex_ref, &vertex_reference, self.parent.ctx());

                    if std::ptr::eq(result, &vertex_reference) {
                        acc_vertex = Some(vertex_reference);
                    }
                } else {
                    // For the first element, we don't apply the reducer, just set it as the accumulator
                    acc_vertex = Some(vertex_reference);
                }
            } else {
                return acc_vertex.map(|acc| acc.id());
            }
        }
    }
}

pub struct EdgeReduce<'graph, Parent, Reducer>
where
    Parent: Walker<'graph>,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    reducer: Reducer,
}

impl<'graph, Parent, Reducer> EdgeReduce<'graph, Parent, Reducer>
where
    Parent: Walker<'graph>,
{
    pub(crate) fn new(parent: Parent, reducer: Reducer) -> Self {
        EdgeReduce {
            _phantom_data: Default::default(),
            parent,
            reducer,
        }
    }
}

impl<'graph, Parent, Reducer> Walker<'graph> for EdgeReduce<'graph, Parent, Reducer>
where
    Parent: EdgeWalker<'graph>,
    Parent::Graph: 'graph,
    Reducer: for<'a> Fn(
        &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
        &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
        &Parent::Context,
    ) -> &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
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

impl<'graph, Parent, Reducer> EdgeWalker<'graph> for EdgeReduce<'graph, Parent, Reducer>
where
    Parent: EdgeWalker<'graph>,
    Parent::Graph: 'graph,
    Reducer: for<'a> Fn(
        &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
        &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
        &Parent::Context,
    ) -> &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
    Parent::Context: Clone,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::EdgeId> {
        let mut acc_edge = None;
        loop {
            if let Some(next) = self.parent.next(graph) {
                let edge_reference = graph.edge(next).expect("edge must exist");
                if let Some(acc_edge_ref) = &acc_edge {
                    let result = (self.reducer)(acc_edge_ref, &edge_reference, self.parent.ctx());

                    if std::ptr::eq(result, &edge_reference) {
                        acc_edge = Some(edge_reference);
                    }
                } else {
                    // For the first element, we don't apply the reducer, just set it as the accumulator
                    acc_edge = Some(edge_reference);
                }
            } else {
                return acc_edge.map(|acc| acc.id());
            }
        }
    }
}

// ================ BUILDER METHODS ================

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph + 'graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Reduce Step
    ///
    /// The `reduce` step combines elements in a traversal using a reducer function,
    /// with the first element as the initial accumulator.
    ///
    /// ## Visual Diagram
    ///
    /// Before reduce step (traversal position on vertices):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*                                        
    /// ```
    ///
    /// After reduce step (a single vertex containing the combined result):
    /// ```text
    ///   [Result]* --- ... ---> [More Traversal Steps]
    /// ```
    ///
    /// ## Parameters
    /// - `reducer`: A closure that takes:
    ///   - The current accumulated element (left)
    ///   - The next element to combine (right)
    ///   - The parent walker's context (passed through)
    ///   - Returns either the left or right element to continue the reduction
    ///
    /// ## Return Value
    ///
    /// Returns a walker containing a single vertex representing the final reduced value.
    /// If the input traversal is empty, the walker will yield nothing.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/reduce.rs", vertex_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The reduce step is a non-terminal operation - it can be chained with other operations
    /// - The walker will yield a single vertex - the final result of combining all input vertices
    /// - If the traversal is empty, the walker will yield nothing
    /// - The first element serves as the initial accumulator value
    /// - Useful for finding maximum/minimum values or combining elements in a custom way
    /// - Unlike `fold`, reduce doesn't require an initial value and can still participate in further traversal
    /// - The reducer function must return a reference to one of the two input elements
    /// - The returned element becomes the new accumulator for the next reduction step
    /// - The reducer function operates on the elements only, the context remains unchanged
    pub fn reduce<Reducer>(
        self,
        reducer: Reducer,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexReduce<'graph, Walker, Reducer>>
    where
        Reducer: for<'a> Fn(
            &'a Graph::VertexReference<'graph>,
            &'a Graph::VertexReference<'graph>,
            &Walker::Context,
        ) -> &'a Graph::VertexReference<'graph>,
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.reduce(reducer),
            graph: self.graph,
        }
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph + 'graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// # Reduce Step
    ///
    /// Combines edges in the traversal using a reducer function, with the first edge as the initial accumulator.
    ///
    /// ## Visual Diagram
    ///
    /// Before reduce step (traversal position on edges):
    /// ```text
    ///   [A] --- edge1* ---> [B] --- edge2* ---> [C]  
    ///    ^                                         
    ///    |                                         
    ///   edge3*                                     
    ///    |                                         
    ///   [D]                                        
    /// ```
    ///
    /// After reduce step (a single edge containing the combined result):
    /// ```text
    ///   [Source] --- [Result]* ---> [Target] --- ... ---> [More Traversal Steps]
    /// ```
    ///
    /// ## Parameters
    /// - `reducer`: A closure that takes:
    ///   - The current accumulated edge (left)
    ///   - The next edge to combine (right)
    ///   - The parent walker's context (passed through)
    ///   - Returns either the left or right edge to continue the reduction
    ///
    /// ## Return Value
    ///
    /// Returns a walker containing a single edge representing the final reduced value.
    /// If the input traversal is empty, the walker will yield nothing.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/reduce.rs", edge_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The reduce step is a non-terminal operation - it can be chained with other operations
    /// - The walker will yield a single edge - the final result of combining all input edges
    /// - If the traversal is empty, the walker will yield nothing
    /// - The first element serves as the initial accumulator value
    /// - The reducer function must return a reference to one of the two input elements
    /// - The returned element becomes the new accumulator for the next reduction step
    /// - The reducer function operates on the elements only, the context remains unchanged
    pub fn reduce<Reducer>(
        self,
        reducer: Reducer,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeReduce<'graph, Walker, Reducer>>
    where
        Reducer: for<'a> Fn(
            &'a Graph::EdgeReference<'graph>,
            &'a Graph::EdgeReference<'graph>,
            &Walker::Context,
        ) -> &'a Graph::EdgeReference<'graph>,
    {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.reduce(reducer),
            graph: self.graph,
        }
    }
}
