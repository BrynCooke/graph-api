use graph_api_lib::{Graph, SupportsVertexLabelIndex, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Vertex, populate_graph};

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);
    example(graph);
}

fn example<G>(graph: G)
where
    G: Graph<Vertex = Vertex> + SupportsVertexLabelIndex,
{
    // The vertices() step is the starting point for most graph traversals
    // It finds vertices matching the given search criteria

    // Get all vertices in the graph using a scan
    let all_vertices = graph
        .walk()
        .vertices(VertexSearch::scan())
        .collect::<Vec<_>>();

    // Should find vertices in the graph
    assert!(!all_vertices.is_empty());

    // Get vertices with a specific label using an index
    let people = graph.walk().vertices(Vertex::person()).collect::<Vec<_>>();

    // Should find person vertices
    assert!(!people.is_empty());
}
