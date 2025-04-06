use crate::standard_model::{standard_populated_graph, Edge, Vertex, VertexExt};
use graph_api_lib::{EdgeSearch, Graph};

// ANCHOR: all
pub fn head_example() {
    let graph = standard_populated_graph();

    // Find the first person vertex to start the traversal
    let Some(start_person_id) = graph.walk().vertices(Vertex::person()).first() else {
        println!("No person found in the graph to start traversal.");
        return;
    };

    // ANCHOR: head_example
    // Find the projects created by people known by the starting person
    let projects: Vec<_> = graph
        .walk()
        .vertices_by_id([start_person_id]) // Start at a specific person
        .edges(EdgeSearch::scan().outgoing()) // Follow outgoing edges
        .filter_knows() // Keep only 'Knows' edges
        .head() // Move to the target vertices (people known by the start person)
        .edges(EdgeSearch::scan().outgoing()) // Follow outgoing edges from these people
        .filter_created() // Keep only 'Created' edges
        .head() // Move to the target vertices (projects created by known people)
        .map(|project, _| {
            // Extract project name
            project
                .project::<graph_api_test::Project<_>>()
                .map(|p| p.name().to_string())
                .unwrap_or_else(|| "Not a project".to_string())
        })
        .collect();

    println!(
        "Projects created by people known by the starting person ({:?}):",
        start_person_id
    );
    if projects.is_empty() {
        println!("  No such projects found.");
    } else {
        for project_name in projects {
            println!("  - {}", project_name);
        }
    }
    // ANCHOR_END: head_example
}
// ANCHOR_END: all
