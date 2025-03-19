use graph_api_lib::{EdgeSearch, Graph, VertexSearch};
use graph_api_test::{Edge, Person, Vertex};

// Multi-step traversal example
pub fn multi_step_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Find friends of friends who are over 30
    let friends_of_friends = graph.walk()
        .vertices(VertexSearch::index(Person::by_name_index(), "Alice"))
        .edges(EdgeSearch::scan().with_label(Edge::knows_label()))
        .tail() // Now at Alice's friends
        .edges(EdgeSearch::scan().with_label(Edge::knows_label()))
        .tail() // Now at friends of friends
        .filter(|v, _| {
            // Only keep people over 30
            v.project::<Person<_>>().unwrap().age() > 30
        })
        .collect::<Vec<_>>();
}