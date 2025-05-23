use crate::{
    Edge, EdgeExt, Knows, Person, Vertex, VertexExt, assert_elements_eq, assert_elements_one_of,
    populate_graph,
};
use graph_api_lib::{EdgeReference, EdgeSearch, Graph, VertexReference, VertexSearch};

/// Test vertex reduce operations with the non-terminal reduce
pub fn test_vertices_reduce<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Populate the graph with test data
    let refs = populate_graph(graph);

    let oldest = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .reduce(|acc, vertex, _ctx| {
            let acc_age = acc.project::<Person<_>>().unwrap().age();
            let vertex_age = vertex.project::<Person<_>>().unwrap().age();
            if vertex_age > acc_age { vertex } else { acc }
        })
        .map(|vertex, _ctx| vertex.id())
        .next()
        .expect("should have got an element");

    // Verify the result exists
    assert_elements_eq!(graph, vec![oldest], vec![refs.julia]);
}

/// Test edge reduce operations with the non-terminal reduce
pub fn test_edges_reduce<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let edge_id = graph
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
        .map(|edge, _ctx| edge.id())
        .next()
        .expect("should have got an element");

    // Verify the result - in this case, both edges have the same since value (1999)
    // So the accumulated edge should be one of the two input edges
    assert_elements_one_of!(
        graph,
        vec![edge_id],
        vec![refs.bryn_knows_julia, refs.julia_knows_bryn]
    );
}
