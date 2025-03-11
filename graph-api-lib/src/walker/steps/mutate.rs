use crate::walker::builder::{EdgeWalkerBuilder, Mutable, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};
use include_doc::function_body;

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Mutate Step
    ///
    /// The `mutate` step allows you to modify the graph during traversal. For each element
    /// in the traversal (vertex or edge), the provided callback function is executed, giving
    /// you the ability to create, modify, or delete elements in the graph.
    ///
    /// ## Visual Diagram
    ///
    /// Before mutate step (traversal position on Person vertices):
    /// ```text
    ///   [Person A]*        [Project X]
    ///        |
    ///   [Person B]*
    /// ```
    ///
    /// After mutate step (adding 'Created' edges to Project X):
    /// ```text
    ///   [Person A]* ---- Created ----> [Project X]
    ///        |
    ///   [Person B]* ---- Created ----> [Project X]
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `callback`: A function that receives:
    ///   - A mutable reference to the graph
    ///   - The element ID (vertex ID or edge ID)
    ///   - The context for the current element
    ///
    /// ## Requirements
    ///
    /// - Must use `walk_mut()` instead of `walk()` to get a mutable graph reference
    /// - Works with both vertex and edge traversals
    /// - The traversal is collected before mutations are applied to avoid interference
    ///
    /// ## Return Value
    ///
    /// Returns the number of elements that were modified (traversed and passed to the callback).
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/mutate.rs", basic_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The `mutate` step doesn't change the traversal position or context
    /// - All modifications happen to the graph itself, not to the traversal
    /// - The traversal is completed first, then mutations are applied afterward to avoid traversal interference
    /// - Be cautious when performing mutations that might interact with each other
    /// - For better performance with large traversals, consider using `limit()` before `mutate()`
    /// - Can be used to implement complex graph algorithms like community detection, path finding, or graph rewriting
    /// - Related steps: `filter()` for conditional traversal, `collect()` for materializing results
    pub fn mutate<Callback>(mut self, callback: Callback) -> usize
    where
        Callback: Fn(&mut Walker::Graph, Graph::VertexId, &Walker::Context),
        Mutability: Mutable,
        'graph: 'graph,
    {
        let graph = self.graph.take_mut();
        let graph_copy: &Graph = unsafe { std::mem::transmute(&*graph) };
        let mut walker = self.walker;

        let mut contexts = Vec::new();
        while let Some(vertex_id) = walker.next(graph_copy) {
            let ctx = walker.ctx().clone();
            contexts.push((vertex_id, ctx));
        }

        let mut count = 0;
        for (vertex_id, ctx) in contexts {
            callback(graph, vertex_id, &ctx);
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
    /// # Mutate Step
    ///
    /// Allows you to modify the graph during edge traversal.
    ///
    /// See the documentation for [`VertexWalkerBuilder::mutate`] for more details.
    pub fn mutate<Callback>(mut self, callback: Callback) -> usize
    where
        Callback: Fn(&mut Walker::Graph, Graph::EdgeId, &Walker::Context),
        Mutability: Mutable,
        'graph: 'graph,
    {
        let graph = self.graph.take_mut();
        let graph_copy: &Graph = unsafe { std::mem::transmute(&*graph) };
        let mut walker = self.walker;

        let mut contexts = Vec::new();
        while let Some(edge) = walker.next(graph_copy) {
            let ctx = walker.ctx().clone();
            contexts.push((edge, ctx));
        }

        let mut count = 0;
        for (edge_id, ctx) in contexts {
            callback(graph, edge_id, &ctx);
            count += 1;
        }
        count
    }
}
