use graph_api_lib::{Graph, Supported, VertexSearch};
use graph_api_test::{Vertex, VertexExt, VertexIndex};

// Function demonstrating the filter step for narrowing traversal results
pub fn filter_example<G>(graph: G)
where
    G: Graph<Vertex = Vertex, SupportsVertexLabelIndex = Supported>,
{
    // Basic filtering - Keep only vertices that match a condition
    // This finds people with age > 30
    let older_people = graph
        .walk()
        .vertices(VertexIndex::person()) // Start with all people
        .filter_by_person(|person, _| {
            person.age() > 30 // Keep only people older than 30
        })
        .collect::<Vec<_>>();

    println!("Found {} people older than 30", older_people.len());

    // Filtering by pattern matching - Keep only certain vertex types
    // This finds all projects in the graph
    let only_projects = graph
        .walk()
        .vertices(VertexSearch::scan()) // Start with all vertices
        .filter_project() // Use the generated filter extension method
        .collect::<Vec<_>>();

    println!("Found {} projects", only_projects.len());

    // Chaining multiple filters - useful for complex conditions
    let filtered_people = graph
        .walk()
        .vertices(VertexIndex::person())
        .filter_by_person(|person, _| {
            // We can combine multiple conditions in a single filter
            person.age() >= 18 && // Adults only
            person.name().starts_with("B") // Names starting with B
        })
        .collect::<Vec<_>>();

    println!(
        "Found {} adults with names starting with B",
        filtered_people.len()
    );
}
