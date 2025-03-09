use graph_api_lib::Graph;
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{populate_graph, Edge, EdgeIndex, Vertex};

type MyGraph = SimpleGraph<Vertex, Edge>;
type MyGraphVertexId = <SimpleGraph<Vertex, Edge> as Graph>::VertexId;
// We'll remove the unused type alias

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);
    example(graph, refs.bryn);
}

fn example(graph: MyGraph, bryn_id: MyGraphVertexId) {
    // Find all projects created by bryn
    // The tail() step returns the target vertices of edges
    let projects = graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .edges(EdgeIndex::created())
        .tail()
        .collect::<Vec<_>>();

    // Bryn should have created at least one project
    assert!(!projects.is_empty());

    // Find all friends of bryn (people they know)
    let friends = graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .edges(EdgeIndex::knows())
        .tail()
        .collect::<Vec<_>>();

    // Bryn should know at least one person
    assert!(!friends.is_empty());
}
