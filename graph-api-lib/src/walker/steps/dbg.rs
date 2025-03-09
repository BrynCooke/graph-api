use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::steps::{EdgeProbe, VertexProbe};
use crate::walker::{EdgeWalker, VertexWalker};
use include_doc::function_body;

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Debug Step
    ///
    /// The `dbg` step prints elements as they are traversed through the graph, making it 
    /// easier to debug complex traversals. Each element is tagged with the provided label.
    ///
    /// ## Visual Diagram
    ///
    /// Before dbg step:
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*                                        
    /// ```
    ///
    /// After dbg step (elements continue in traversal, but are also printed to console):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*                                        
    ///
    ///   Console output:
    ///   [stage1] Vertex(A)
    ///   [stage1] Vertex(B)
    ///   [stage1] Vertex(C)
    ///   [stage1] Vertex(D)
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `tag`: A string label that will prefix all debug output to help identify which debug step is printing
    ///
    /// ## Return Value
    ///
    /// Returns the same traversal that was passed in, allowing the traversal to continue unmodified.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/dbg.rs", basic_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The `dbg` step is non-destructive - it does not modify the traversal path
    /// - Debug output goes to the console using the standard Debug trait implementation
    /// - Remember that traversals are typically depth-first, so the output order may not be immediately intuitive
    /// - For complex graphs, consider using more descriptive tags at each debug point
    /// - This step is particularly useful for understanding how graph elements flow through a complex traversal
    /// - The `dbg` step has minimal performance impact when not in debug mode
    pub fn dbg(
        self,
        tag: &'static str
    ) -> VertexWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        VertexProbe<'graph, Walker, impl FnMut(&Graph::VertexReference<'_>)>,
    > {
        let callback = move |vertex: &Graph::VertexReference<'_>| {
            println!("{}: {:?}", tag, vertex);
        };
        self.probe(callback)
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// # Debug Step
    ///
    /// Prints edges as they are traversed through the graph, making it easier to debug complex traversals.
    ///
    /// See the documentation for [`VertexWalkerBuilder::dbg`] for more details.
    pub fn dbg(
        self,
        tag: &'static str
    ) -> EdgeWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        EdgeProbe<'graph, Walker, impl FnMut(&Graph::EdgeReference<'_>)>,
    > {
        let callback = move |edge: &Graph::EdgeReference<'_>| {
            println!("{}: {:?}", tag, edge);
        };
        self.probe(callback)
    }
}