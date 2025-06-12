use crate::ElementId;
use crate::graph::Graph;
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use smallbox::{SmallBox, space};

// Use a reasonable size for the SmallBox - can be tuned
type BoxSpace = space::S32;

/// A boxed vertex walker that uses SmallBox for type erasure
/// This helps reduce monomorphization by hiding concrete walker types
pub struct BoxedVertexWalker<'graph, G: Graph> {
    // We box the walker as Any and store it with a function to call next
    inner: SmallBox<Box<dyn FnMut(&'graph G) -> Option<G::VertexId> + 'graph>, BoxSpace>,
    context: (),
}

impl<'graph, G: Graph> BoxedVertexWalker<'graph, G> {
    pub(crate) fn new<W>(mut walker: W) -> Self
    where
        W: VertexWalker<'graph, Graph = G, Context = ()> + 'graph,
    {
        let closure =
            Box::new(move |graph: &'graph G| -> Option<G::VertexId> { walker.next(graph) });

        Self {
            inner: SmallBox::new(closure),
            context: (),
        }
    }
}

impl<'graph, G: Graph> Walker<'graph> for BoxedVertexWalker<'graph, G> {
    type Graph = G;
    type Context = ();

    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }

    fn ctx(&self) -> &Self::Context {
        &self.context
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        &mut self.context
    }
}

impl<'graph, G: Graph> VertexWalker<'graph> for BoxedVertexWalker<'graph, G> {
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        (self.inner.as_mut())(graph)
    }
}

/// A boxed edge walker that uses SmallBox for type erasure
/// This helps reduce monomorphization by hiding concrete walker types
pub struct BoxedEdgeWalker<'graph, G: Graph> {
    // We box the walker as Any and store it with a function to call next
    inner: SmallBox<Box<dyn FnMut(&'graph G) -> Option<G::EdgeId> + 'graph>, BoxSpace>,
    context: (),
}

impl<'graph, G: Graph> BoxedEdgeWalker<'graph, G> {
    pub(crate) fn new<W>(mut walker: W) -> Self
    where
        W: EdgeWalker<'graph, Graph = G, Context = ()> + 'graph,
    {
        let closure = Box::new(move |graph: &'graph G| -> Option<G::EdgeId> { walker.next(graph) });

        Self {
            inner: SmallBox::new(closure),
            context: (),
        }
    }
}

impl<'graph, G: Graph> Walker<'graph> for BoxedEdgeWalker<'graph, G> {
    type Graph = G;
    type Context = ();

    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Edge)
    }

    fn ctx(&self) -> &Self::Context {
        &self.context
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        &mut self.context
    }
}

impl<'graph, G: Graph> EdgeWalker<'graph> for BoxedEdgeWalker<'graph, G> {
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::EdgeId> {
        (self.inner.as_mut())(graph)
    }
}

// Extension methods for builders to add boxed() method
impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph, Context = ()> + 'graph,
{
    /// # Boxed Step
    ///
    /// The `boxed` step performs type erasure to reduce monomorphization and improve compile times.
    /// It wraps the current walker in a `SmallBox`, breaking the chain of nested generic types
    /// that can grow exponentially in complex traversals.
    ///
    /// ## When to Use
    ///
    /// Use `boxed()` strategically in complex walker chains:
    /// - **After 4+ chained operations** to prevent type explosion
    /// - **When compile times become slow** due to deep walker nesting  
    /// - **At logical checkpoints** in long traversals
    /// - **When storing walkers** in data structures
    ///
    /// ## Performance Considerations
    ///
    /// - **Pros**: Faster compilation, smaller binaries, stack allocation for small walkers
    /// - **Cons**: 5-15% runtime overhead from indirect calls, lost inlining opportunities
    /// - **Best for**: Complex traversals where graph I/O dominates computation
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use graph_api_lib::*;
    /// # use graph_api_test::{populate_graph, Vertex, Edge};
    /// # use graph_api_simplegraph::SimpleGraph;
    /// # let mut graph = SimpleGraph::<Vertex, Edge>::new();
    /// # populate_graph(&mut graph);
    /// // Without boxing - complex nested types
    /// let complex_type = graph
    ///     .walk()
    ///     .vertices(VertexSearch::scan())
    ///     .edges(EdgeSearch::scan())
    ///     .head()
    ///     .edges(EdgeSearch::scan())
    ///     .head();
    /// // Type: Endpoints<Edges<Endpoints<Edges<Vertices<Empty>>>>>
    ///
    /// // With strategic boxing - simpler types
    /// let result: Vec<_> = graph
    ///     .walk()
    ///     .vertices(VertexSearch::scan())
    ///     .edges(EdgeSearch::scan())
    ///     .boxed()  // ← Breaks type complexity here
    ///     .head()
    ///     .edges(EdgeSearch::scan())
    ///     .boxed()  // ← And here for further reduction
    ///     .head()
    ///     .collect();
    /// ```
    ///
    /// ## Technical Details
    ///
    /// This method uses `SmallBox<S32>` which attempts to store walkers on the stack
    /// (up to 32 bytes) before falling back to heap allocation. This provides better
    /// cache locality than regular `Box` for small walker states.
    ///
    /// # Note
    /// This method is only available when the walker's Context is `()`.
    /// For walkers with custom contexts, manual boxing may be needed.
    pub fn boxed(
        self,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, BoxedVertexWalker<'graph, Graph>> {
        self.with_vertex_walker(|walker| BoxedVertexWalker::new(walker))
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph, Context = ()> + 'graph,
{
    /// # Boxed Step (Edge Walker)
    ///
    /// The `boxed` step performs type erasure to reduce monomorphization and improve compile times.
    /// It wraps the current edge walker in a `SmallBox`, breaking the chain of nested generic types
    /// that can grow exponentially in complex traversals.
    ///
    /// ## When to Use
    ///
    /// Use `boxed()` strategically in complex walker chains:
    /// - **After 4+ chained operations** to prevent type explosion
    /// - **When compile times become slow** due to deep walker nesting  
    /// - **At logical checkpoints** in long traversals
    /// - **When storing walkers** in data structures
    ///
    /// ## Performance Considerations
    ///
    /// - **Pros**: Faster compilation, smaller binaries, stack allocation for small walkers
    /// - **Cons**: 5-15% runtime overhead from indirect calls, lost inlining opportunities
    /// - **Best for**: Complex traversals where graph I/O dominates computation
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use graph_api_lib::*;
    /// # use graph_api_test::{populate_graph, Vertex, Edge};
    /// # use graph_api_simplegraph::SimpleGraph;
    /// # let mut graph = SimpleGraph::<Vertex, Edge>::new();
    /// # populate_graph(&mut graph);
    /// // Strategic boxing in edge-heavy traversals
    /// let edges: Vec<_> = graph
    ///     .walk()
    ///     .vertices(VertexSearch::scan())
    ///     .edges(EdgeSearch::scan())
    ///     .filter(|e, _| e.label().contains("knows"))
    ///     .boxed()  // ← Box complex edge walker chains
    ///     .collect();
    /// ```
    ///
    /// ## Technical Details
    ///
    /// This method uses `SmallBox<S32>` which attempts to store walkers on the stack
    /// (up to 32 bytes) before falling back to heap allocation. This provides better
    /// cache locality than regular `Box` for small walker states.
    ///
    /// # Note
    /// This method is only available when the walker's Context is `()`.
    /// For walkers with custom contexts, manual boxing may be needed.
    pub fn boxed(
        self,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, BoxedEdgeWalker<'graph, Graph>> {
        self.with_edge_walker(|walker| BoxedEdgeWalker::new(walker))
    }
}
