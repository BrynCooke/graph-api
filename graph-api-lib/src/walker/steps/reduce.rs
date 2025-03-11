use crate::graph::{
    EdgeReference as GraphEdgeReference, Graph, VertexReference as GraphVertexReference,
};
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use crate::ElementId;
use include_doc::function_body;
use std::marker::PhantomData;

// ================ REDUCE IMPLEMENTATION ================

pub struct VertexReduce<'graph, Parent, Init, Reducer, Context>
where
    Parent: Walker<'graph>,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    init: Init,
    reducer: Reducer,
    context: Option<Context>,
}

impl<'graph, Parent, Init, Reducer, Context> VertexReduce<'graph, Parent, Init, Reducer, Context>
where
    Parent: Walker<'graph>,
{
    pub(crate) fn new(parent: Parent, init: Init, reducer: Reducer) -> Self {
        VertexReduce {
            _phantom_data: Default::default(),
            parent,
            init,
            reducer,
            context: None,
        }
    }
}

impl<'graph, Parent, Init, Reducer, Context> Walker<'graph>
    for VertexReduce<'graph, Parent, Init, Reducer, Context>
where
    Parent: VertexWalker<'graph>,
    Parent::Graph: 'graph,
    Init: Fn(&<Parent::Graph as Graph>::VertexReference<'graph>, &Parent::Context) -> Context,
    Reducer: for<'a> Fn(
        &'a <Parent::Graph as Graph>::VertexReference<'graph>,
        &mut Context,
        &'a <Parent::Graph as Graph>::VertexReference<'graph>,
        &Parent::Context,
    ) -> &'a <Parent::Graph as Graph>::VertexReference<'graph>,
    Context: Clone + 'static,
{
    type Graph = Parent::Graph;
    type Context = Context;

    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }

    fn ctx(&self) -> &Self::Context {
        self.context
            .as_ref()
            .expect("next must be called before calling ctx")
    }
    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.context
            .as_mut()
            .expect("next must be called before calling ctx")
    }
}

impl<'graph, Parent, Init, Reducer, Context> VertexWalker<'graph>
    for VertexReduce<'graph, Parent, Init, Reducer, Context>
where
    Parent: VertexWalker<'graph>,
    Parent::Graph: 'graph,
    Init: Fn(&<Parent::Graph as Graph>::VertexReference<'graph>, &Parent::Context) -> Context,
    Reducer: for<'a> Fn(
        &'a <Parent::Graph as Graph>::VertexReference<'graph>,
        &mut Context,
        &'a <Parent::Graph as Graph>::VertexReference<'graph>,
        &Parent::Context,
    ) -> &'a <Parent::Graph as Graph>::VertexReference<'graph>,
    Context: Clone + 'static,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        let mut acc_vertex = None;
        loop {
            if let Some(next) = self.parent.next(graph) {
                let vertex_reference = graph.vertex(next).expect("vertex must exist");
                if let Some(acc_vertex) = &mut acc_vertex {
                    let result = (self.reducer)(
                        acc_vertex,
                        self.context
                            .as_mut()
                            .expect("context must have been initialized"),
                        &vertex_reference,
                        self.parent.ctx(),
                    );
                    if std::ptr::eq(result, &vertex_reference) {
                        *acc_vertex = vertex_reference;
                    }
                } else {
                    self.context = Some((self.init)(&vertex_reference, self.parent.ctx()));
                    acc_vertex = Some(vertex_reference);
                }
            } else {
                return acc_vertex.map(|acc| acc.id());
            }
        }
    }
}

pub struct EdgeReduce<'graph, Parent, Init, Reducer, Context>
where
    Parent: Walker<'graph>,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    init: Init,
    reducer: Reducer,
    context: Option<Context>,
}

impl<'graph, Parent, Init, Reducer, Context> EdgeReduce<'graph, Parent, Init, Reducer, Context>
where
    Parent: Walker<'graph>,
{
    pub(crate) fn new(parent: Parent, init: Init, reducer: Reducer) -> Self {
        EdgeReduce {
            _phantom_data: Default::default(),
            parent,
            init,
            reducer,
            context: None,
        }
    }
}

