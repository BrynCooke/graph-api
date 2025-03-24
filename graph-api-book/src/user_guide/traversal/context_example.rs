use graph_api_lib::{Graph, Supported, VertexReference};
use graph_api_test::{Edge, EdgeIndex, Person, Vertex, VertexExt};
use std::ops::Deref;

// ANCHOR: all
// Context example showing how to calculate the total age of all friends
pub fn context_traversal_example<G>(graph: &G, person_id: G::VertexId)
where
    G: Graph<Vertex = Vertex, Edge = Edge, SupportsEdgeLabelIndex = Supported>,
{
    // Calculate total age of all friends of a person
    let _total_age = graph
        .walk()
        .vertices_by_id(vec![person_id])
        .edges(EdgeIndex::knows())
        .tail()
        .filter_person()
        .push_context(|v, _| {
            // Store age in context
            if let Some(person) = v.project::<Person<_>>() {
                person.age()
            } else {
                0
            }
        })
        .fold(0, |acc, _, age| acc + age.deref());
}
// ANCHOR_END: all
