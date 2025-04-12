use crate::walker::EdgeWalker;
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::steps::Endpoints;
use include_doc::function_body;

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// # Head Step
    ///
    /// The `head` step transforms an edge traversal into a vertex traversal by moving to the
    /// head vertex of each edge. In graph theory, the head is the destination/target vertex that
    /// the edge point to.
    ///
    /// ## Visual Diagram
    ///
    /// Before head step (with edges as current elements):
    /// ```text
    ///   [A] --- edge1* ---> [B] --- edge2* ---> [C]  
    ///    ^                                         
    ///    |                                         
    ///   edge3*                                       
    ///    |                                         
    ///   [D]                                        
    /// ```
    ///
    /// After head step (moved to target vertices of edges):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]
    /// ```
    ///
    /// ## Parameters
    ///
    /// None
    ///
    /// ## Return Value
    ///
    /// A vertex walker that will traverse the destination/target vertices of the edges from the previous step.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/head.rs", example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The `head` step can only be used after an edge traversal step
    /// - Transforms the traversal type from EdgeWalker to VertexWalker
    /// - For directed graphs, head refers to the destination/target vertex
    /// - For undirected graphs, the distinction between head and tail may depend on implementation
    /// - Commonly used in conjunction with incoming edges to find vertices that point to certain destinations
    /// - The head-tail terminology follows standard graph theory convention
    /// - When working with edges, remember that `head()` gives you "where the edge points to" (destination)
    pub fn head(self) -> VertexWalkerBuilder<'graph, Mutability, Graph, Endpoints<'graph, Walker>> {
        self.with_vertex_walker(|walker| Endpoints::new(walker, crate::walker::steps::End::Head))
    }
}
