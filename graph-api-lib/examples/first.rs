use graph_api_lib::{EdgeSearch, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{populate_graph, Vertex, VertexExt};

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);
    example(graph);
}

fn example<Graph>(graph: Graph)
where
    Graph: graph_api_lib::Graph<Vertex = Vertex>,
{
    // Get the first person vertex
    let first_person = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .first();

    assert!(first_person.is_some());

    // Using first with edges
    let first_created_edge = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .edges(EdgeSearch::scan())
        .first();

    assert!(first_created_edge.is_some());
}
