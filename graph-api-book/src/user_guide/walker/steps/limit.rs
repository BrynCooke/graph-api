use graph_api_lib::{Graph, Supported};
use graph_api_test::{Vertex, VertexExt, VertexIndex};

// Function demonstrating the limit step
pub fn limit_example<G>(graph: G)
where
    G: Graph<Vertex = Vertex, SupportsVertexLabelIndex = Supported>,
{
    // Basic limit usage - get at most 5 vertices
    let some_people = graph
        .walk()
        .vertices(VertexIndex::person())
        .limit(5) // Get at most 5 people
        .collect::<Vec<_>>();

    println!("Retrieved {} people (limit 5)", some_people.len());

    // Limiting with filter - get first 3 people over age 30
    let some_older_people = graph
        .walk()
        .vertices(VertexIndex::person())
        .filter_by_person(|person, _| person.age() > 30)
        .limit(3) // Get at most 3 matching people
        .collect::<Vec<_>>();

    println!(
        "Retrieved {} people over 30 (limit 3)",
        some_older_people.len()
    );

    // Limiting for pagination - skip 5, take 5 (for page 2)
    // Note: For real pagination, you would use a more sophisticated approach
    let all_projects = graph
        .walk()
        .vertices(VertexIndex::project())
        .collect::<Vec<_>>();

    let page_size = 5;
    let page_number = 2;
    let offset = (page_number - 1) * page_size;

    let page_2_projects = all_projects
        .into_iter()
        .skip(offset)
        .take(page_size)
        .collect::<Vec<_>>();

    println!("Page 2 contains {} projects", page_2_projects.len());
}
