use graph_api_lib::{Graph, Supported};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{Edge, EdgeIndex, Vertex, VertexIndex, populate_graph};

pub fn head_tail_example() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    // ------------------------------
    // Head Step Example - Get sources
    // ------------------------------

    // Find all vertices that created a project
    // First find all projects, then their incoming Created edges,
    // then use head() to move to the source vertices (people who created projects)
    let project_creators = graph
        .walk()
        .vertices(VertexIndex::project()) // Start with all projects
        .edges(EdgeIndex::created().incoming()) // Find incoming Created edges
        .head() // Navigate to source vertices (creators)
        .collect::<Vec<_>>();

    println!("Found {} project creators", project_creators.len());

    // ------------------------------
    // Tail Step Example - Get targets
    // ------------------------------

    // Find all projects created by people
    // First find all people, then their outgoing Created edges,
    // then use tail() to move to the target vertices (projects they created)
    let created_projects = graph
        .walk()
        .vertices(VertexIndex::person()) // Start with all people
        .edges(EdgeIndex::created().outgoing()) // Find outgoing Created edges
        .tail() // Navigate to target vertices (projects)
        .collect::<Vec<_>>();

    println!("Found {} created projects", created_projects.len());

    // ------------------------------
    // Combined Example - Navigate through multiple relationships
    // ------------------------------

    // Find "friends of friends" (people who know people that a person knows)
    // This demonstrates a 2-level traversal
    let friends_of_friends = graph
        .walk()
        .vertices(VertexIndex::person())
        .filter(|person, _| {
            if let Vertex::Person(p) = person.weight() {
                p.name.contains("Alice") // Start with people named Alice
            } else {
                false
            }
        })
        .edges(EdgeIndex::knows().outgoing()) // Find people Alice knows
        .tail() // Go to those people (Alice's friends)
        .edges(EdgeIndex::knows().outgoing()) // Find people they know
        .tail() // Go to those people (friends of Alice's friends)
        .collect::<Vec<_>>();

    println!("Found {} friends of friends", friends_of_friends.len());
}
