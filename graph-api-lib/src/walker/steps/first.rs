use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};
use include_doc::function_body;

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # First Step
    ///
    /// The `first` step retrieves only the first element from a traversal and terminates.
    /// This is a terminal operation that consumes the walker and returns an Option containing
    /// the first element if one exists, or None if the traversal is empty.
    ///
    /// ## Visual Diagram
    ///
    /// Before first step (multiple vertices in traversal):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*
    ///    ^
    ///    |
    ///   edge3
    ///    |
    ///   [D]*
    /// ```
    ///
    /// After first step (only first vertex returned):
    /// ```text
    ///   [A]* --- edge1 ---> [B] --- edge2 ---> [C]
    ///    ^
    ///    |
    ///   edge3
    ///    |
    ///   [D]
    ///
    ///   Result: A
    /// ```
    ///
    /// ## Parameters
    ///
    /// None
    ///
    /// ## Return Value
    ///
    /// `Option<VertexId>` - Returns Some(id) with the first vertex ID if the traversal contains at least one element,
    /// or None if the traversal is empty.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/first.rs", example, [])]
    /// ```
    ///
    /// For more examples, see the [first example](https://github.com/yourusername/graph-api/blob/main/graph-api-lib/examples/first.rs).
    ///
    /// ## Notes
    ///
    /// - The `first` step is a terminal operation - it consumes the walker and no further traversal steps can be added after it
    /// - If the traversal is empty, `first` returns None
    /// - This is more efficient than using `collect` when you only need the first element
    /// - Order is determined by the graph implementation and previous traversal steps
    /// - Does not modify the graph itself, only retrieves the first element from the traversal
    pub fn first(mut self) -> Option<Graph::VertexId>
    where
        'graph: 'graph,
    {
        let graph = self.graph.take();
        let mut walker = self.walker;
        walker.next(graph)
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// Returns the first edge ID from this traversal, if one exists.
    ///
    /// This is a terminal operation that consumes the walker.
    ///
    /// See the documentation for [`VertexWalkerBuilder::first`] for more details.
    pub fn first(mut self) -> Option<Graph::EdgeId> {
        let graph = self.graph.take();
        let mut walker = self.walker;
        walker.next(graph)
    }
}