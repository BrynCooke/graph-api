use graph_api_lib::{Graph, SupportsEdgeLabelIndex, SupportsVertexLabelIndex, VertexSearch};
use crate::standard_model::{Edge, Vertex, VertexExt};

// ANCHOR: all
// Function demonstrating the count step
pub fn count_example<G>(graph: G)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexLabelIndex + SupportsEdgeLabelIndex,
{
    // ANCHOR: basic_count
    // Basic count - how many people are in the graph?
    let person_count = graph.walk().vertices(Vertex::person()).count();

    println!("Total people in graph: {}", person_count);
    // ANCHOR_END: basic_count

    // ANCHOR: filtered_count
    // Count with filtering - how many people are over 30?
    let older_person_count = graph
        .walk()
        .vertices(Vertex::person())
        .filter_by_person(|person, _| person.age() > 30)
        .count();

    println!("People over 30: {}", older_person_count);
    // ANCHOR_END: filtered_count

    // ANCHOR: edge_count
    // Count relationships - how many 'follows' relationships exist?
    let knows_count = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(Edge::follows())
        .count();

    println!("Total 'follows' relationships: {}", knows_count);
    // ANCHOR_END: edge_count

    // ANCHOR: analytics
    // Count for analytics - average number of people followed per person
    let person_count = graph.walk().vertices(Vertex::person()).count();

    if person_count > 0 {
        let knows_count = graph
            .walk()
            .vertices(Vertex::person())
            .edges(Edge::follows().incoming())
            .count();

        let avg_known = knows_count as f64 / person_count as f64;
        println!("Average people followed per person: {:.2}", avg_known);
    }
    // ANCHOR_END: analytics
}
// ANCHOR_END: all
