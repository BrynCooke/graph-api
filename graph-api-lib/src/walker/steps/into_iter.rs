use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker};
use include_doc::function_body;

/// # Iter Step
///
/// While there is no explicit `iter` step method in the walker API, the walker builders implement
/// the `IntoIterator` trait, allowing you to convert a traversal into a standard Rust iterator
/// with the `.into_iter()` method. This enables using standard iterator methods like `map`,
/// `filter`, and `fold` on your graph traversal results.
///
/// ## Visual Diagram
///
/// Before iter (walker traversal):
/// ```text
///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
///    ^                                         
///    |                                         
///   edge3                                       
///    |                                         
///   [D]*                                        
/// ```
///
/// After iter (converted to standard Rust iterator):
/// ```text
///   Iterator<Item = VertexId>  or  Iterator<Item = EdgeId>
///   A -> B -> C -> D
/// ```
///
/// When using `map()`, the iterator yields references with contexts:
/// ```text
///   Iterator<Item = (reference, context)>
///   [A, ctx] -> [B, ctx] -> [C, ctx] -> [D, ctx]
/// ```
///
/// ## Parameters
///
/// None (when using `.into_iter()`)
///
/// ## Return Value
///
/// An iterator that yields element IDs (vertex or edge IDs, depending on the walker type).
///
/// When using the `map()` method, the iterator yields tuples of `(reference, context)` where:
/// - `reference` is either a vertex or edge reference depending on the walker type
/// - `context` contains any accumulated context data from the traversal
///
/// ## Example
///
/// ```rust
#[doc = function_body!("examples/into_iter.rs", example, [])]
/// ```
///
/// For more examples, see the [into_iter example](https://github.com/yourusername/graph-api/blob/main/graph-api-lib/examples/into_iter.rs).
///
/// ## Notes
///
/// - Using `.into_iter()` consumes the walker and returns an iterator over element IDs
/// - This is the bridge between Graph API's walker system and Rust's standard iterator ecosystem
/// - After converting to an iterator, you can use all standard Rust iterator methods
/// - Prefer using walker steps for graph traversal logic, and iterator methods for post-traversal processing
impl<'graph, Mutability, Graph, Walker> IntoIterator
    for VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    type Item = Graph::VertexId;
    type IntoIter = VertexIterImpl<'graph, Graph, Walker>;

    fn into_iter(mut self) -> Self::IntoIter {
        VertexIterImpl::new(self.graph(), self.walker())
    }
}

impl<'graph, Mutability, Graph, Walker> IntoIterator
    for EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    type Item = Graph::EdgeId;
    type IntoIter = EdgeIterImpl<'graph, Graph, Walker>;

    fn into_iter(mut self) -> Self::IntoIter {
        EdgeIterImpl::new(self.graph(), self.walker())
    }
}

pub struct VertexReferenceIterImpl<'graph, Graph, Walker> {
    graph: &'graph Graph,
    walker: Walker,
}

impl<'graph, Graph, Walker> VertexReferenceIterImpl<'graph, Graph, Walker> {
    pub(crate) fn new(graph: &'graph Graph, walker: Walker) -> Self
    where
        Graph: crate::graph::Graph,
        Walker: VertexWalker<'graph, Graph = Graph>,
    {
        Self { graph, walker }
    }
}

impl<'graph, Graph, Walker> Iterator for VertexReferenceIterImpl<'graph, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    type Item = (Graph::VertexReference<'graph>, Walker::Context);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.walker.next(self.graph) {
            Some((
                self.graph
                    .vertex(next)
                    .expect("vertex ID must resolve to vertex"),
                self.walker.ctx().clone(),
            ))
        } else {
            None
        }
    }
}

pub struct EdgeReferenceIterImpl<'graph, Graph, Walker> {
    graph: &'graph Graph,
    walker: Walker,
}

impl<'graph, Graph, Walker> EdgeReferenceIterImpl<'graph, Graph, Walker> {
    pub(crate) fn new(graph: &'graph Graph, walker: Walker) -> Self
    where
        Graph: crate::graph::Graph,
        Walker: EdgeWalker<'graph, Graph = Graph>,
    {
        Self { graph, walker }
    }
}

impl<'graph, Graph, Walker> Iterator for EdgeReferenceIterImpl<'graph, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    type Item = (Graph::EdgeReference<'graph>, Walker::Context);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.walker.next(self.graph) {
            let edge = self
                .graph
                .edge(next)
                .expect("edge ID must resolve to an edge");
            Some((edge, self.walker.ctx().clone()))
        } else {
            None
        }
    }
}

pub struct VertexIterImpl<'graph, Graph, Walker> {
    graph: &'graph Graph,
    walker: Walker,
}

impl<'graph, Graph, Walker> VertexIterImpl<'graph, Graph, Walker> {
    pub(crate) fn new(graph: &'graph Graph, walker: Walker) -> Self
    where
        Graph: crate::graph::Graph,
        Walker: VertexWalker<'graph, Graph = Graph>,
    {
        Self { graph, walker }
    }
}

impl<'graph, Graph, Walker> Iterator for VertexIterImpl<'graph, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    type Item = Graph::VertexId;

    fn next(&mut self) -> Option<Self::Item> {
        self.walker.next(self.graph)
    }
}

pub struct EdgeIterImpl<'graph, Graph, Walker> {
    graph: &'graph Graph,
    walker: Walker,
}

impl<'graph, Graph, Walker> EdgeIterImpl<'graph, Graph, Walker> {
    pub(crate) fn new(graph: &'graph Graph, walker: Walker) -> Self
    where
        Graph: crate::graph::Graph,
        Walker: EdgeWalker<'graph, Graph = Graph>,
    {
        Self { graph, walker }
    }
}

impl<'graph, Graph, Walker> Iterator for EdgeIterImpl<'graph, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    type Item = Graph::EdgeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.walker.next(self.graph)
    }
}
