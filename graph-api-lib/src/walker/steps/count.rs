use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};
use include_doc::function_body;

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Count Step
    ///
    /// The `count` step fully traverses the graph and returns the number of elements emitted by the traversal.
    ///
    /// ## Visual Diagram
    ///
    /// Before count step (with elements in traversal):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*                                        
    /// ```
    ///
    /// After count step (consumed all elements, returned count 4):
    /// ```text
    ///   [A] --- edge1 ---> [B] --- edge2 ---> [C]  
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
    /// Returns an integer representing the total number of elements in the traversal.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/count.rs", example, [])]
    /// ```
    ///
    /// For more examples, see the [count example](https://github.com/yourusername/graph-api/blob/main/graph-api-lib/examples/count.rs).
    ///
    /// ## Notes
    ///
    /// - The `count` step consumes the entire traversal
    /// - This is a terminal operation - no further steps can be added after `count`
    /// - For very large graphs, be aware that this will traverse the entire graph which may be expensive
    /// - Consider using `limit` before `count` if you only need to check up to a certain number of elements
    pub fn count(mut self) -> usize
    where
        'graph: 'graph,
    {
        let mut count = 0;
        let graph = self.graph();
        let mut walker = self.walker();
        while let Some(_vertex_id) = walker.next(graph) {
            count += 1;
        }
        count
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// Counts the total number of edges in this traversal.
    ///
    /// This is a terminal operation that consumes the traversal.
    ///
    /// See the documentation for [`VertexWalkerBuilder::count`] for more details.
    pub fn count(mut self) -> usize
    where
        'graph: 'graph,
    {
        let mut count = 0;
        let graph = self.graph();
        let mut walker = self.walker();
        while let Some(_vertex_id) = walker.next(graph) {
            count += 1;
        }
        count
    }
}
