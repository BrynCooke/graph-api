use crate::walker::EdgeWalker;
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::steps::Endpoints;
use include_doc::function_body;

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// # Tail Step
    ///
    /// The `tail` step transforms an edge traversal into a vertex traversal by moving to the
    /// tail vertex of each edge. In graph theory, the tail is the source/origin vertex
    /// that the edge comes from.
    ///
    /// ## Visual Diagram
    ///
    /// Before tail step (with edges as current elements):
    /// ```text
    ///   [A] --- edge1* ---> [B] --- edge2* ---> [C]
    ///    ^                                         
    ///    |                                         
    ///   edge3*                                       
    ///    |                                         
    ///   [D]                                        
    /// ```
    ///
    /// After tail step (moved to source vertices of edges):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*
    /// ```
    ///
    /// ## Parameters
    ///
    /// None
    ///
    /// ## Return Value
    ///
    /// A vertex walker that will traverse the source vertices of the edges from the previous step.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/tail.rs", example, [])]
    /// ```
    ///
    /// For more examples, see the [tail example](https://github.com/yourusername/graph-api/blob/main/graph-api-lib/examples/tail.rs).
    ///
    /// ## Notes
    ///
    /// - The `tail` step can only be used after an edge traversal step
    /// - Transforms the traversal type from EdgeWalker to VertexWalker
    /// - For directed graphs, tail refers to the source/origin vertex
    /// - For undirected graphs, the distinction between head and tail may depend on implementation
    /// - Commonly used in conjunction with `edges` to follow relationships
    /// - The head-tail terminology follows standard graph theory convention
    /// - When working with edges, remember that `tail()` gives you "where the edge comes from" (source)
    pub fn tail(self) -> VertexWalkerBuilder<'graph, Mutability, Graph, Endpoints<'graph, Walker>> {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: Endpoints::new(self.walker, crate::walker::steps::End::Tail),
            graph: self.graph,
        }
    }
}
