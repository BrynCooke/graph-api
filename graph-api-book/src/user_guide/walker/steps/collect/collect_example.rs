use graph_api_lib::{Graph, Supported, VertexReference};
use graph_api_test::{Person, Vertex, VertexIndex};
use std::collections::{BTreeSet, HashSet};

/* ANCHOR: all */
// Function demonstrating how to use the collect step
pub fn collect_example<G>(graph: G)
where
    G: Graph<Vertex = Vertex, SupportsVertexLabelIndex = Supported>,
{
    // ANCHOR: collect_vec
    // Collect results into a Vec
    let person_vertices: Vec<_> = graph
        .walk()
        .vertices(VertexIndex::person())
        .collect::<Vec<_>>();

    println!("Found {} person vertices", person_vertices.len());
    // ANCHOR_END: collect_vec

    // ANCHOR: collect_hashset
    // Collect into a HashSet for unique elements
    let unique_names: HashSet<String> = graph
        .walk()
        .vertices(VertexIndex::person())
        // Use map to extract properties from each person
        .map(|person, _| {
            // Use projection to access Person methods
            person
                .project::<Person<_>>()
                .map(|p| p.name().to_string())
                .unwrap_or_else(|| "Unknown".to_string())
        })
        .collect::<HashSet<String>>();

    println!("Found {} unique person names", unique_names.len());
    // ANCHOR_END: collect_hashset

    // ANCHOR: collect_btreeset
    // Collect into a BTreeSet for ordered unique elements
    let ordered_ages: BTreeSet<u64> = graph
        .walk()
        .vertices(VertexIndex::person())
        // Extract age from each Person vertex
        .map(|person, _| {
            // Use projection to access Person methods
            person.project::<Person<_>>().map(|p| p.age()).unwrap_or(0)
        })
        .collect::<BTreeSet<u64>>();

    // Print ages in ascending order (BTreeSet maintains order)
    println!("Person ages (ordered): {:?}", ordered_ages);
    // ANCHOR_END: collect_btreeset
}
/* ANCHOR_END: all */