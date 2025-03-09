use graph_api_lib::{Graph, VertexReference, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{populate_graph, Edge, Vertex, VertexExt};

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);
    example(graph);
}

fn example<G>(graph: G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>
{
    // Filter to keep only vertices with a specific type 
    let people = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .collect::<Vec<_>>();

    // Should find people (bryn and julia)
    assert!(!people.is_empty());

    // Filter based on a property in the vertex
    let projects = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter(|v, _| {
            matches!(v.weight(), Vertex::Project(p) if p.name == "GraphApi")
        })
        .collect::<Vec<_>>();

    // Should find the GraphApi project
    assert!(!projects.is_empty());
}