use crate::walker::builder::{EdgeWalkerBuilder, VertexWalkerBuilder};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use crate::graph::Graph;
use crate::ElementId;
use include_doc::function_body;
use std::marker::PhantomData;

// ================ PROBE IMPLEMENTATION ================

pub struct VertexProbe<'graph, Parent, Callback> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    callback: Callback,
}

impl<Parent, Callback> VertexProbe<'_, Parent, Callback> {
    pub(crate) fn new(parent: Parent, callback: Callback) -> Self {
        VertexProbe {
            _phantom_data: Default::default(),
            parent,
            callback,
        }
    }
}

impl<'graph, Parent, Callback> Walker<'graph> for VertexProbe<'graph, Parent, Callback>
where
    Parent: VertexWalker<'graph>,
    Callback: FnMut(&<Parent::Graph as Graph>::VertexReference<'_>),
{
    type Graph = Parent::Graph;
    type Context = Parent::Context;

    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }

    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent, Callback> VertexWalker<'graph> for VertexProbe<'graph, Parent, Callback>
where
    Parent: VertexWalker<'graph>,
    Callback: FnMut(&<Parent::Graph as Graph>::VertexReference<'_>),
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        let next = self.parent.next(graph);
        if let Some(id) = next {
            if let Some(vertex) = graph.vertex(id) {
                (self.callback)(&vertex);
            }
        }
        next
    }
}

pub struct EdgeProbe<'graph, Parent, Callback> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    callback: Callback,
}

impl<Parent, Callback> EdgeProbe<'_, Parent, Callback> {
    pub(crate) fn new(parent: Parent, callback: Callback) -> Self {
        EdgeProbe {
            _phantom_data: Default::default(),
            parent,
            callback,
        }
    }
}

impl<'graph, Parent, Callback> Walker<'graph> for EdgeProbe<'graph, Parent, Callback>
where
    Parent: EdgeWalker<'graph>,
    Callback: FnMut(&<Parent::Graph as Graph>::EdgeReference<'_>),
{
    type Graph = Parent::Graph;
    type Context = Parent::Context;

    fn next_element(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Edge)
    }

    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }
}

impl<'graph, Parent, Callback> EdgeWalker<'graph> for EdgeProbe<'graph, Parent, Callback>
where
    Parent: EdgeWalker<'graph>,
    Callback: FnMut(&<Parent::Graph as Graph>::EdgeReference<'_>),
{
    fn next(
        &mut self,
        graph: &'graph Self::Graph,
    ) -> Option<<Self::Graph as Graph>::EdgeId> {
        let next = self.parent.next(graph);
        if let Some(next) = next {
            let edge = graph.edge(next).expect("edge must exist");
            (self.callback)(&edge);
        }
        next
    }
}

impl<'graph, Mutability, Graph, Walker> VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    /// # Probe Step
    ///
    /// The `probe` step allows you to execute a callback function for each element in the traversal 
    /// without altering the traversal itself. This is useful for debugging, logging, or collecting 
    /// information during a traversal.
    ///
    /// ## Visual Diagram
    ///
    /// Before probe step:
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*                                        
    /// ```
    ///
    /// After probe step (unchanged, but callback executed for each *):
    /// ```text
    ///   [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
    ///    ^                                         
    ///    |                                         
    ///   edge3                                       
    ///    |                                         
    ///   [D]*                                        
    /// ```
    ///
    /// ## Parameters
    ///
    /// - `callback`: A function that takes a reference to the current element being traversed (vertex or edge). 
    ///   For vertices, the function signature is `FnMut(&Graph::VertexReference<'_>)` and for edges, 
    ///   it's `FnMut(&Graph::EdgeReference<'_>)`.
    ///
    /// ## Return Value
    ///
    /// A walker of the same type as the input with the probe operation added to the pipeline, 
    /// allowing for further chaining of operations.
    ///
    /// ## Example
    ///
    /// ```rust
    #[doc = function_body!("examples/probe.rs", example, [])]
    /// ```
    ///
    /// For more examples, see the [probe example](https://github.com/yourusername/graph-api/blob/main/graph-api-lib/examples/probe.rs).
    ///
    /// ## Notes
    ///
    /// - The `probe` step does not modify the traversal path or elements
    /// - The callback function is executed for each element as it's traversed
    /// - It's useful for debugging complex traversals without modifying the traversal logic
    /// - Side effects in the callback function (like printing or collecting statistics) do not affect the traversal
    /// - Can be used at multiple points in a traversal to monitor the flow at different stages
    /// - Consider using `probe` instead of creating temporary variables outside the traversal for debugging purposes
    pub fn probe<Callback>(
        self,
        callback: Callback,
    ) -> VertexWalkerBuilder<'graph, Mutability, Graph, VertexProbe<'graph, Walker, Callback>>
    where
        Callback: FnMut(&Graph::VertexReference<'_>),
    {
        VertexWalkerBuilder {
            _phantom: Default::default(),
            walker: VertexProbe::new(self.walker, callback),
            graph: self.graph,
        }
    }
}

impl<'graph, Mutability, Graph, Walker> EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: crate::graph::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    /// Executes a callback function for each edge in the traversal without altering the traversal.
    ///
    /// This is useful for debugging, logging, or collecting information during a traversal.
    ///
    /// See the documentation for [`VertexWalkerBuilder::probe`] for more details.
    pub fn probe<Callback>(
        self,
        callback: Callback,
    ) -> EdgeWalkerBuilder<'graph, Mutability, Graph, EdgeProbe<'graph, Walker, Callback>>
    where
        Callback: FnMut(&Graph::EdgeReference<'_>),
    {
        EdgeWalkerBuilder {
            _phantom: Default::default(),
            walker: EdgeProbe::new(self.walker, callback),
            graph: self.graph,
        }
    }
}