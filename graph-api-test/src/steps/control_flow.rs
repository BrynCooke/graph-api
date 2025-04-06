use crate::{Edge, Vertex, assert_elements_eq, populate_graph};
use graph_api_lib::{EdgeReference, EdgeSearch, Graph, VertexReference};
use std::ops::ControlFlow;

pub fn test_vertices_control_flow<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Test Continue(Some) / Continue(None) - Should work like filter
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn, refs.julia, refs.graph_api])
        .control_flow(|vertex, _| match vertex.weight() {
            Vertex::Person { .. } => ControlFlow::Continue(Some(vertex)),
            _ => ControlFlow::Continue(None), // Skip non-Person vertices
        })
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn, refs.julia]);

    // Test Break with Some - Should stop traversal after finding Julia but include it
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn, refs.julia, refs.graph_api])
        .control_flow(|vertex, _| match vertex.weight() {
            Vertex::Person { name, .. } if name == "Bryn" => ControlFlow::Continue(Some(vertex)),
            Vertex::Person { name, .. } if name == "Julia" => ControlFlow::Break(Some(vertex)),
            _ => ControlFlow::Continue(None),
        })
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn, refs.julia]);

    // Test Break with None - Should stop traversal after finding Julia without including it
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn, refs.julia, refs.graph_api])
        .control_flow(|vertex, _| match vertex.weight() {
            Vertex::Person { name, .. } if name == "Bryn" => ControlFlow::Continue(Some(vertex)),
            Vertex::Person { name, .. } if name == "Julia" => ControlFlow::Break(None),
            _ => ControlFlow::Continue(Some(vertex)),
        })
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);

    // Test context mutation with control flow
    let collected = graph
        .walk()
        .push_context(0)
        .vertices_by_id(vec![refs.bryn, refs.julia, refs.graph_api])
        .control_flow(|vertex, ctx| {
            *ctx += 1;
            if *ctx >= 2 {
                ControlFlow::Break(None)
            } else {
                ControlFlow::Continue(Some(vertex))
            }
        })
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);
}

pub fn test_edges_control_flow<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // As we don't know what order edges are returned we rely on context.
    let collected = graph
        .walk()
        .push_context(0)
        .vertices_by_id(vec![refs.bryn, refs.graph_api])
        .edges(EdgeSearch::scan())
        .control_flow(|edge, ctx| {
            *ctx += 1;
            if *ctx >= 2 {
                ControlFlow::Break(None)
            } else {
                ControlFlow::Continue(Some(edge))
            }
        })
        .count();
    // Only the first edge should be processed
    assert_eq!(collected, 1);

    // Test skipping certain edges with None
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn])
        .edges(EdgeSearch::scan())
        .control_flow(|edge, _| {
            if let Edge::Knows { .. } = edge.weight() {
                ControlFlow::Continue(Some(edge))
            } else {
                ControlFlow::Continue(None) // Skip non-Knows edges
            }
        })
        .collect::<Vec<_>>();

    // We should only collect "knows" edges
    for edge_id in &collected {
        if let Some(edge) = graph.edge(*edge_id) {
            assert!(matches!(edge.weight(), Edge::Knows { .. }));
        }
    }
}
