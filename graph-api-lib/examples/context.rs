use graph_api_lib::EdgeReference;
use graph_api_lib::EdgeSearch;
use graph_api_lib::Graph;
use graph_api_lib::SupportsEdgeLabelIndex;
use graph_api_lib::VertexReference;
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

    // Run both examples
    vertex_context_example(&graph, refs.bryn, refs.julia);
    edge_context_example(&graph, refs.bryn);
}

fn vertex_context_example<G>(graph: &G, bryn_id: G::VertexId, julia_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
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
    println!("Vertex Context Example - Relationships found:");
    for relationship in &knows {
        println!("- {}", relationship);
    }
}

fn edge_context_example<G>(graph: &G, person_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsEdgeLabelIndex,
{
    // Walk the graph starting from the person vertex
    let edge_types = graph
        .walk()
        .vertices_by_id(vec![person_id])
        .edges(EdgeSearch::scan().outgoing())
        .push_context(|edge, _ctx| {
            // Determine edge type based on the edge type
            let edge_type = match edge.weight() {
                Edge::Created => "Created",
                Edge::Knows { .. } => "Knows",
                Edge::Language(_) => "Language",
            };

            // Return the edge type as context
            edge_type
        })
        .map(|_v, c| *c)
        .collect::<Vec<_>>();

    println!("{:?}", edge_types);
}
