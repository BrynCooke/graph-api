use graph_api_lib::{Graph, SupportsEdgeLabelIndex, SupportsVertexLabelIndex, VertexSearch};
use graph_api_test::{Edge, Vertex, VertexExt};

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
    // Count relationships - how many 'knows' relationships exist?
    let knows_count = graph
        .walk()
        .vertices(VertexSearch::scan())
        .edges(Edge::knows())
        .count();

    println!("Total 'knows' relationships: {}", knows_count);
    // ANCHOR_END: edge_count

    // ANCHOR: analytics
    // Count for analytics - average number of people known per person
    let person_count = graph.walk().vertices(Vertex::person()).count();

    if person_count > 0 {
        let knows_count = graph
            .walk()
            .vertices(Vertex::person())
            .edges(Edge::knows().incoming())
            .count();

        let avg_known = knows_count as f64 / person_count as f64;
        println!("Average people known per person: {:.2}", avg_known);
    }
    // ANCHOR_END: analytics
}
// ANCHOR_END: all
