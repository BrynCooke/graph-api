use graph_api_lib::EdgeSearch;
use graph_api_lib::Graph;
use graph_api_lib::VertexReference;
use graph_api_lib::VertexSearch;
use graph_api_lib::{EdgeReference, Supported};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::populate_graph;
use graph_api_test::Edge;
use graph_api_test::EdgeExt;
use graph_api_test::Person;
use graph_api_test::Project;
use graph_api_test::Vertex;
use graph_api_test::VertexExt;
use std::ops::Deref;

fn main() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with some test data
    let refs = populate_graph(&mut graph);

    basic_example(graph, refs.bryn, refs.julia);
}

fn basic_example<G>(graph: G, bryn_id: G::VertexId, julia_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge, SupportsEdgeLabelIndex = Supported>,
{
    // Use push_default_context to make source vertex information available during traversal
    let knows: Vec<_> = graph
        .walk()
        .vertices_by_id(vec![bryn_id, julia_id])
        .push_default_context()
        .edges(EdgeSearch::scan().outgoing())
        .filter_knows()
        .head()
        .map(|target, ctx| {
            if let Vertex::Person { name, .. } = ctx.vertex() {
                format!(
                    "{} knows {}",
                    name,
                    target.project::<Person<_>>().unwrap().name()
                )
            } else {
                "Not a person".to_string()
            }
        })
        .collect::<Vec<_>>();

    // Check the results - should have 2 person descriptions
    assert_eq!(knows.len(), 2);
    println!("Relationships found:");
    for relationship in &knows {
        println!("- {}", relationship);
    }
}
