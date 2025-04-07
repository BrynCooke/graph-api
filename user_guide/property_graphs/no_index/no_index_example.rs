use crate::standard_model::{Vertex, standard_populated_graph};
use graph_api_lib::{Graph, VertexReference, VertexSearch};

// ANCHOR: all
// ANCHOR: non_indexed_fields
// Function explaining non-indexed fields
pub fn non_indexed_fields() {
    // In the standard model, several fields are intentionally left without indexes:
    //
    // 1. Person::name - Non-indexed field requiring full scan for searches
    // 2. Person::unique_id - UUID field that doesn't need indexing for typical usage
    // 3. Project::name - Non-indexed field in the Project variant
    // 4. All fields in Comment variant - None are indexed
    //
    // Non-indexed fields can still be searched, but require a full graph scan
    // which is less efficient than using indexed lookups
}
// ANCHOR_END: non_indexed_fields

// Example showing how to scan the entire graph without indexes
pub fn scan_example() {
    // Use the standard graph defined in standard_model.rs
    let graph = standard_populated_graph();

    // ANCHOR: scan_for_name
    // INEFFICIENT: Find all people named "Bryn" by scanning
    // Since name is not indexed, we must scan all vertices
    let bryn_vertices = graph
        .walk()
        .vertices(VertexSearch::scan()) // Must scan ALL vertices
        .filter(|vertex, _| {
            // Manual pattern matching and filtering
            if let Vertex::Person { name, .. } = vertex.weight() {
                name == "Bryn"
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    println!("Found {} vertices for Bryn", bryn_vertices.len());
    // ANCHOR_END: scan_for_name

    // ANCHOR: scan_for_project
    // INEFFICIENT: Find all projects with a specific name by scanning
    // Since Project::name is not indexed, we must scan all vertices
    let graphapi_projects = graph
        .walk()
        .vertices(VertexSearch::scan()) // Must scan ALL vertices
        .filter(|vertex, _| {
            // Manual pattern matching and filtering
            if let Vertex::Project { name } = vertex.weight() {
                name == "GraphApi"
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    println!("Found {} GraphApi projects", graphapi_projects.len());
    // ANCHOR_END: scan_for_project

    // ANCHOR: comparison_scan
    // COMPARISON: Using an index vs. not using an index
    // 1. Inefficient: Find people with a specific username using a scan
    let start_scan = std::time::Instant::now();
    let _julia_by_scan = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter(|vertex, _| {
            if let Vertex::Person { username, .. } = vertex.weight() {
                username == "julia456"
            } else {
                false
            }
        })
        .collect::<Vec<_>>();
    let scan_duration = start_scan.elapsed();
    // ANCHOR_END: comparison_scan

    // ANCHOR: comparison_index
    // 2. Efficient: Find the same person using the username index
    let start_index = std::time::Instant::now();
    let _julia_by_index = graph
        .walk()
        .vertices(Vertex::person_by_username("julia456"))
        .collect::<Vec<_>>();
    let index_duration = start_index.elapsed();

    println!("Scan duration: {:?}", scan_duration);
    println!("Index duration: {:?}", index_duration);
    println!(
        "Speedup factor: {:.2}x",
        scan_duration.as_nanos() as f64 / index_duration.as_nanos() as f64
    );
    // ANCHOR_END: comparison_index
}
// ANCHOR_END: all
