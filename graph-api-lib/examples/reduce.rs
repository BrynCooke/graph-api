use graph_api_lib::{EdgeReference, VertexReference};
use graph_api_lib::{EdgeSearch, Supported};
use graph_api_lib::{Graph, VertexSearch};
use graph_api_simplegraph::SimpleGraph;
use graph_api_test::Vertex;
use graph_api_test::VertexExt;
use graph_api_test::{Edge, Knows};
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
    G: Graph<Vertex = Vertex, Edge = Edge, SupportsVertexLabelIndex = Supported>,
{
    // Find the oldest person in the graph
    let oldest = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .reduce(
            |vertex, _| vertex.project::<Person<_>>().unwrap().age(),
            |acc, ctx, vertex, _vertex_ctx| {
                let acc_age = acc.project::<Person<_>>().unwrap().age();
                let vertex_age = vertex.project::<Person<_>>().unwrap().age();
                if vertex_age > acc_age {
                    *ctx = vertex_age;
                    vertex
                } else {
                    acc
                }
            },
        )
        .map(|vertex, ctx| format!("The oldest person is {:?}, age {}", vertex.id(), ctx))
        .next()
        .expect("should have got a result");

    println!("{}", oldest);
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
    // Find the edge with the highest "since" value between two edges
    let oldest_edge = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .edges(EdgeSearch::scan())
        .reduce(
            |edge, _| edge.project::<Knows<_>>().unwrap().since(),
            |acc, ctx, edge, _edge_ctx| {
                let acc_since = acc.project::<Knows<_>>().unwrap().since();
                let edge_since = edge.project::<Knows<_>>().unwrap().since();
                if edge_since > acc_since {
                    *ctx = edge_since;
                    edge
                } else {
                    acc
                }
            },
        )
        .map(|edge, ctx| {
            format!(
                "The edge with latest 'since' value is {:?}, since {}",
                edge.id(),
                ctx
            )
        })
        .next()
        .expect("should have got a result");

    println!("{}", oldest_edge);
}
