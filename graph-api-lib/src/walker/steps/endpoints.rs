use crate::graph::{EdgeReference, Graph};
use crate::walker::{EdgeWalker, VertexWalker, Walker};
use std::marker::PhantomData;
use crate::ElementId;

/// # Endpoint Type
///
/// Identifies which endpoint of an edge to navigate to.
///
/// - `Head`: The target/destination vertex of a directed edge
/// - `Tail`: The source/origin vertex of a directed edge
pub enum End {
    /// The head (target/destination) of a directed edge
    Head,
    /// The tail (source/origin) of a directed edge
    Tail,
}

/// # Endpoints Walker
///
/// Internal implementation for the head() and tail() steps.
/// This walker traverses from edges to their endpoint vertices.
///
/// ## Visual Diagram
///
/// For a Tail traversal:
/// ```text
///   [Person A] --- edge1* ---> [Person B]
///    ^
///    |
///   edge2*
///    |
///   [Person C]
/// ```
///
/// After endpoints step:
/// ```text
///   [Person A]* --- edge1 ---> [Person B]
///    
///   [Person C]*
/// ```
///
/// For a Head traversal the target vertices would be selected instead.
pub struct Endpoints<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
{
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
    end: End,
}

impl<'graph, Parent> Endpoints<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
{
    /// Creates a new Endpoints walker that navigates to either the head 
    /// or tail of each edge in the parent walker
    pub(crate) fn new(parent: Parent, end: End) -> Endpoints<'graph, Parent> {
        Self {
            _phantom_data: Default::default(),
            parent,
            end,
        }
    }
}

impl<'search, 'graph, Parent> Walker<'graph> for Endpoints<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
    <Parent as Walker<'graph>>::Context: Clone + 'static,
    <Parent as Walker<'graph>>::Graph: 'graph,
    <Parent::Graph as Graph>::EdgeIter<'search, 'graph>:
        Iterator<Item = <Parent::Graph as Graph>::EdgeReference<'graph>>,
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

impl<'search, 'graph, Parent> VertexWalker<'graph> for Endpoints<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
    <Parent as Walker<'graph>>::Context: Clone + 'static,
    <Parent as Walker<'graph>>::Graph: 'graph,
    <Parent::Graph as Graph>::EdgeIter<'search, 'graph>:
        Iterator<Item = <Parent::Graph as Graph>::EdgeReference<'graph>>,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        self.parent.next(graph).map(|e| match &self.end {
            End::Head =>
                graph.edge(e).expect("edge must exist").head(),
            End::Tail => graph.edge(e).expect("edge must exist").tail(),
        })
    }
}