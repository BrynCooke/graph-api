use crate::graph::Graph;
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use crate::ElementId;
use include_doc::function_body;
use std::marker::PhantomData;
use std::ops::Deref;

// ================ CONTEXT IMPLEMENTATION ================

#[derive(Clone, Debug)]
pub struct ContextRef<Current, Parent> {
    inner: Inner<Current, Parent>,
}

#[derive(Debug, Clone)]
struct Inner<Current, Parent> {
    parent: Parent,
    delegate: Current,
}

impl<Current, Parent> Deref for ContextRef<Current, Parent> {
    type Target = Current;

    fn deref(&self) -> &Self::Target {
        &self.inner.delegate
    }
}

impl<Current, Parent> ContextRef<Current, Parent> {
    pub(crate) fn new(delegate: Current, parent: Parent) -> ContextRef<Current, Parent> {
        Self {
            inner: Inner { parent, delegate },
        }
    }
    pub fn parent(&self) -> &Parent {
        &self.inner.parent
    }
}

#[derive(Clone, Debug)]
pub struct DefaultVertexContext<VertexId, Vertex> {
    pub(crate) vertex_id: VertexId,
    pub(crate) vertex: Vertex,
}

impl<VertexId, Vertex> DefaultVertexContext<VertexId, Vertex> {
    pub fn vertex(&self) -> &Vertex {
        &self.vertex
    }

    pub fn vertex_id(&self) -> &VertexId {
        &self.vertex_id
    }
}

#[derive(Clone, Debug)]
pub struct DefaultEdgeContext<EdgeId, Edge> {
    pub(crate) edge_id: EdgeId,
    pub(crate) edge: Edge,
}

impl<EdgeId, Edge> DefaultEdgeContext<EdgeId, Edge> {
    pub fn edge(&self) -> &Edge {
        &self.edge
    }

    pub fn edge_id(&self) -> &EdgeId {
        &self.edge_id
    }
}

pub struct VertexContext<'graph, Parent, Callback, Context>
where
    Parent: VertexWalker<'graph>,
    Callback: Fn(&<Parent::Graph as Graph>::VertexReference<'_>, &Parent::Context) -> Context,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    callback: Callback,
    context: Option<Context>,
}

impl<'graph, Parent, Callback, Context> VertexContext<'graph, Parent, Callback, Context>
where
    Parent: VertexWalker<'graph>,
    Callback: Fn(&<Parent::Graph as Graph>::VertexReference<'_>, &Parent::Context) -> Context,
{
    pub fn new(parent: Parent, callback: Callback) -> Self {
        VertexContext {
            _phantom_data: Default::default(),
            parent,
            callback,
            context: None,
        }
    }
}

impl<'graph, Parent, Predicate, Context> Walker<'graph>
    for VertexContext<'graph, Parent, Predicate, Context>
where
    Parent: VertexWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::VertexReference<'_>, &Parent::Context) -> Context,
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
            .expect("context cannot be retrieved before call to next")
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.context
            .as_mut()
            .expect("context cannot be retrieved before call to next")
    }
}

impl<'graph, Parent, Predicate, Context> VertexWalker<'graph>
    for VertexContext<'graph, Parent, Predicate, Context>
where
    Parent: VertexWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::VertexReference<'_>, &Parent::Context) -> Context,
    Context: Clone + 'static,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        while let Some(next) = self.parent.next(graph) {
            if let Some(vertex) = graph.vertex(next) {
                self.context = Some((self.callback)(&vertex, self.parent.ctx()));
                return Some(next);
            }
        }
        None
    }
}

pub struct EdgeContext<'graph, Parent, Callback, Context>
where
    Parent: EdgeWalker<'graph>,
    Callback: Fn(&<Parent::Graph as Graph>::EdgeReference<'_>, &Parent::Context) -> Context,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    callback: Callback,
    context: Option<Context>,
}

impl<'graph, Parent, Callback, Context> EdgeContext<'graph, Parent, Callback, Context>
where
    Parent: EdgeWalker<'graph>,
    Callback: Fn(&<Parent::Graph as Graph>::EdgeReference<'_>, &Parent::Context) -> Context,
{
    pub fn new(parent: Parent, callback: Callback) -> Self {
        EdgeContext {
            _phantom_data: Default::default(),
            parent,
            callback,
            context: None,
        }
    }
}

impl<'graph, Parent, Predicate, Context> Walker<'graph>
    for EdgeContext<'graph, Parent, Predicate, Context>
where
    Parent: EdgeWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::EdgeReference<'_>, &Parent::Context) -> Context,
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
            .expect("context cannot be retrieved before call to next")
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.context
            .as_mut()
            .expect("context cannot be retrieved before call to next")
    }
}

