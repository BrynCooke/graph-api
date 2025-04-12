use graph_api_lib::{
    EdgeSearch, EdgeWalker, EdgeWalkerBuilder, ElementId, Graph, VertexSearch, VertexWalker,
    VertexWalkerBuilder, Walker,
};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, Vertex, populate_graph};
use std::marker::PhantomData;

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);

    vertex_example(&graph);
    edge_example(&graph, refs.bryn);
}

// ================ STEP IMPLEMENTATION ================

// The PrintId struct for vertices
pub struct VertexPrintId<'graph, Parent> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
}

impl<Parent> VertexPrintId<'_, Parent> {
    pub(crate) fn new(parent: Parent) -> Self {
        VertexPrintId {
            _phantom_data: Default::default(),
            parent,
        }
    }
}

// Implement Walker trait for VertexPrintId
impl<'graph, Parent> Walker<'graph> for VertexPrintId<'graph, Parent>
where
    Parent: VertexWalker<'graph>,
{
    type Graph = Parent::Graph;
    type Context = Parent::Context;

    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Vertex)
    }

    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.parent.ctx_mut()
    }
}

// Implement VertexWalker trait for VertexPrintId
impl<'graph, Parent> VertexWalker<'graph> for VertexPrintId<'graph, Parent>
where
    Parent: VertexWalker<'graph>,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::VertexId> {
        let next = self.parent.next(graph);
        if let Some(id) = next {
            println!("Vertex ID: {:?}", id);
        }
        next
    }
}

// The PrintId struct for edges
pub struct EdgePrintId<'graph, Parent> {
    _phantom_data: PhantomData<&'graph ()>,
    parent: Parent,
}

impl<Parent> EdgePrintId<'_, Parent> {
    pub(crate) fn new(parent: Parent) -> Self {
        EdgePrintId {
            _phantom_data: Default::default(),
            parent,
        }
    }
}

// Implement Walker trait for EdgePrintId
impl<'graph, Parent> Walker<'graph> for EdgePrintId<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
{
    type Graph = Parent::Graph;
    type Context = Parent::Context;

    fn next_element(&mut self, graph: &'graph Self::Graph) -> Option<ElementId<Self::Graph>> {
        self.next(graph).map(ElementId::Edge)
    }

    fn ctx(&self) -> &Self::Context {
        self.parent.ctx()
    }

    fn ctx_mut(&mut self) -> &mut Self::Context {
        self.parent.ctx_mut()
    }
}

// Implement EdgeWalker trait for EdgePrintId
impl<'graph, Parent> EdgeWalker<'graph> for EdgePrintId<'graph, Parent>
where
    Parent: EdgeWalker<'graph>,
{
    fn next(&mut self, graph: &'graph Self::Graph) -> Option<<Self::Graph as Graph>::EdgeId> {
        let next = self.parent.next(graph);
        if let Some(id) = next {
            println!("Edge ID: {:?}", id);
        }
        next
    }
}

// ================ EXTENSION TRAITS ================

// Extension trait for the WalkerBuilder to add the print_id method
pub trait WalkerBuilderPrintIdExt<'graph, Mutability, Graph, Walker> {
    type Output;

    fn print_id(self) -> Self::Output;
}

// Implementation for VertexWalkerBuilder
impl<'graph, Mutability, Graph, Walker> WalkerBuilderPrintIdExt<'graph, Mutability, Graph, Walker>
    for VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: graph_api_lib::Graph,
    Walker: VertexWalker<'graph, Graph = Graph>,
{
    type Output = VertexWalkerBuilder<'graph, Mutability, Graph, VertexPrintId<'graph, Walker>>;

    fn print_id(self) -> Self::Output {
        self.with_vertex_walker(VertexPrintId::new)
    }
}

// Implementation for EdgeWalkerBuilder
impl<'graph, Mutability, Graph, Walker> WalkerBuilderPrintIdExt<'graph, Mutability, Graph, Walker>
    for EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
where
    Graph: graph_api_lib::Graph,
    Walker: EdgeWalker<'graph, Graph = Graph>,
{
    type Output = EdgeWalkerBuilder<'graph, Mutability, Graph, EdgePrintId<'graph, Walker>>;

    fn print_id(self) -> Self::Output {
        self.with_edge_walker(EdgePrintId::new)
    }
}

// ================ EXAMPLE USAGE ================

fn vertex_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    println!("\n=== Vertex PrintId Example ===");

    // Use our custom print_id step in the walker
    // First, bring the extension trait into scope
    use WalkerBuilderPrintIdExt;

    let count = graph
        .walk()
        .vertices(VertexSearch::scan())
        .print_id() // Our custom step that prints vertex IDs
        .count();

    println!("Total vertices processed: {}", count);
}

fn edge_example<G>(graph: &G, bryn_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    println!("\n=== Edge PrintId Example ===");

    // Use our custom print_id step in the walker
    // First, bring the extension trait into scope
    use WalkerBuilderPrintIdExt;

    let count = graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .edges(EdgeSearch::scan())
        .print_id() // Our custom step that prints edge IDs
        .count();

    println!("Total edges processed from Bryn: {}", count);
}
