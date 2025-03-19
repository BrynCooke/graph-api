use graph_api_lib::{EdgeSearch, Graph, VertexSearch};
use graph_api_test::{Edge, Person, Project, Vertex};

// Basic traversal example
pub fn basic_walker_example<G>(graph: &G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Find all projects created by Alice
    let alice_projects = graph.walk()
        .vertices(VertexSearch::index(Person::by_name_index(), "Alice"))
        .edges(EdgeSearch::scan().with_label(Edge::created_label()))
        .tail()
        .collect::<Vec<_>>();
}