impl<'graph, Parent, Predicate, Context> EdgeWalker<'graph>
    for EdgeContext<'graph, Parent, Predicate, Context>
where
    Parent: EdgeWalker<'graph>,
    Predicate: Fn(&<Parent::Graph as Graph>::EdgeReference<'_>, &Parent::Context) -> Context,
    Context: Clone + 'static,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::EdgeId> {
        if let Some(next) = self.parent.next(graph) {
            if let Some(edge) = graph.edge(next) {
                self.context = Some((self.callback)(&edge, self.parent.ctx()));
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
    /// # Context Step
    ///
    /// The `push_context` step allows you to associate additional data with each element in the traversal.
    /// This is useful for carrying information along as you traverse, preserving state between traversal steps,
    /// or accumulating results.
    ///
    /// ## Visual Diagram
    ///
    /// Before push_context step (traversal with regular elements):
    /// ```text
    ///   [Person A]* --- created ---> [Project X]*  
    ///    |
    ///   knows
    ///    |
    ///   [Person B]*
    /// ```
    ///
    /// After push_context step (elements now have associated context data):
    /// ```text
    ///   [Person A]* + {age: 30} --- created ---> [Project X]* + {name: "Graph API"}
    ///    |
    ///   knows
    ///    |
    ///   [Person B]* + {age: 25}
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `callback`: A function that takes the current element and its existing context,
    ///   and returns a new context value to associate with that element
    ///
    /// ## Return Value
    ///
    /// Returns a traversal with the same elements, but with additional context information
    /// attached to each element.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/context.rs", vertex_context_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - Context is carried through the entire traversal, even across different graph elements
    /// - Each push_context call creates a new context layer, with the previous context available as `ctx.parent()`
    /// - For complex traversals, you can build a nested context structure
    /// - The context is cloned for each element, so keep context objects relatively small for performance
    /// - Use `push_default_context()` for common patterns like storing the element's ID and data
    /// - Context persists even when traversing to different elements (e.g., from vertex to connected edge)
    /// - When retrieving results, both the element and its context are returned in a tuple
    pub fn push_context<Callback, Context>(
        self,
        callback: Callback,
    ) -> VertexWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        VertexContext<
            'graph,
            Walker,
            impl Fn(
                &Graph::VertexReference<'_>,
                &Walker::Context,
            ) -> ContextRef<Context, Walker::Context>,
            ContextRef<Context, Walker::Context>,
        >,
    >
    where
        Callback: Fn(&Graph::VertexReference<'_>, &Walker::Context) -> Context + 'graph,
        Context: Clone + 'static,
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.context(move |vertex, context| {
                ContextRef::new(callback(vertex, context), context.clone())
            }),
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
    /// # Context Step
    ///
    /// The `push_context` step allows you to associate additional data with each edge in the traversal.
    /// This is useful for carrying information along as you traverse, preserving state between traversal steps,
    /// or accumulating results.
    ///
    /// ## Visual Diagram
    ///
    /// Before push_context step (traversal with regular edges):
    /// ```text
    ///   [Person A] --- created* ---> [Project X]  
    ///    |
    ///   knows*
    ///    |
    ///    v
    ///   [Person B]
    /// ```
    ///
    /// After push_context step (edges now have associated context data):
    /// ```text
    ///   [Person A] --- created* + {type: "maintainer"} ---> [Project X]  
    ///    |
    ///   knows* + {since: "2020"}
    ///    |
    ///    v
    ///   [Person B]
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `callback`: A function that takes the current edge and its existing context,
    ///   and returns a new context value to associate with that edge
    ///
    /// ## Return Value
    ///
    /// Returns a traversal with the same elements, but with additional context information
    /// attached to each edge.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/context.rs", edge_context_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - Context is carried through the entire traversal, even across different graph elements
    /// - Each push_context call creates a new context layer, with the previous context available as `ctx.parent()`
    /// - For complex traversals, you can build a nested context structure
    /// - The context is cloned for each element, so keep context objects relatively small for performance
    /// - Use `push_default_context()` for common patterns like storing the edge's ID and data
    /// - When retrieving results, both the element and its context are returned in a tuple
    pub fn push_context<Callback, Context>(
        self,
        callback: Callback,
    ) -> EdgeWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        EdgeContext<
            'graph,
            Walker,
            impl Fn(&Graph::EdgeReference<'_>, &Walker::Context) -> ContextRef<Context, Walker::Context>,
            ContextRef<Context, Walker::Context>,
        >,
    >
    where
        Callback: Fn(&Graph::EdgeReference<'_>, &Walker::Context) -> Context,
        Context: Clone + 'static,
    {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.context(move |edge, context| {
                ContextRef::new(callback(edge, context), context.clone())
            }),
            graph: self.graph,
        }
    }
}
