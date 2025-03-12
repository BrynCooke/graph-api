use crate::{Edge, PersonMut, Vertex, VertexIndex, assert_elements_eq, populate_graph};
use graph_api_lib::{Graph, Supported, VertexReferenceMut};

/// Tests that a vertex can be added to the graph and that the indexed field
/// search returns the added vertex.
///
/// This function creates a vertex, adds it to the graph, and then verifies
/// that a search for the indexed field of the added vertex returns the
/// expected result.
pub fn test_index<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge, SupportsVertexIndex = Supported>,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices(VertexIndex::person_by_name("Bryn"))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
    graph.remove_vertex(refs.bryn);
}

/// Tests that a vertex can be removed from the graph, and that the indexed field
/// search no longer returns the removed vertex.
///
/// This function creates a vertex, adds it to the graph, and then removes it.
/// It then verifies that a search for the indexed field of the removed vertex
/// returns an empty result.
pub fn test_index_remove<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge, SupportsVertexIndex = Supported>,
{
    let refs = populate_graph(graph);
    graph.remove_vertex(refs.bryn);
    assert_eq!(
        graph
            .walk()
            .vertices(VertexIndex::person_by_name("Bryn"))
            .count(),
        0
    );
}

pub fn test_index_update<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge, SupportsVertexIndex = Supported>,
{
    let refs = populate_graph(graph);
    graph
        .vertex_mut(refs.bryn)
        .expect("bryn must exist")
        .project_mut::<PersonMut<_, _>>()
        .expect("must be a person")
        .set_name("Dyllan".to_string());
    assert_eq!(
        graph
            .walk()
            .vertices(VertexIndex::person_by_name("Bryn"))
            .count(),
        0
    );
    assert_eq!(
        graph
            .walk()
            .vertices(VertexIndex::person_by_name("Dyllan"))
            .count(),
        1
    );
}
