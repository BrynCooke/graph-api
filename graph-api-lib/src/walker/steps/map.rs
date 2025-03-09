use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};
use crate::walker::steps::into_iter::{EdgeReferenceIterImpl, VertexReferenceIterImpl};
use include_doc::function_body;

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Map Step
    ///
    /// The `map` step transforms elements in the traversal by applying a mapping function to each vertex or edge. 
    /// Unlike other steps that continue the traversal chain, `map` returns an iterator that yields the transformed elements directly.
    ///
    /// ## Visual Diagram
    ///
    /// Before map step:
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*                                        
    /// ```
    ///
    /// After map step with `vertex -> vertex.name()`:
    /// ```text
    ///   "A", "B", "C", "D"
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `mapping`: A function that takes a vertex/edge reference and context, and returns a transformed value
    ///
    /// ## Return Value
    ///
    /// Returns an iterator that yields the transformed elements. The type of the iterator items is determined by the return type of the mapping function.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/map.rs", example, [])]
    /// ```
    ///
    /// For more examples, see the [map example](https://github.com/yourusername/graph-api/blob/main/graph-api-lib/examples/map.rs).
    ///
    /// ## Notes
    ///
    /// - The `map` step is terminal - it returns an iterator, not a traversal builder
    /// - Use `map` when you want to transform the traversal results into a different data type
    /// - The mapping function has access to both the element and its context
    /// - Unlike other traversal steps, map returns a standard Rust iterator
    /// - Common uses include extracting specific properties, computing derived values, or creating DTOs
    /// - For complex transformations, consider using `push_context` to accumulate data before mapping
    pub fn map<R, M>(mut self, mut mapping: M) -> impl Iterator<Item=R> + 'graph
    where
        M: FnMut(Graph::VertexReference<'graph>, Walker::Context) -> R + 'graph,
        Walker: 'graph,
    {
        VertexReferenceIterImpl::new(self.graph.take(), self.walker).map(move |(reference, ctx)| mapping(reference, ctx))
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// Transforms edges in this traversal by applying a mapping function to each edge.
    ///
    /// Returns an iterator that yields the transformed elements.
    ///
    /// See the documentation for [`VertexWalkerBuilder::map`] for more details.
    pub fn map<R, M>(mut self, mut mapping: M) -> impl Iterator<Item=R> + 'graph
    where
        M: FnMut(Graph::EdgeReference<'graph>, Walker::Context) -> R + 'graph,
        Walker: 'graph,
    {
        EdgeReferenceIterImpl::new(self.graph.take(), self.walker).map(move |(reference, ctx)| mapping(reference, ctx))
    }
}