use graph_api_lib::EdgeReference;
use graph_api_lib::Graph;
use graph_api_lib::VertexReference;
use graph_api_lib::{SupportsEdgeLabelIndex, SupportsVertexLabelIndex};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::Edge;
use graph_api_test::Vertex;
use graph_api_test::populate_graph;

fn main() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let _refs = populate_graph(&mut graph);

    vertex_example(&graph);
    edge_example(&graph);
}

fn vertex_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexLabelIndex,
{
    // Calculate the average age of people
    let (total_age, person_count) =
        graph
            .walk()
            .vertices(Vertex::person())
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
    // Average age should be reasonable for our test data
    assert!(average_age > 20 && average_age < 100);
    println!("Average age of people: {}", average_age);
}

fn edge_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexLabelIndex + SupportsEdgeLabelIndex,
{
    // Working with edges - find the earliest relationship year
    let earliest_relation = graph
        .walk()
        .vertices(Vertex::person())
        .edges(Edge::knows())
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

    // There should be at least one relationship in our test graph
    assert!(earliest_relation.is_some());
    if let Some(year) = earliest_relation {
        println!("Earliest relationship started in: {}", year);
    }
}
