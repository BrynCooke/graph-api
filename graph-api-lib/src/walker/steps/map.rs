use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::steps::into_iter::{EdgeReferenceIterImpl, VertexReferenceIterImpl};
use crate::walker::{EdgeWalker, VertexWalker};
use include_doc::function_body;

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Map Step
    ///
    /// The `map` step transforms vertices in the traversal by applying a mapping function to each vertex.
    /// Unlike other steps that continue the traversal chain, `map` returns an iterator that yields the
    /// transformed elements directly.
    ///
    /// ## Visual Diagram
    ///
    /// Before map step:
    /// ```text
    ///   [Project A]* --- created_by ---> [Person B]* --- owns ---> [Project C]*  
    ///    ^                                         
    ///    |                                         
    ///   uses                                       
    ///    |                                         
    ///   [Project D]*                                        
    /// ```
    ///
    /// After map step with `vertex -> extract project name`:
    /// ```text
    ///   "Project A", "Person B", "Project C", "Project D"
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `mapping`: A function that takes a vertex reference and context, and returns a transformed value.
    ///   The function signature is `FnMut(Graph::VertexReference<'graph>, Walker::Context) -> R`.
    ///
    /// ## Return Value
    ///
    /// Returns an iterator that yields the transformed elements. The type of the iterator items
    /// is determined by the return type of the mapping function.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/map.rs", vertex_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The `map` step is terminal - it returns an iterator, not a traversal builder
    /// - The mapping function has access to both the vertex and its context
    /// - After mapping, you can continue with standard Rust iterator operations like filter or collect
    /// - Common uses include extracting properties from vertices (e.g., names, IDs, attributes)
    /// - For building complex data structures, consider using pattern matching in the mapping function
    /// - To map vertices with context data, use `push_context` before mapping
    pub fn map<R, M>(mut self, mut mapping: M) -> impl Iterator<Item = R> + 'graph
    where
        M: FnMut(Graph::VertexReference<'graph>, Walker::Context) -> R + 'graph,
        Walker: 'graph,
    {
        VertexReferenceIterImpl::new(self.graph(), self.walker())
            .map(move |(reference, ctx)| mapping(reference, ctx))
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// # Map Step
    ///
    /// The `map` step transforms edges in the traversal by applying a mapping function to each edge.
    /// It returns an iterator that yields the transformed elements, allowing you to convert edge data
    /// into any desired format.
    ///
    /// ## Visual Diagram
    ///
    /// Before map step (with edges in traversal):
    /// ```text
    ///   [Person A] --- knows(2018)* ---> [Person B] --- created(2022)* ---> [Project]
    ///    ^                                         
    ///    |                                         
    ///   owns(2020)*                                       
    ///    |                                         
    ///   [Company]                                        
    /// ```
    ///
    /// After map step with `edge -> relationship description`:
    /// ```text
    ///   "knows since 2018", "created in 2022", "owns since 2020"
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `mapping`: A function that takes an edge reference and context, and returns a transformed value.
    ///   The function signature is `FnMut(Graph::EdgeReference<'graph>, Walker::Context) -> R`.
    ///
    /// ## Return Value
    ///
    /// Returns an iterator that yields the transformed elements, with the type determined by
    /// the return type of the mapping function.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/map.rs", edge_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - Perfect for extracting relationship data (e.g., edge types, weights, or properties)
    /// - Use pattern matching to handle different edge types appropriately
    /// - You can access connected vertices through the edge's tail() and head() methods
    /// - For analyzing graph connectivity, pair with edge-traversal steps like filter
    /// - The iterator chain can continue with standard Rust iterator methods after mapping
    pub fn map<R, M>(mut self, mut mapping: M) -> impl Iterator<Item = R> + 'graph
    where
        M: FnMut(Graph::EdgeReference<'graph>, Walker::Context) -> R + 'graph,
        Walker: 'graph,
    {
        EdgeReferenceIterImpl::new(self.graph(), self.walker())
            .map(move |(reference, ctx)| mapping(reference, ctx))
    }
}
