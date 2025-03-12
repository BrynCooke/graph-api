use crate::{
    Edge, EdgeExt, Knows, Person, Vertex, VertexExt, assert_elements_eq, assert_elements_one_of,
    populate_graph,
};
use graph_api_lib::{EdgeReference, EdgeSearch, Graph, VertexReference, VertexSearch};
// We import ControlFlow variants (Break, Continue) directly below
use std::ops::ControlFlow::{Break, Continue};

/// Test vertex reduce operations with the non-terminal reduce
pub fn test_vertices_reduce<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Populate the graph with test data
    let refs = populate_graph(graph);

    let (oldest, age) = graph
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
                    Continue(vertex)
                } else {
                    Continue(acc)
                }
            },
        )
        .map(|vertex, ctx| (vertex.id(), ctx))
        .next()
        .expect("should have got an element");

    // Verify the result exists
    assert_elements_eq!(graph, vec![oldest], vec![refs.julia]);
    assert_eq!(age, 48);
}

/// Test edge reduce operations with the non-terminal reduce
pub fn test_edges_reduce<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    let (edge_id, since) = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .edges(EdgeSearch::scan())
        .filter_knows()
        .reduce(
            |edge, _| edge.project::<Knows<_>>().unwrap().since(),
            |acc, ctx, edge, _edge_ctx| {
                let acc_since = acc.project::<Knows<_>>().unwrap().since();
                let edge_since = edge.project::<Knows<_>>().unwrap().since();
                if edge_since > acc_since {
                    *ctx = edge_since;
                    Continue(edge)
                } else {
                    Continue(acc)
                }
            },
        )
        .map(|edge, ctx| (edge.id(), ctx))
        .next()
        .expect("should have got an element");

    // Verify the result - in this case, both edges have the same since value (1999)
    // So the accumulated edge should be one of the two input edges
    assert_elements_one_of!(
        graph,
        vec![edge_id],
        vec![refs.bryn_knows_julia, refs.julia_knows_bryn]
    );
    assert_eq!(since, 1999);
}

/// Test vertex reduce operations with early break
pub fn test_vertices_reduce_break<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    // Populate the graph with test data
    let refs = populate_graph(graph);

    // Break as soon as we find a person with age > 30
    let (person, age) = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .reduce(
            |vertex, _| vertex.project::<Person<_>>().unwrap().age(),
            |acc, ctx, vertex, _vertex_ctx| {
                let vertex_age = vertex.project::<Person<_>>().unwrap().age();

                // Break on first person with age > 30
                if vertex_age > 30 {
                    *ctx = vertex_age;
                    Break(vertex)
                } else {
                    *ctx = vertex_age;
                    Continue(acc)
                }
            },
        )
        .map(|vertex, ctx| (vertex.id(), ctx))
        .next()
        .expect("should have got an element");

    // Either person could be returned depending on iteration order
    assert_elements_one_of!(graph, vec![person], vec![refs.bryn, refs.julia]);
    assert!(age > 30);
}

/// Test edge reduce operations with early break
pub fn test_edges_reduce_break<G>(graph: &mut G)
where
    G: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Break when we find any Knows edge
    let (edge_id, since) = graph
        .walk()
        .vertices(VertexSearch::scan())
        .filter_person()
        .edges(EdgeSearch::scan())
        .filter_knows()
        .reduce(
            |edge, _| edge.project::<Knows<_>>().unwrap().since(),
            |_acc, ctx, edge, _edge_ctx| {
                let edge_since = edge.project::<Knows<_>>().unwrap().since();
                *ctx = edge_since;
                // Break on the first edge we see
                Break(edge)
            },
        )
        .map(|edge, ctx| (edge.id(), ctx))
        .next()
        .expect("should have got an element");

    // The first edge we encounter should be returned
    assert_elements_one_of!(
        graph,
        vec![edge_id],
        vec![refs.bryn_knows_julia, refs.julia_knows_bryn]
    );
    assert_eq!(since, 1999);
}
