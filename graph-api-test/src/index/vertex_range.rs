#[cfg(feature = "vertex-range-index")]
use crate::{PersonMut, assert_elements_eq, populate_graph};
#[cfg(feature = "vertex-range-index")]
use graph_api_lib::VertexReferenceMut;

use crate::{Edge, Vertex};
use graph_api_lib::Graph;

#[cfg(feature = "vertex-range-index")]
pub fn test_index<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsVertexRangeIndex,
{
    let refs = populate_graph(graph);
    // Test range query for age between 20-40
    let collected = graph
        .walk()
        .vertices(Vertex::person_by_age_range(20..46))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}

#[cfg(not(feature = "vertex-range-index"))]
pub fn test_index<T>(_graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
}

#[cfg(all(feature = "vertex-range-index", feature = "element-removal"))]
pub fn test_index_remove<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>
        + graph_api_lib::SupportsVertexRangeIndex
        + graph_api_lib::SupportsElementRemoval,
{
    let refs = populate_graph(graph);
    // Remove a vertex
    graph.remove_vertex(refs.bryn).expect("person must exist");

    // The vertex should no longer appear in the age range query
    assert_eq!(
        graph
            .walk()
            .vertices(Vertex::person_by_age_range(20..46))
            .count(),
        0
    );
}

#[cfg(not(all(feature = "element-removal", feature = "vertex-range-index")))]
pub fn test_index_remove<T>(_graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
}

#[cfg(feature = "vertex-range-index")]
pub fn test_index_update<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + graph_api_lib::SupportsVertexRangeIndex,
{
    let refs = populate_graph(graph);
    graph
        .vertex_mut(refs.bryn)
        .expect("person must exist")
        .project_mut::<PersonMut<_, _>>()
        .expect("person")
        .set_age(100);

    // Test range query for age between 20-40
    let collected = graph
        .walk()
        .vertices(Vertex::person_by_age_range(20..46))
        .count();
    assert_eq!(collected, 0);

    let collected = graph
        .walk()
        .vertices(Vertex::person_by_age_range(100..106))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}

#[cfg(not(feature = "vertex-range-index"))]
pub fn test_index_update<T>(_graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
}
