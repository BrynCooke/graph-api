use graph_api_lib::EdgeSearch;
use graph_api_lib::{Graph, VertexReference};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::Edge;
use graph_api_test::EdgeExt;
use graph_api_test::Person;
use graph_api_test::Vertex;
use graph_api_test::populate_graph;

fn main() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with some test data
    let refs = populate_graph(&mut graph);

    basic_example(&graph, refs.bryn, refs.julia);
}

fn basic_example<G>(graph: &G, bryn_id: G::VertexId, julia_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Use default context to access vertex information directly from prior in the traversal
    let knows = graph
        .walk()
        .vertices_by_id(vec![bryn_id, julia_id])
        .push_default_context()
        .edges(EdgeSearch::scan().outgoing())
        .filter_knows()
        .head()
        .map(|target_vertex, ctx| {
            // Access source person name from context
            let source_name = match ctx.vertex() {
                Vertex::Person { name, .. } => name.clone(),
                _ => "Unknown".to_string(),
            };

            // Access target person name from vertex
            let person = target_vertex.project::<Person<_>>().unwrap();

            format!("{} knows {}", source_name, person.name())
        })
        .collect::<Vec<_>>();

    // Check the results
    println!("Relationships found:");
    for relationship in &knows {
        println!("- {}", relationship);
    }
}
