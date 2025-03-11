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
    /// The `first` step retrieves only the first vertex from a traversal and terminates.
    /// This is a terminal operation that consumes the walker and returns an Option containing
    /// the first vertex ID if one exists, or None if the traversal is empty.
    ///
    /// ## Visual Diagram
    ///
    /// Before first step (multiple vertices in traversal):
    /// ```text
    ///   [Project A]* --- uses ---> [Project B]* --- created_by ---> [Person]*
    ///    ^
    ///    |
    ///   uses
    ///    |
    ///   [Project C]*
    /// ```
    ///
    /// After first step (only first vertex returned):
    /// ```text
    ///   [Project A]* --- uses ---> [Project B] --- created_by ---> [Person]
    ///    ^
    ///    |
    ///   uses
    ///    |
    ///   [Project C]
    ///
    ///   Result: Project A
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
    #[doc = function_body!("examples/first.rs", vertex_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The `first` step is a terminal operation - it consumes the walker and returns immediately
    /// - If the traversal is empty, `first` returns None
    /// - More efficient than using `collect` when you only need the first vertex
    /// - Can be combined with filtering to find the first vertex of a specific type
    /// - Order depends on the traversal steps and graph implementation
    /// - For more deterministic results, consider using ordered indexes or sorting steps
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
    /// # First Step
    ///
    /// The `first` step retrieves only the first edge from a traversal and terminates.
    /// This is a terminal operation that consumes the walker and returns an Option containing
    /// the first edge ID if one exists, or None if the traversal is empty.
    ///
    /// ## Visual Diagram
    ///
    /// Before first step (multiple edges in traversal):
    /// ```text
    ///   [Person A] --- knows* ---> [Person B] --- created* ---> [Project]
    ///    ^                                         
    ///    |                                         
    ///   owns*                                       
    ///    |                                         
    ///   [Company]                                        
    /// ```
    ///
    /// After first step (only first edge returned):
    /// ```text
    ///   [Person A] --- knows* ---> [Person B] --- created ---> [Project]
    ///    ^                                         
    ///    |                                         
    ///   owns                                       
    ///    |                                         
    ///   [Company]               
    ///
    ///   Result: knows
    /// ```
    ///
    /// ## Parameters
    ///
    /// None
    ///
    /// ## Return Value
    ///
    /// `Option<EdgeId>` - Returns Some(id) with the first edge ID if the traversal contains at least one edge,
    /// or None if the traversal is empty.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/first.rs", edge_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - Efficient for finding the first matching edge in a graph
    /// - Particularly useful for checking existence of specific relationship types
    /// - Often used with filtering to find specific types of edges
    /// - Returns immediately after finding the first edge, improving performance for large graphs
    /// - After getting the edge ID, you can use graph.edge() to access the full edge data
    pub fn first(mut self) -> Option<Graph::EdgeId> {
        let graph = self.graph.take();
        let mut walker = self.walker;
        walker.next(graph)
    }
}
