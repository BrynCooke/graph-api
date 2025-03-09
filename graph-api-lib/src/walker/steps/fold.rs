use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};
use include_doc::function_body;

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Fold Step
    ///
    /// The `fold` step allows you to accumulate a result by processing each element in a traversal. 
    /// This is similar to the standard Rust `fold` operation but works directly on graph traversals.
    ///
    /// ## Visual Diagram
    ///
    /// Before fold step (traversal position on vertices):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*                                        
    /// ```
    ///
    /// During fold step (processing each element with accumulator):
    /// ```text
    ///   Accumulator: Init -> [A] -> [B] -> [C] -> [D] -> Final Result
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `init`: The initial value for the accumulator
    /// - `f`: A closure that takes:
    ///   - The current accumulator value
    ///   - A reference to the current element (vertex or edge)
    ///   - The current element's context
    ///   - Returns the updated accumulator value
    ///
    /// ## Return Value
    ///
    /// Returns the final accumulated value after processing all elements in the traversal.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/fold.rs", vertex_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The fold step is a terminal operation - it consumes the traversal and returns a value
    /// - Unlike map, fold doesn't yield a stream of values but a single accumulated result
    /// - The closure is called once for each element with the accumulator and element
    /// - Can be used for many common operations like counting, summing, finding min/max, etc.
    /// - More flexible than specialized steps like count when you need to calculate custom aggregates
    /// - The accumulator can be any type that matches your needs
    /// - Context is available if you need it for your calculations
    pub fn fold<Acc, F>(mut self, init: Acc, mut f: F) -> Acc 
    where
        F: FnMut(Acc, Graph::VertexReference<'graph>, &Walker::Context) -> Acc,
        'graph: 'graph,
    {
        let graph = self.graph.take();
        let mut walker = self.walker;
        let mut acc = init;
        
        while let Some(vertex_id) = walker.next(graph) {
            let vertex = graph.vertex(vertex_id).expect("vertex ID must resolve to vertex");
            acc = f(acc, vertex, walker.ctx());
        }
        
        acc
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// # Fold Step
    ///
    /// Accumulates a result by processing each edge in the traversal.
    ///
    /// See the documentation for [`VertexWalkerBuilder::fold`] for more details.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/fold.rs", edge_example, [])]
    /// ```
    pub fn fold<Acc, F>(mut self, init: Acc, mut f: F) -> Acc 
    where
        F: FnMut(Acc, Graph::EdgeReference<'graph>, &Walker::Context) -> Acc,
        'graph: 'graph,
    {
        let graph = self.graph.take();
        let mut walker = self.walker;
        let mut acc = init;
        
        while let Some(edge_id) = walker.next(graph) {
            let edge = graph.edge(edge_id).expect("edge ID must resolve to edge");
            acc = f(acc, edge, walker.ctx());
        }
        
        acc
    }
}