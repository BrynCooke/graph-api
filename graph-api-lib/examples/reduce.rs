use graph_api_lib::{EdgeReference, VertexReference};
use graph_api_lib::{EdgeSearch, SupportsVertexLabelIndex, SupportsEdgeLabelIndex};
use graph_api_lib::{Graph, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::Vertex;
use graph_api_test::{Edge, Knows};
use graph_api_test::{EdgeExt, VertexExt};
use graph_api_test::{Person, populate_graph};

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
    // Find the oldest person in the graph
    let oldest = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .reduce(|acc, vertex, _ctx| {
            let acc_age = acc.project::<Person<_>>().unwrap().age();
            let vertex_age = vertex.project::<Person<_>>().unwrap().age();
            if vertex_age > acc_age { vertex } else { acc }
        })
        .map(|vertex, _ctx| {
            let age = vertex.project::<Person<_>>().unwrap().age();
            format!("The oldest person is {:?}, age {}", vertex.id(), age)
        })
        .next()
        .expect("should have got a result");

    println!("{}", oldest);
}

fn edge_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>
        + SupportsVertexLabelIndex
        + SupportsEdgeLabelIndex,
{
    // Find the edge with the highest "since" value between two edges
    let oldest_edge = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .edges(EdgeSearch::scan())
        .filter_knows()
        .reduce(|acc, edge, _ctx| {
            let acc_since = acc.project::<Knows<_>>().unwrap().since();
            let edge_since = edge.project::<Knows<_>>().unwrap().since();
            if edge_since > acc_since { edge } else { acc }
        })
        .map(|edge, _ctx| {
            let since = edge.project::<Knows<_>>().unwrap().since();
            format!(
                "The edge with latest 'since' value is {:?}, since {}",
                edge.id(),
                since
            )
        })
        .next()
        .expect("should have got a result");

    println!("{}", oldest_edge);
}
