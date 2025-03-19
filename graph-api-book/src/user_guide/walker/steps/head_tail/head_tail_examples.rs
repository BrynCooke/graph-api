use graph_api_lib::Graph;
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::{EdgeIndex, VertexIndex, populate_graph};

/* ANCHOR: all */
// Function demonstrating the head and tail steps
pub fn head_tail_examples() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    // ANCHOR: head_example
    // Find people who created projects
    // 1. Start with all people
    // 2. Find "created" edges they made
    // 3. Use head() to get back to those same people (the source vertices)
    let creators = graph
        .walk()
        .vertices(VertexIndex::person())
        .edges(EdgeIndex::created().outgoing())
        .head() // Move to the source vertex of the edge
        .collect::<Vec<_>>();

    println!("Found {} people who created projects", creators.len());
    // ANCHOR_END: head_example

    // ANCHOR: tail_example
    // Find projects created by people
    // 1. Start with all people
    // 2. Find "created" edges they made
    // 3. Use tail() to get to the projects (the target vertices)
    let projects = graph
        .walk()
        .vertices(VertexIndex::person())
        .edges(EdgeIndex::created().outgoing())
        .tail() // Move to the target vertex of the edge
        .collect::<Vec<_>>();

    println!("Found {} projects created by people", projects.len());
    // ANCHOR_END: tail_example

    // ANCHOR: multi_step
    // Find people who know someone who created a project
    // This demonstrates chaining head and tail multiple times
    let indirect_creators = graph
        .walk()
        .vertices(VertexIndex::person())
        .edges(EdgeIndex::knows().outgoing())
        .tail() // Move to the people they know
        .edges(EdgeIndex::created().outgoing())
        .tail() // Move to the projects created by those people
        .collect::<Vec<_>>();

    println!(
        "Found {} projects created by people the original people know",
        indirect_creators.len()
    );
    // ANCHOR_END: multi_step
}
/* ANCHOR_END: all */
