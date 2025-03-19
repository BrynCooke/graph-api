use graph_api_lib::{EdgeReference, EdgeSearch};
use graph_api_lib::{Graph, VertexReference};
use graph_api_test::Edge;
use graph_api_test::EdgeExt;
use graph_api_test::Person;
use graph_api_test::Vertex;

/* ANCHOR: all */
// ANCHOR: knows_example
pub fn default_context_example<G>(graph: &G, bryn_id: G::VertexId, julia_id: G::VertexId)
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
// ANCHOR_END: knows_example

// ANCHOR: edge_properties
pub fn edge_properties_example<G>(graph: &G, person_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Find relationships with metadata
    let relationships = graph
        .walk()
        .vertices_by_id(vec![person_id])
        .push_default_context()
        .edges(EdgeSearch::scan().outgoing())
        .map(|edge, ctx| {
            // Get the source person's name
            let source_name = match ctx.vertex() {
                Vertex::Person { name, .. } => name.clone(),
                _ => "Unknown".to_string(),
            };

            // Format based on edge type
            match edge.weight() {
                Edge::Knows { since } => {
                    format!("{} has known someone since {}", source_name, since)
                }
                Edge::Created => {
                    format!("{} created something", source_name)
                }
                Edge::Language(lang) => {
                    format!("{} knows the {:?} language", source_name, lang)
                }
            }
        })
        .collect::<Vec<_>>();

    println!("Person relationships with metadata:");
    for rel in relationships {
        println!("- {}", rel);
    }
}
// ANCHOR_END: edge_properties
/* ANCHOR_END: all */
