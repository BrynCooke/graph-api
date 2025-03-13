use graph_api_lib::{Graph, Supported};
use graph_api_test::{Vertex, VertexExt, VertexIndex};

// Function demonstrating the first step
pub fn first_example<G>(graph: G)
where
    G: Graph<Vertex = Vertex, SupportsVertexLabelIndex = Supported>,
{
    // Get the first person in the graph (if any)
    let first_person = graph.walk().vertices(VertexIndex::person()).first();

    match first_person {
        Some(person) => println!("Found a person: {:?}", person),
        None => println!("No people in the graph"),
    }

    // Get the first person with a specific name
    let first_alice = graph
        .walk()
        .vertices(VertexIndex::person())
        .filter_by_person(|person, _| person.name().contains("Alice"))
        .first();

    match first_alice {
        Some(alice) => println!("Found Alice: {:?}", alice),
        None => println!("No one named Alice in the graph"),
    }

    // Use first to check if any element matches a condition
    let has_young_person = graph
        .walk()
        .vertices(VertexIndex::person())
        .filter_by_person(|person, _| person.age() < 25)
        .first()
        .is_some();

    println!(
        "Graph {} people under 25",
        if has_young_person {
            "contains"
        } else {
            "doesn't contain"
        }
    );
}
