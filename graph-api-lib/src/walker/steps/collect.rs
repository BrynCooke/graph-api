use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};
use include_doc::{function_body};

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Collect Step
    ///
    /// The `collect` step finalizes a traversal and gathers the results into a collection. 
    /// This is a terminal operation that ends the traversal and provides access to the traversed elements.
    ///
    /// ## Visual Diagram
    ///
    /// Before collect step (with elements in traversal):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*                                        
    /// ```
    ///
    /// After collect step (all elements consumed and collected):
    /// ```text
    ///   [A] --- edge1 ---> [B] --- edge2 ---> [C]  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]                                        
    ///
    ///   Collection: [A, B, C, D]
    /// ```
    ///
    /// ## Parameters
    ///
    /// None - but the resulting collection type is determined by the type parameter provided to the collect call.
    ///
    /// ## Return Value
    ///
    /// Returns a collection of the traversed elements. The exact type depends on what you're collecting into, commonly:
    /// - `Vec<ElementId>` for simple ID collection
    /// - `Vec<(ElementId, Context)>` when context is used
    /// - Custom types when implementing `FromIterator`
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/collect.rs", example, [])]
    /// ```
    ///
    /// For more examples, see the [collect example](https://github.com/yourusername/graph-api/blob/main/graph-api-lib/examples/collect.rs).
    ///
    /// ## Notes
    ///
    /// - The `collect` step is a terminal operation - no further traversal steps can be added after it
    /// - When collecting with context, use `map` first to format the data for collection
    /// - The collect step fully consumes the traversal
    /// - Most commonly used with `Vec<_>`, but can collect into any type that implements `FromIterator`
    /// - Consider using `limit` before `collect` for large graphs to avoid excessive memory use
    /// - For single-element queries, consider using `first()` instead of `collect` for efficiency
    pub fn collect<T: FromIterator<Graph::VertexId>>(self) -> T
    where
        Walker: VertexWalker<'graph>,
    {
        self.into_iter().collect()
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// Collects all edge IDs from this traversal into a collection of type `T`.
    ///
    /// This is a terminal operation that consumes the traversal.
    ///
    /// See the documentation for [`VertexWalkerBuilder::collect`] for more details.
    pub fn collect<T: FromIterator<Graph::EdgeId>>(self) -> T
    where
        Walker: EdgeWalker<'graph>,
    {
        self.into_iter().collect()
    }
}