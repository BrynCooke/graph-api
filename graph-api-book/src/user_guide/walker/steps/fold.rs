use graph_api_lib::Graph;
use graph_api_lib::{EdgeReference, Supported};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::Edge;
use graph_api_test::EdgeIndex;
use graph_api_test::Vertex;
use graph_api_test::VertexIndex;
use graph_api_test::populate_graph;

pub fn vertex_example() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    // Calculate the average age of people
    let (total_age, person_count) =
        graph
            .walk()
            .vertices(VertexIndex::person())
            .fold((0, 0), |(total, count), vertex, _| {
                if let Vertex::Person { age, .. } = vertex.weight() {
                    (total + age, count + 1)
                } else {
                    (total, count)
                }
            });

    let average_age = if person_count > 0 {
        total_age / person_count
    } else {
        0
    };
    println!("Average age of people: {}", average_age);
}

pub fn edge_example() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    // Working with edges - find the earliest relationship year
    let earliest_relation = graph
        .walk()
        .vertices(VertexIndex::person())
        .edges(EdgeIndex::knows())
        .fold(None, |earliest, edge, _| {
            if let Edge::Knows { since } = edge.weight() {
                match earliest {
                    None => Some(*since),
                    Some(year) if *since < year => Some(*since),
                    _ => earliest,
                }
            } else {
                earliest
            }
        });

    // Display the result
    if let Some(year) = earliest_relation {
        println!("Earliest relationship started in: {}", year);
    }
}