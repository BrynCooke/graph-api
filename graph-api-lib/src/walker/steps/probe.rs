use crate::graph::Graph;
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use crate::ElementId;
use include_doc::function_body;
use std::marker::PhantomData;

// ================ PROBE IMPLEMENTATION ================

pub struct VertexProbe<'graph, Parent, Callback> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    callback: Callback,
}

impl<Parent, Callback> VertexProbe<'_, Parent, Callback> {
    pub(crate) fn new(parent: Parent, callback: Callback) -> Self {
        VertexProbe {
            _phantom_data: Default::default(),
            parent,
            callback,
        }
    }
}

impl<'graph, Parent, Callback> Walker<'graph> for VertexProbe<'graph, Parent, Callback>
where
    Parent: VertexWalker<'graph>,
    Callback: FnMut(&<Parent::Graph as Graph>::VertexReference<'_>, &Parent::Context),
{
    type Graph = Parent::Graph;
    type Context = Parent::Context;

    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }

    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent, Callback> VertexWalker<'graph> for VertexProbe<'graph, Parent, Callback>
where
    Parent: VertexWalker<'graph>,
    Callback: FnMut(&<Parent::Graph as Graph>::VertexReference<'_>, &Parent::Context),
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        let next = self.parent.next(graph);
        if let Some(id) = next {
            if let Some(vertex) = graph.vertex(id) {
                (self.callback)(&vertex, self.parent.ctx());
            }
        }
        next
    }
}

pub struct EdgeProbe<'graph, Parent, Callback> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    callback: Callback,
}

impl<Parent, Callback> EdgeProbe<'_, Parent, Callback> {
    pub(crate) fn new(parent: Parent, callback: Callback) -> Self {
        EdgeProbe {
            _phantom_data: Default::default(),
            parent,
            callback,
        }
    }
}

impl<'graph, Parent, Callback> Walker<'graph> for EdgeProbe<'graph, Parent, Callback>
where
    Parent: EdgeWalker<'graph>,
    Callback: FnMut(&<Parent::Graph as Graph>::EdgeReference<'_>, &Parent::Context),
{
    type Graph = Parent::Graph;
    type Context = Parent::Context;

    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Edge)
    }

    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent, Callback> EdgeWalker<'graph> for EdgeProbe<'graph, Parent, Callback>
where
    Parent: EdgeWalker<'graph>,
    Callback: FnMut(&<Parent::Graph as Graph>::EdgeReference<'_>, &Parent::Context),
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::EdgeId> {
        let next = self.parent.next(graph);
        if let Some(next) = next {
            let edge = graph.edge(next).expect("edge must exist");
            (self.callback)(&edge, self.parent.ctx());
        }
        next
    }
}

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Probe Step
    ///
    /// The `probe` step allows you to execute a callback function for each vertex in the traversal
    /// without altering the traversal itself. This is useful for debugging, logging, or collecting
    /// information during a traversal.
    ///
    /// ## Visual Diagram
    ///
    /// Before probe step:
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*
    ///    ^
    ///    |
    ///   edge3
    ///    |
    ///   [D]*
    /// ```
    ///
    /// After probe step (unchanged, but callback executed for each vertex *):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*
    ///    ^
    ///    |
    ///   edge3
    ///    |
    ///   [D]*
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `callback`: A function that takes a reference to the current vertex being traversed,
    ///   and optionally the current context.
    ///   The function signature can be either:
    ///   - `FnMut(&Graph::VertexReference<'_>, &Context)` - Probe with access to current context
    ///
    /// ## Return Value
    ///
    /// A walker of the same type as the input with the probe operation added to the pipeline,
    /// allowing for further chaining of operations.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/probe.rs", vertex_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The `probe` step does not modify the traversal path or elements
    /// - The callback function is executed for each vertex as it's traversed
    /// - When using the context variant, you can access traversal context data during probing
    /// - Useful for debugging, logging, or gathering statistics about your graph
    /// - Side effects in the callback function (like printing or counting) do not affect the traversal
    /// - Can be used at multiple points in a traversal to monitor the flow at different stages
    /// - Consider using pattern matching in the callback to work with specific vertex types
    /// - Context access is especially useful when combined with `push_context` steps earlier in the traversal
    pub fn probe<Callback>(
        self,
        callback: Callback,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexProbe<'graph, Walker, Callback>>
    where
        Callback: FnMut(&Graph::VertexReference<'_>, &Walker::Context),
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: VertexProbe::new(self.walker, callback),
            graph: self.graph,
        }
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// # Probe Step
    ///
    /// The `probe` step allows you to execute a callback function for each edge in the traversal
    /// without altering the traversal itself. This is useful for debugging, analyzing connections,
    /// or collecting edge statistics during a traversal.
    ///
    /// ## Visual Diagram
    ///
    /// Before probe step:
    /// ```text
    ///   [Person A] --- knows* ---> [Person B] --- created* ---> [Project]
    ///    ^
    ///    |
    ///   owns*
    ///    |
    ///   [Company]
    /// ```
    ///
    /// After probe step (unchanged, but callback executed for each edge *):
    /// ```text
    ///   [Person A] --- knows* ---> [Person B] --- created* ---> [Project]
    ///    ^
    ///    |
    ///   owns*
    ///    |
    ///   [Company]
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `callback`: A function that takes a reference to the current edge being traversed,
    ///   and optionally the current context.
    ///   The function signature can be either:
    ///   - `FnMut(&Graph::EdgeReference<'_>, &Context)` - Probe with access to current context
    ///
    /// ## Return Value
    ///
    /// A walker of the same type as the input with the probe operation added to the pipeline,
    /// allowing for further chaining of operations.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/probe.rs", edge_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The `probe` step does not modify the traversal path or edges
    /// - The callback function is executed for each edge as it's traversed
    /// - When using the context variant, you can access traversal context data during probing
    /// - Useful for analyzing connection patterns without modifying the traversal
    /// - Consider using pattern matching in the callback to handle different edge types
    /// - You can use endpoint accessors like `tail()` and `head()` to inspect connected vertices
    /// - Context access is especially useful when combined with `push_context` steps earlier in the traversal
    pub fn probe<Callback>(
        self,
        callback: Callback,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeProbe<'graph, Walker, Callback>>
    where
        Callback: FnMut(&Graph::EdgeReference<'_>, &Walker::Context),
    {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: EdgeProbe::new(self.walker, callback),
            graph: self.graph,
        }
    }
}
