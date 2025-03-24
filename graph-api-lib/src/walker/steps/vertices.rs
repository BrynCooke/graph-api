use crate::graph::Graph;
use crate::search::vertex::VertexSearch;
use crate::walker::builder::VertexWalkerBuilder;
use crate::walker::{VertexWalker, Walker};
use crate::{ElementId, VertexReference};
use include_doc::function_body;
use std::marker::PhantomData;

// ================ VERTICES IMPLEMENTATION ================

pub struct Vertices<'search, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    current_iter: Option<<Parent::Graph as Graph>::VertexIter<'search, 'graph>>,
    vertex_search: VertexSearch<'search, Parent::Graph>,
}

impl<'search, 'graph, Parent> Vertices<'search, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    pub fn new(parent: Parent, vertex_search: VertexSearch<'search, Parent::Graph>) -> Self {
        Self {
            _phantom_data: Default::default(),
            parent,
            current_iter: None,
            vertex_search,
        }
    }
}

impl<'graph, Parent> Walker<'graph> for Vertices<'_, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
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

impl<'graph, Parent> VertexWalker<'graph> for Vertices<'_, 'graph, Parent>
where
    Parent: VertexWalker<'graph>,
    <Parent as Walker<'graph>>::Graph: 'graph,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        if self.current_iter.is_none() {
            self.current_iter = Some(graph.vertices(&self.vertex_search));
        }

        self.current_iter
            .as_mut()
            .expect("iterator must be populated")
            .next()
            .map(|next| next.id())
    }
}

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Vertices Step
    ///
    /// The `vertices` step is the primary entry point for graph traversals, allowing you to select a set of
    /// vertices to start traversing from. It accepts a `VertexSearch` parameter that specifies which
    /// vertices to include in the traversal.
    ///
    /// ## Visual Diagram
    ///
    /// Before vertices step (empty traversal):
    /// ```text
    ///   [A] --- edge1 ---> [B] --- edge2 ---> [C]  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]                                        
    /// ```
    ///
    /// After vertices step (with VertexSearch::scan()):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*                                        
    /// ```
    ///
    /// After vertices step (with VertexSearch::scan().with_label(Vertex::person_label())):
    /// ```text
    ///   [Person A]* --- edge1 ---> [Project B]  --- edge2 ---> [Person C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [Person D]*                                        
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `vertex_search`: A `VertexSearch` object that defines the criteria for selecting vertices
    ///
    /// ## Return Value
    ///
    /// Returns a traversal containing all vertices that match the search criteria.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/vertices.rs", example, [])]
    /// ```
    ///
    /// For more examples, see the [vertices example](https://github.com/yourusername/graph-api/blob/main/graph-api-lib/examples/vertices.rs).
    ///
    /// ## Notes
    ///
    /// - The `vertices` step is typically the first step in a traversal
    /// - Use `VertexIndex` methods for faster access when you have appropriate indexes defined
    /// - For more complex criteria, you can chain the `filter` step after this one
    /// - When working with large graphs, consider using indexed properties for better performance
    /// - This step supports all vertex search mechanisms, including label-based, index-based, and full scans
    /// - The traversal order is not guaranteed unless you specifically use an range index
    pub fn vertices<'a, T: Into<VertexSearch<'a, Graph>>>(
        self,
        vertex_search: T,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, Vertices<'a, 'graph, Walker>> {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: self.walker.vertices(vertex_search.into()),
            graph: self.graph,
        }
    }
}
