use graph_api_lib::Graph;
use graph_api_lib::VertexSearch;
use graph_api_lib::{EdgeSearch, Supported};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::Edge;
use graph_api_test::Vertex;
use graph_api_test::{populate_graph, EdgeIndex};

fn main() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);

    example(&graph, refs.bryn);
}

fn example<G>(graph: &G, bryn_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge, SupportsEdgeLabelIndex = Supported>,
{
    // Count projects that Bryn created using detour
    let bryn_project_count = graph
        .walk()
        .vertices_by_id(vec![bryn_id])
        .push_context(|_, _| 0) // Start with count 0
        .detour(|waypoint| {
            // For each person, find outgoing created edges and count them
            waypoint
                .edges(EdgeIndex::created().outgoing())
                .map(|_, count| *count + 1) // Increment count for each created edge
        })
        .map(|_, count| *count)
        .collect::<Vec<_>>();

    // Bryn should have at least one project
    assert_eq!(bryn_project_count.len(), 1);
    println!("Bryn has {} project(s)", bryn_project_count[0]);
}
