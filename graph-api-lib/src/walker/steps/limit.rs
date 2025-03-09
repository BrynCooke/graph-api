use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use crate::graph::Graph;
use crate::ElementId;
use include_doc::function_body;
use std::marker::PhantomData;

// ================ LIMIT IMPLEMENTATION ================

pub struct VertexLimit<'graph, Parent> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    limit: usize,
}

impl<Parent> VertexLimit<'_, Parent> {
    pub(crate) fn new(parent: Parent, limit: usize) -> Self {
        Self {
            _phantom_data: Default::default(),
            parent,
            limit,
        }
    }
}

impl<'graph, Parent> Walker<'graph> for VertexLimit<'graph, Parent>
where
    Parent: VertexWalker<'graph>,
{
    type Graph = Parent::Graph;

    type Context = Parent::Context;
    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }

    fn ctx(&self) -> &Parent::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent> VertexWalker<'graph> for VertexLimit<'graph, Parent>
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

pub struct EdgeLimit<'graph, Parent> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    limit: usize,
}

impl<Parent> EdgeLimit<'_, Parent> {
    pub(crate) fn new(parent: Parent, limit: usize) -> Self {
        Self {
            _phantom_data: Default::default(),
            parent,
            limit,
        }
    }
}

impl<'graph, Parent> Walker<'graph> for EdgeLimit<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
{
    type Graph = Parent::Graph;

    type Context = Parent::Context;
    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Edge)
    }
    fn ctx(&self) -> &Parent::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent> EdgeWalker<'graph> for EdgeLimit<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
{
    fn next(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<<Self::Graph as Graph>::EdgeId> {
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
    /// # Limit Step
    ///
    /// The `limit` step restricts a traversal to return at most a specified number of elements. 
    /// This is useful for pagination, performance optimization, or when you only need a subset of results.
    ///
    /// ## Visual Diagram
    ///
    /// Before limit step (with multiple elements in traversal):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*                                        
    /// ```
    ///
    /// After limit(2) step (only first 2 elements remain in traversal):
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
    /// - `limit`: A usize value specifying the maximum number of elements to include in the traversal
    ///
    /// ## Return Value
    ///
    /// Returns a traversal containing at most the specified number of elements.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/limit.rs", example, [])]
    /// ```
    ///
    /// For more examples, see the [limit example](https://github.com/yourusername/graph-api/blob/main/graph-api-lib/examples/limit.rs).
    ///
    /// ## Notes
    ///
    /// - The `limit` step is generally applied after other filtering operations
    /// - It does not guarantee which elements will be returned, just how many
    /// - For predictable results, combine with sorting operations or ordered indexes
    /// - Can significantly improve performance by avoiding unnecessary traversal
    /// - Particularly useful for large graphs where full traversal would be expensive
    /// - If the traversal contains fewer elements than the limit, all elements are returned
    /// - Different from `first()` which returns only a single element as an Option
    pub fn limit(
        self,
        limit: usize,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexLimit<'graph, Walker>> {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.limit(limit),
            graph: self.graph,
        }
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// Limits the number of edges returned by this traversal to at most the specified number.
    ///
    /// This is useful for pagination, performance optimization, or when you only need a subset of results.
    ///
    /// See the documentation for [`VertexWalkerBuilder::limit`] for more details.
    pub fn limit(
        self,
        limit: usize,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeLimit<'graph, Walker>> {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.limit(limit),
            graph: self.graph,
        }
    }
}