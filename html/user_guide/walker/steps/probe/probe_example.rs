use crate::standard_model::{Person, Project, Vertex, standard_populated_graph};
use graph_api_lib::{EdgeReference, EdgeSearch, Graph, VertexReference};

// ANCHOR: all
// Function demonstrating how to use the probe step
pub fn probe_example() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: probe_vertices
    // Use probe to inspect vertices during traversal without affecting the flow
    println!("Examining each person vertex:");

    let mut person_count = 0;

    graph
        .walk()
        .vertices(Vertex::person()) // Type-safe vertex lookup by label
        .probe(|vertex, _| {
            // Inspect vertex properties using type-safe projection
            if let Some(person) = vertex.project::<Person<_>>() {
                person_count += 1;
                println!("  Found person: {} (age {})", person.name(), person.age());
            }
        })
        .count();

    println!("  Total people found: {}", person_count);
    // ANCHOR_END: probe_vertices

    // ANCHOR: probe_edges
    // Use probe to inspect relationships between vertices
    println!("\nExamining Bryn's outgoing relationships:");

    graph
        .walk()
        .vertices(Vertex::person_by_username("bryn123"))
        .edges(EdgeSearch::scan().outgoing())
        .probe(|edge, _| {
            // Get the target vertex to display relationship context
            let target = graph.vertex(edge.head()).unwrap();

            // Display relationship information
            match edge.weight() {
                crate::standard_model::Edge::Created => {
                    if let Some(project) = target.project::<Project<_>>() {
                        println!("  Bryn created project: {}", project.name());
                    }
                }
                crate::standard_model::Edge::Follows => {
                    if let Some(person) = target.project::<Person<_>>() {
                        println!("  Bryn follows: {}", person.name());
                    }
                }
                crate::standard_model::Edge::Liked { timestamp } => {
                    if let Some(project) = target.project::<Project<_>>() {
                        println!("  Bryn liked project {} on {}", project.name(), timestamp);
                    }
                }
                crate::standard_model::Edge::Commented { timestamp } => {
                    if let Some(project) = target.project::<Project<_>>() {
                        println!(
                            "  Bryn commented on project {} on {}",
                            project.name(),
                            timestamp
                        );
                    }
                }
            }
        })
        .count();
    // ANCHOR_END: probe_edges
}
// ANCHOR_END: all
