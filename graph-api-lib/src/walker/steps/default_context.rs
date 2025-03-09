use crate::graph::{EdgeReference, VertexReference};
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::steps::{ContextRef, DefaultEdgeContext, DefaultVertexContext, EdgeContext, VertexContext};
use crate::walker::{EdgeWalker, VertexWalker};
use include_doc::function_body;

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Default Context Step
    ///
    /// The `push_default_context` step is a specialized version of the `push_context` step that 
    /// automatically stores the current element's ID and data in the context. This provides a 
    /// convenient way to preserve information about elements as you traverse through the graph.
    ///
    /// ## Visual Diagram
    ///
    /// Before push_default_context step (traversal with regular elements):
    /// ```text
    ///   [Person A]* --- created ---> [Project X]*  
    ///    |
    ///   knows
    ///    |
    ///   [Person B]*
    /// ```
    ///
    /// After push_default_context step (elements with default context):
    /// ```text
    ///   [Person A]* + {vertex_id: id_a, vertex: Person{name: "A", age: 30}} --- created ---> [Project X]* + {vertex_id: id_x, vertex: Project{name: "X"}}
    ///    |
    ///   knows
    ///    |
    ///   [Person B]* + {vertex_id: id_b, vertex: Person{name: "B", age: 25}}
    /// ```
    ///
    /// ## Parameters
    ///
    /// None
    ///
    /// ## Return Value
    ///
    /// Returns a traversal with the same elements, but with each element's ID and data stored in its context.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/default_context.rs", basic_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - Default context for vertices includes:
    ///   - `vertex_id`: The ID of the vertex
    ///   - `vertex`: The vertex data (cloned from the graph)
    /// - Type safety is maintained as the vertex/edge types are preserved
    /// - This step requires that your vertex/edge types implement Clone + 'static
    /// - More concise than manual context handling for common use cases
    /// - Especially useful when you need to preserve information across multiple traversal steps
    /// - Combines well with other context operations for complex data gathering
    pub fn push_default_context(
        self,
    ) -> VertexWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        VertexContext<
            'graph,
            Walker,
            impl Fn(
                &Graph::VertexReference<'_>,
                &Walker::Context,
            )
                -> ContextRef<DefaultVertexContext<Graph::VertexId, Graph::Vertex>, Walker::Context>,
            ContextRef<DefaultVertexContext<Graph::VertexId, Graph::Vertex>, Walker::Context>,
        >,
    >
    where
        Graph::Vertex: Clone + 'static,
    {
        self.push_context(|vertex, _context| DefaultVertexContext {
            vertex_id: vertex.id(),
            vertex: vertex.weight().clone(),
        })
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
    <Walker as crate::walker::Walker<'graph>>::Context: Clone + 'static,
{
    /// # Default Context Step
    ///
    /// A specialized version of the `push_context` step for edges that automatically 
    /// stores the current edge's ID and data in the context.
    ///
    /// See the documentation for [`VertexWalkerBuilder::push_default_context`] for more details.
    pub fn push_default_context(
        self,
    ) -> EdgeWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        EdgeContext<
            'graph,
            Walker,
            impl Fn(
                &Graph::EdgeReference<'_>,
                &Walker::Context,
            )
                -> ContextRef<DefaultEdgeContext<Graph::EdgeId, Graph::Edge>, Walker::Context>,
            ContextRef<DefaultEdgeContext<Graph::EdgeId, Graph::Edge>, Walker::Context>,
        >,
    >
    where
        Graph::Edge: Clone + 'static,
    {
        self.push_context(|edge, _context| DefaultEdgeContext {
            edge_id: edge.id(),
            edge: edge.weight().clone(),
        })
    }
}