impl<'graph, Parent, Init, Reducer, Context> Walker<'graph>
    for EdgeReduce<'graph, Parent, Init, Reducer, Context>
where
    Parent: EdgeWalker<'graph>,
    Parent::Graph: 'graph,
    Init: Fn(&<Parent::Graph as Graph>::EdgeReference<'graph>, &Parent::Context) -> Context,
    Reducer: for<'a> Fn(
        &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
        &mut Context,
        &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
        &Parent::Context,
    ) -> &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
    Context: Clone + 'static,
{
    type Graph = Parent::Graph;
    type Context = Context;

    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Edge)
    }

    fn ctx(&self) -> &Self::Context {
        self.context
            .as_ref()
            .expect("Context should be initialized after first next() call")
    }
    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.context
            .as_mut()
            .expect("Context should be initialized after first next() call")
    }
}

impl<'graph, Parent, Init, Reducer, Context> EdgeWalker<'graph>
    for EdgeReduce<'graph, Parent, Init, Reducer, Context>
where
    Parent: EdgeWalker<'graph>,
    Parent::Graph: 'graph,
    Init: Fn(&<Parent::Graph as Graph>::EdgeReference<'graph>, &Parent::Context) -> Context,
    Reducer: for<'a> Fn(
        &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
        &mut Context,
        &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
        &Parent::Context,
    ) -> &'a <Parent::Graph as Graph>::EdgeReference<'graph>,
    Context: Clone + 'static,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::EdgeId> {
        let mut acc_edge = None;
        loop {
            if let Some(next) = self.parent.next(graph) {
                let edge_reference = graph.edge(next).expect("edge must exist");
                if let Some(acc_edge) = &mut acc_edge {
                    let result = (self.reducer)(
                        acc_edge,
                        self.context
                            .as_mut()
                            .expect("context must have been initialized"),
                        &edge_reference,
                        self.parent.ctx(),
                    );
                    if std::ptr::eq(result, &edge_reference) {
                        *acc_edge = edge_reference;
                    }
                } else {
                    self.context = Some((self.init)(&edge_reference, self.parent.ctx()));
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
    /// - The context to push if no elements are reduced.
    /// - `f`: A closure that takes:
    ///   - The current accumulated element
    ///   - The current element's context
    ///   - The next element to combine
    ///   - The next element's context
    ///   - Returns a tuple of (new accumulated element, new context)
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
    /// - The reducer function must return the same type as the elements in the traversal
    /// - The reducer function also produces a new context that is used for subsequent operations
    pub fn reduce<Init, Reducer, Context>(
        self,
        init: Init,
        reducer: Reducer,
    ) -> VertexWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        VertexReduce<'graph, Walker, Init, Reducer, Context>,
    >
    where
        Init: Fn(&Graph::VertexReference<'graph>, &Walker::Context) -> Context,
        Reducer: for<'a> Fn(
            &'a Graph::VertexReference<'graph>,
            &mut Context,
            &'a Graph::VertexReference<'graph>,
            &Walker::Context,
        ) -> &'a Graph::VertexReference<'graph>,
        Context: Clone + 'static,
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.reduce(init, reducer),
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
    /// See the documentation for [`VertexWalkerBuilder::reduce`] for more details.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/reduce.rs", edge_example, [])]
    /// ```
    pub fn reduce<Init, Reducer, Context>(
        self,
        init: Init,
        reducer: Reducer,
    ) -> EdgeWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        EdgeReduce<'graph, Walker, Init, Reducer, Context>,
    >
    where
        Init: Fn(&Graph::EdgeReference<'graph>, &Walker::Context) -> Context,
        Reducer: for<'a> Fn(
            &'a Graph::EdgeReference<'graph>,
            &mut Context,
            &'a Graph::EdgeReference<'graph>,
            &Walker::Context,
        ) -> &'a Graph::EdgeReference<'graph>,
        Context: Clone + 'static,
    {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.reduce(init, reducer),
            graph: self.graph,
        }
    }
}
