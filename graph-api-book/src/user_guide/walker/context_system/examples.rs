use graph_api_lib::{EdgeReference, EdgeSearch, Graph, Supported, VertexReference};
use graph_api_book::standard_model::{Edge, Vertex, standard_populated_graph};

// ANCHOR: basic_context
pub fn basic_context_example() {
    let graph = standard_populated_graph();
    
    // Store person name in context and use it later
    let person_projects = graph.walk()
        .vertices()
        .filter_person() // Type-safe filter using generated helper
        .push_context(|v, _| {
            // Extract and store person's name
            v.project::<Person<_>>().unwrap().name().to_string()
        })
        .edges(EdgeSearch::scan().outgoing())
        .filter_created() // Type-safe filter using generated helper
        .vertices()
        .filter_project() // Type-safe filter using generated helper
        .map(|project, ctx| {
            // Format using project name and person name from context
            format!("{} created the {} project", 
                    ctx, 
                    project.project::<Project<_>>().unwrap().name())
        })
        .collect::<Vec<_>>();
}
// ANCHOR_END: basic_context

// ANCHOR: nested_context
pub fn nested_context_example() {
    let graph = standard_populated_graph();
    
    // Use nested context to track relationships
    let follows_ages = graph.walk()
        .vertices()
        .filter_person() // Type-safe filter using generated helper
        .push_context(|v, _| {
            // Store source person's age in context
            v.project::<Person<_>>().unwrap().age()
        })
        .detour(|w| {
            // Start a sub-traversal that keeps parent context
            w.edges(EdgeSearch::scan().outgoing())
                .filter_follows() // Type-safe filter using generated helper
                .vertices()
                .filter_person() // Type-safe filter using generated helper
                .map(|target, ctx| {
                    // Access source's age from context
                    let source_age = *ctx;
                    let target_age = target.project::<Person<_>>().unwrap().age();
                    
                    // Calculate and return age difference
                    let diff = target_age as i32 - source_age as i32;
                    
                    if diff > 0 {
                        "follows someone older"
                    } else if diff < 0 {
                        "follows someone younger"
                    } else {
                        "follows someone the same age"
                    }
                })
        })
        .collect::<Vec<_>>();
}
// ANCHOR_END: nested_context

// ANCHOR: default_context
pub fn default_context_example() {
    let graph = standard_populated_graph();
    
    // Use the built-in default context to access source vertex
    let relationships = graph.walk()
        .vertices()
        .filter_person() // Type-safe filter using generated helper
        .push_default_context() // Store current vertex in default context
        .edges(EdgeSearch::scan().outgoing())
        .vertices()
        .map(|target, ctx| {
            // Access the source vertex from context
            let source = ctx.vertex();
            let source_name = source.project::<Person<_>>()
                                  .map(|p| p.name())
                                  .unwrap_or("Unknown");
            
            // Format the relationship based on target type
            match target.weight() {
                Vertex::Person { name, .. } => 
                    format!("{} is connected to {}", source_name, name),
                Vertex::Project { name, .. } => 
                    format!("{} works on {}", source_name, name),
                _ => "Unknown relationship".to_string()
            }
        })
        .collect::<Vec<_>>();
}
// ANCHOR_END: default_context