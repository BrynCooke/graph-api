use graph_api_lib::{Graph, SupportsVertexLabelIndex};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::Edge;
use graph_api_test::Project;
use graph_api_test::Vertex;
use graph_api_test::VertexIndex;
use graph_api_test::populate_graph;

fn main() {
    // Create a mutable graph for testing
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    basic_example(&mut graph);
}

fn basic_example<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexLabelIndex,
{
    // Add a new project node to demonstrate mutations
    let new_project_id = graph.add_vertex(Vertex::Project(Project {
        name: "NewProject".to_string(),
    }));

    // Add 'Created' edges from people to a project using mutate
    let mutations_count = graph
        .walk_mut() // Must use walk_mut for mutations
        .vertices(VertexIndex::person())
        .mutate(|graph, person_id, _| {
            // Add a 'Created' edge from each person to the new project
            graph.add_edge(person_id, new_project_id, Edge::Created);
        });

    // Should have created edges for all people in the graph (at least 2)
    assert!(mutations_count >= 2);
    println!(
        "Created {} edges connecting people to the project",
        mutations_count
    );
}
