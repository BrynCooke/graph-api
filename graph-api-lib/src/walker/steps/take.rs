use crate::ElementId;
use crate::graph::Graph;
use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use include_doc::function_body;
use std::marker::PhantomData;

// ================ TAKE IMPLEMENTATION ================

pub struct VertexTake<'graph, Parent> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    limit: usize,
}

impl<Parent> VertexTake<'_, Parent> {
    pub(crate) fn new(parent: Parent, limit: usize) -> Self {
        Self {
            _phantom_data: Default::default(),
            parent,
            limit,
        }
    }
}

impl<'graph, Parent> Walker<'graph> for VertexTake<'graph, Parent>
where
    Parent: VertexWalker<'graph>,
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

impl<'graph, Parent> VertexWalker<'graph> for VertexTake<'graph, Parent>
where
    Parent: VertexWalker<'graph>,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        if self.limit > 0 {
            self.limit -= 1;
            self.parent.next(graph)
        } else {
            None
        }
    }
}

pub struct EdgeTake<'graph, Parent> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    limit: usize,
}

impl<Parent> EdgeTake<'_, Parent> {
    pub(crate) fn new(parent: Parent, limit: usize) -> Self {
        Self {
            _phantom_data: Default::default(),
            parent,
            limit,
        }
    }
}

impl<'graph, Parent> Walker<'graph> for EdgeTake<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
{
    type Graph = Parent::Graph;

    type Context = Parent::Context;
    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Edge)
    }
    fn ctx(&self) -> &Parent::Context {
        self.parent.ctx()
    }
    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.parent.ctx_mut()
    }
}

impl<'graph, Parent> EdgeWalker<'graph> for EdgeTake<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::EdgeId> {
        if self.limit > 0 {
            self.limit -= 1;
            self.parent.next(graph)
        } else {
            None
        }
    }
}

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Take Step
    ///
    /// The `take` step restricts a vertex traversal to return at most a specified number of vertices.
    /// This is useful for pagination, performance optimization, or when you only need a subset of results.
    ///
    /// ## Visual Diagram
    ///
    /// Before take step (with multiple vertices in traversal):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*                                        
    /// ```
    ///
    /// After take(2) step (only first 2 vertices remain in traversal):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]                                        
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `n`: A usize value specifying the maximum number of vertices to include in the traversal
    ///
    /// ## Return Value
    ///
    /// Returns a traversal containing at most the specified number of vertices.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/take.rs", vertex_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - The `take` step is generally applied after filtering operations but before terminal operations
    /// - It does not guarantee which vertices will be returned, just how many
    /// - For predictable results, combine with sorting operations or range indexes
    /// - Can significantly improve performance by avoiding unnecessary traversal
    /// - Particularly useful for large graphs where full traversal would be expensive
    /// - If the traversal contains fewer vertices than the limit, all vertices are returned
    /// - Different from `first()` which returns only a single vertex as an Option
    /// - Follows the naming convention of Rust's standard library Iterator::take
    pub fn take(
        self,
        n: usize,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexTake<'graph, Walker>> {
        self.with_vertex_walker(|walker| walker.take(n))
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// # Take Step
    ///
    /// The `take` step restricts an edge traversal to return at most a specified number of edges.
    /// This is useful for pagination, performance optimization, or when you only need a subset of edges.
    ///
    /// ## Visual Diagram
    ///
    /// Before take step (with multiple edges in traversal):
    /// ```text
    ///   [Person A] --- knows* ---> [Person B] --- created* ---> [Project]
    ///    ^                                         
    ///    |                                         
    ///   owns*                                       
    ///    |                                         
    ///   [Company]                                        
    /// ```
    ///
    /// After take(2) step (only first 2 edges remain in traversal):
    /// ```text
    ///   [Person A] --- knows* ---> [Person B] --- created* ---> [Project]
    ///    ^                                         
    ///    |                                         
    ///   owns                                       
    ///    |                                         
    ///   [Company]                                        
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `n`: A usize value specifying the maximum number of edges to include in the traversal
    ///
    /// ## Return Value
    ///
    /// Returns a traversal containing at most the specified number of edges.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/take.rs", edge_example, [])]
    /// ```
    ///
    /// ## Notes
    ///
    /// - Use take to avoid processing excessive numbers of connections in a dense graph
    /// - Improves performance for graphs with highly connected nodes
    /// - Particularly useful when you only need to analyze a sample of connections
    /// - The order of edges returned depends on the graph implementation
    /// - For pagination purposes, consider combining with sorting or other ordering mechanisms
    /// - Follows the naming convention of Rust's standard library Iterator::take
    pub fn take(
        self,
        n: usize,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeTake<'graph, Walker>> {
        self.with_edge_walker(|walker| walker.take(n))
    }
}
