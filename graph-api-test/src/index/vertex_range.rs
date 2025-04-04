use crate::{Edge, PersonMut, Vertex, VertexIndex, assert_elements_eq, populate_graph};
use graph_api_lib::{Graph, SupportsVertexHashIndex, SupportsVertexRangeIndex, VertexReferenceMut};

pub fn test_index<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexRangeIndex + SupportsVertexHashIndex,
{
    let refs = populate_graph(graph);
    // Test range query for age between 20-40
    let collected = graph
        .walk()
        .vertices(VertexIndex::person_by_age_range(20..46))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);

    let collected = graph
        .walk()
        .vertices(VertexIndex::person_by_age(45))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}

pub fn test_index_remove<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexRangeIndex,
{
    let refs = populate_graph(graph);
    // Remove a vertex
    graph.remove_vertex(refs.bryn).expect("person must exist");

    // The vertex should no longer appear in the age range query
    assert_eq!(
        graph
            .walk()
            .vertices(VertexIndex::person_by_age_range(20..46))
            .count(),
        0
    );
}

pub fn test_index_update<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge> + SupportsVertexRangeIndex,
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
        .vertices(VertexIndex::person_by_age_range(20..46))
        .count();
    assert_eq!(collected, 0);

    let collected = graph
        .walk()
        .vertices(VertexIndex::person_by_age_range(100..106))
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}
