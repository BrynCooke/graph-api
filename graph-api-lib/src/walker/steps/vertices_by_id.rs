use crate::ElementId;
use crate::graph::Graph;
use crate::walker::builder::{StartWalkerBuilder, VertexWalkerBuilder};
use crate::walker::steps::Empty;
use crate::walker::{VertexWalker, Walker};
use include_doc::function_body;
use std::marker::PhantomData;
// ================ VERTEX_ITER IMPLEMENTATION ================

pub struct VertexIter<'graph, Parent, Iter>
where
    Parent: VertexWalker<'graph>,
    Iter: Iterator<Item = <Parent::Graph as Graph>::VertexId>,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    start: Iter,
}

impl<'graph, Parent, Iter> VertexIter<'graph, Parent, Iter>
where
    Parent: VertexWalker<'graph>,
    Iter: Iterator<Item = <Parent::Graph as Graph>::VertexId>,
{
    pub fn new(parent: Parent, start: Iter) -> Self {
        Self {
            _phantom_data: Default::default(),
            parent,
            start,
        }
    }
}

impl<'graph, Parent, Iter> Walker<'graph> for VertexIter<'graph, Parent, Iter>
where
    Parent: VertexWalker<'graph>,
    Iter: Iterator<Item = <Parent::Graph as Graph>::VertexId>,
{
    type Graph = Parent::Graph;

    type Context = Parent::Context;
    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }
    fn ctx(&self) -> &Parent::Context {
        self.parent.ctx()
    }
    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.parent.ctx_mut()
    }
}

impl<'graph, Parent, Iter> VertexWalker<'graph> for VertexIter<'graph, Parent, Iter>
where
    Parent: VertexWalker<'graph>,
    Iter: Iterator<Item = <Parent::Graph as Graph>::VertexId>,
{
    fn next(&mut self, _graph: &Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        self.start.next()
    }
}

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Vertices By ID Step
    ///
    /// The `vertices_by_id` step allows you to begin a traversal from a specific set of vertex IDs.
    /// This is useful when you already know the IDs of the vertices you want to include in your traversal.
    ///
    /// ## Visual Diagram
    ///
    /// Before vertices_by_id step (empty traversal):
    /// ```text
    ///   [A] --- edge1 ---> [B] --- edge2 ---> [C]  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]                                        
    /// ```
    ///
    /// After vertices_by_id step (with [id_A, id_C]):
    /// ```text
    ///   [A]* --- edge1 ---> [B] --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]                                        
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `vertex_ids`: An iterator that yields vertex IDs to include in the traversal
    ///
    /// ## Return Value
    ///
    /// Returns a traversal containing all vertices with the specified IDs.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/vertices_by_id.rs", example, [])]
    /// ```
    ///
    /// For more examples, see the [vertices_by_id example](https://github.com/yourusername/graph-api/blob/main/graph-api-lib/examples/vertices_by_id.rs).
    ///
    /// ## Notes
    ///
    /// - This step is efficient when you already know the exact IDs of vertices you want to work with
    /// - The order of vertices in the traversal will match the order of IDs in the input iterator
    /// - For vertices that don't exist in the graph, they will be skipped without error
    /// - This step is often used after a previous traversal has produced vertex IDs of interest
    /// - When working with a large number of IDs, consider using a `HashSet` for deduplication if needed
    pub fn vertices_by_id<Iter>(
        self,
        vertex_ids: Iter,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexIter<'graph, Walker, Iter::IntoIter>>
    where
        Iter: IntoIterator<Item = Graph::VertexId>,
    {
        self.with_vertex_walker(|walker| walker.vertices_by_id(vertex_ids))
    }
}

impl<'graph, Graph, Mutability, Context> StartWalkerBuilder<'graph, Mutability, Graph, Context>
where
    Graph: crate::graph::Graph,
    Context: Clone + 'static,
{
    pub fn vertices_by_id<Iter>(
        self,
        vertex_ids: Iter,
    ) -> VertexWalkerBuilder<
        'graph,
        Mutability,
        Graph,
        VertexIter<'graph, Empty<Graph, Context>, Iter::IntoIter>,
    >
    where
        Iter: IntoIterator<Item = Graph::VertexId>,
    {
        crate::walker::builder::new(self.graph, self.empty.vertices_by_id(vertex_ids))
    }
}
