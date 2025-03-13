use graph_api_lib::Graph;
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, Vertex, VertexIndex, EdgeIndex, populate_graph};

pub fn detour_example() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    // Find projects created by people over 30
    let projects = graph
        .walk()
        .vertices(VertexIndex::person())
        .detour(|person_walker| {
            // This sub-traversal filters people by age
            person_walker
                .filter(|vertex, _| {
                    if let Vertex::Person { age, .. } = vertex.weight() {
                        *age > 30
                    } else {
                        false
                    }
                })
        })
        .edges(EdgeIndex::created())
        .tail()
        .map(|project, _| {
            if let Vertex::Project(project) = project.weight() {
                project.name.clone()
            } else {
                "Unknown".to_string()
            }
        })
        .collect::<Vec<_>>();

    // Display results
    println!("Projects created by people over 30:");
    for project in &projects {
        println!("- {}", project);
    }
}