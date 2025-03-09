use graph_api_lib::EdgeSearch;
use graph_api_lib::Graph;
use graph_api_lib::VertexReference;
use graph_api_lib::VertexSearch;
use graph_api_lib::{EdgeReference, Supported};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::populate_graph;
use graph_api_test::Edge;
use graph_api_test::EdgeExt;
use graph_api_test::EdgeIndex;
use graph_api_test::Person;
use graph_api_test::Project;
use graph_api_test::Vertex;
use graph_api_test::VertexExt;
use graph_api_test::VertexIndex;

fn main() {
    // Create a new graph
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);

    vertex_example(&graph);
    edge_example(&graph);
}

fn vertex_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge, SupportsVertexLabelIndex = Supported>,
{
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
    // Average age should be reasonable for our test data
    assert!(average_age > 20 && average_age < 100);
    println!("Average age of people: {}", average_age);
}

fn edge_example<G>(graph: &G)
where
    G: Graph<
        Vertex = Vertex,
        Edge = Edge,
        SupportsVertexLabelIndex = Supported,
        SupportsEdgeLabelIndex = Supported,
    >,
{
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

    // There should be at least one relationship in our test graph
    assert!(earliest_relation.is_some());
    if let Some(year) = earliest_relation {
        println!("Earliest relationship started in: {}", year);
    }
}
