#[cfg(feature = "vertex-full-text-index")]
use crate::{PersonMut, assert_elements_eq, populate_graph};
#[cfg(feature = "vertex-full-text-index")]
use graph_api_lib::VertexReferenceMut;

use crate::{Edge, Vertex};
use graph_api_lib::Graph;

/// Tests that a vertex can be added to the graph and that the indexed field
/// search returns the added vertex.
///
/// This function creates a vertex, adds it to the graph, and then verifies
/// that a search for the indexed field of the added vertex returns the
/// expected result.
#[cfg(feature = "vertex-full-text-index")]
pub fn test_index<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsVertexFullTextIndex,
{
    let refs = populate_graph(graph);
    let collected = graph
        .walk()
        .vertices(Vertex::person_by_biography("graph"))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}
#[cfg(not(feature = "vertex-full-text-index"))]
pub fn test_index<T>(_graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
}

/// Tests that a vertex can be removed from the graph, and that the indexed field
/// search no longer returns the removed vertex.
///
/// This function creates a vertex, adds it to the graph, and then removes it.
/// It then verifies that a search for the indexed field of the removed vertex
/// returns an empty result.
#[cfg(all(feature = "vertex-full-text-index", feature = "element-removal"))]
pub fn test_index_remove<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>
        + graph_api_lib::SupportsVertexFullTextIndex
        + graph_api_lib::SupportsElementRemoval,
{
    let refs = populate_graph(graph);
    graph.remove_vertex(refs.bryn);
    assert_eq!(
        graph
            .walk()
            .vertices(Vertex::person_by_biography("graph"))
            .count(),
        0
    );
}

#[cfg(not(all(feature = "vertex-full-text-index", feature = "element-removal")))]
pub fn test_index_remove<T>(_graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
}

/// Tests that a vertex's indexed field can be updated and that the graph's
/// search functionality reflects this change.
///
/// This function creates a vertex, adds it to the graph, updates the indexed
/// field of the vertex, and then performs searches to verify that:
///
/// - The old value of the indexed field is no longer present in the graph
/// - The new value of the indexed field is correctly associated
///   with the updated vertex
#[cfg(feature = "vertex-full-text-index")]
pub fn test_index_update<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsVertexFullTextIndex,
{
    let refs = populate_graph(graph);
    graph
        .vertex_mut(refs.bryn)
        .expect("bryn must exist")
        .project_mut::<PersonMut<_, _>>()
        .expect("must be a person")
        .set_biography("Developed a graphql proxy in Rust".to_string());
    assert_eq!(
        graph
            .walk()
            .vertices(Vertex::person_by_biography("graph"))
            .count(),
        0
    );
    assert_eq!(
        graph
            .walk()
            .vertices(Vertex::person_by_biography("proxy"))
            .count(),
        1
    );
}

#[cfg(not(feature = "vertex-full-text-index"))]
pub fn test_index_update<T>(_graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
}
