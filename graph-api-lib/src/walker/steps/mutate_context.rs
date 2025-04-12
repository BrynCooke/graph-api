use crate::graph::Graph;
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use include_doc::function_body;
use std::marker::PhantomData;

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Mutate Context Step
    ///
    /// The `mutate_context` step allows you to modify the context during traversal. For each element
    /// in the traversal, the provided callback function is executed, giving you the ability to modify
    /// the current context object in-place.
    ///
    /// ## Visual Diagram
    ///
    /// Before mutate_context step (traversal with contexts):
    /// ```text
    ///   [Person A]* + {visited: false} --- knows ---> [Person B]* + {visited: false}
    /// ```
    ///
    /// After mutate_context step (contexts have been modified):
    /// ```text
    ///   [Person A]* + {visited: true} --- knows ---> [Person B]* + {visited: true}
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `callback`: A function that receives:
    ///   - The current vertex reference
    ///   - A mutable reference to the current context
    ///
    /// ## Return Value
    ///
    /// Returns a traversal with the same elements, but with modified context values.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/mutate_context.rs", vertex_mutate_context_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - Unlike `push_context`, this step doesn't create a new context layer
    /// - The callback can modify the context in-place, allowing for updating state during traversal
    /// - Context modifications are applied immediately, affecting subsequent steps
    /// - This is useful for building accumulators or updating state as you traverse
    /// - Can be combined with other context-based steps for complex traversal logic
    /// - When using nested contexts, only the current context level is mutable; parent contexts remain immutable
    pub fn mutate_context<Callback>(
        self,
        callback: Callback,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexMutateContext<'graph, Walker, Callback>>
    where
        Callback: Fn(&Graph::VertexReference<'_>, &mut Walker::Context) + 'graph,
    {
        self.with_vertex_walker(|walker| VertexMutateContext::new(walker, callback))
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// # Mutate Context Step
    ///
    /// The `mutate_context` step allows you to modify the context during edge traversal. For each edge
    /// in the traversal, the provided callback function is executed, giving you the ability to modify
    /// the current context object in-place.
    ///
    /// ## Visual Diagram
    ///
    /// Before mutate_context step (traversal with contexts):
    /// ```text
    ///   [Person A] --- knows* + {weight: 1} ---> [Person B]
    /// ```
    ///
    /// After mutate_context step (contexts have been modified):
    /// ```text
    ///   [Person A] --- knows* + {weight: 2} ---> [Person B]
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `callback`: A function that receives:
    ///   - The current edge reference
    ///   - A mutable reference to the current context
    ///
    /// ## Return Value
    ///
    /// Returns a traversal with the same elements, but with modified context values.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/mutate_context.rs", edge_mutate_context_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// See the documentation for [`VertexWalkerBuilder::mutate_context`] for more details.
    pub fn mutate_context<Callback>(
        self,
        callback: Callback,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeMutateContext<'graph, Walker, Callback>>
    where
        Callback: Fn(&Graph::EdgeReference<'_>, &mut Walker::Context) + 'graph,
    {
        self.with_edge_walker(|walker| EdgeMutateContext::new(walker, callback))
    }
}

pub struct VertexMutateContext<'graph, Parent, Callback>
where
    Parent: VertexWalker<'graph>,
    Callback:
        Fn(&<Parent::Graph as crate::graph::Graph>::VertexReference<'_>, &mut Parent::Context),
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    callback: Callback,
}

impl<'graph, Parent, Callback> VertexMutateContext<'graph, Parent, Callback>
where
    Parent: VertexWalker<'graph>,
    Callback:
        Fn(&<Parent::Graph as crate::graph::Graph>::VertexReference<'_>, &mut Parent::Context),
{
    pub fn new(parent: Parent, callback: Callback) -> Self {
        VertexMutateContext {
            _phantom_data: Default::default(),
            parent,
            callback,
        }
    }
}

impl<'graph, Parent, Callback> Walker<'graph> for VertexMutateContext<'graph, Parent, Callback>
where
    Parent: VertexWalker<'graph>,
    Callback:
        Fn(&<Parent::Graph as crate::graph::Graph>::VertexReference<'_>, &mut Parent::Context),
{
    type Graph = Parent::Graph;
    type Context = Parent::Context;

    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<crate::ElementId<Self::Graph>> {
        self.next(graph).map(crate::ElementId::Vertex)
    }

    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.parent.ctx_mut()
    }
}

impl<'graph, Parent, Callback> VertexWalker<'graph>
    for VertexMutateContext<'graph, Parent, Callback>
where
    Parent: VertexWalker<'graph>,
    Callback:
        Fn(&<Parent::Graph as crate::graph::Graph>::VertexReference<'_>, &mut Parent::Context),
{
    fn next(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<<Self::Graph as crate::graph::Graph>::VertexId> {
        if let Some(next) = self.parent.next(graph) {
            if let Some(vertex) = graph.vertex(next) {
                (self.callback)(&vertex, self.parent.ctx_mut());
                return Some(next);
            }
        }
        None
    }
}

pub struct EdgeMutateContext<'graph, Parent, Callback>
where
    Parent: EdgeWalker<'graph>,
    Callback: Fn(&<Parent::Graph as crate::graph::Graph>::EdgeReference<'_>, &mut Parent::Context),
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    callback: Callback,
}

impl<'graph, Parent, Callback> EdgeMutateContext<'graph, Parent, Callback>
where
    Parent: EdgeWalker<'graph>,
    Callback: Fn(&<Parent::Graph as crate::graph::Graph>::EdgeReference<'_>, &mut Parent::Context),
{
    pub fn new(parent: Parent, callback: Callback) -> Self {
        EdgeMutateContext {
            _phantom_data: Default::default(),
            parent,
            callback,
        }
    }
}

impl<'graph, Parent, Callback> Walker<'graph> for EdgeMutateContext<'graph, Parent, Callback>
where
    Parent: EdgeWalker<'graph>,
    Callback: Fn(&<Parent::Graph as crate::graph::Graph>::EdgeReference<'_>, &mut Parent::Context),
{
    type Graph = Parent::Graph;
    type Context = Parent::Context;

    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<crate::ElementId<Self::Graph>> {
        self.next(graph).map(crate::ElementId::Edge)
    }

    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.parent.ctx_mut()
    }
}

impl<'graph, Parent, Callback> EdgeWalker<'graph> for EdgeMutateContext<'graph, Parent, Callback>
where
    Parent: EdgeWalker<'graph>,
    Callback: Fn(&<Parent::Graph as crate::graph::Graph>::EdgeReference<'_>, &mut Parent::Context),
{
    fn next(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<<Self::Graph as crate::graph::Graph>::EdgeId> {
        if let Some(next) = self.parent.next(graph) {
            if let Some(edge) = graph.edge(next) {
                (self.callback)(&edge, self.parent.ctx_mut());
                return Some(next);
            }
        }
        None
    }
}
