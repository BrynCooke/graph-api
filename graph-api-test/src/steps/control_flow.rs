use crate::{Edge, Vertex, assert_elements_eq, populate_graph};
use graph_api_lib::{EdgeSearch, Graph, VertexReference};
use std::ops::ControlFlow;

pub fn test_vertices_control_flow<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);

    // Test Continue - Should work like filter but with ControlFlow
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn, refs.julia, refs.graph_api])
        .control_flow(|vertex, _| match vertex.weight() {
            Vertex::Person { .. } => ControlFlow::Continue(()),
            _ => ControlFlow::Break(()),
        })
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn, refs.julia]);

    // Test Break - Should stop traversal after finding Bryn
    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.bryn, refs.julia, refs.graph_api])
        .control_flow(|vertex, _| match vertex.weight() {
            Vertex::Person { name, .. } if name == "Bryn" => ControlFlow::Continue(()),
            Vertex::Person { name, .. } if name == "Julia" => ControlFlow::Break(()),
            _ => ControlFlow::Continue(()),
        })
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.bryn]);

    // Test context mutation with control flow
    let collected = graph
        .walk()
        .push_context(0)
        .vertices_by_id(vec![refs.bryn, refs.julia, refs.graph_api])
        .control_flow(|_, ctx| {
            *ctx += 1;
            if *ctx >= 2 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
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
        .control_flow(|_, ctx| {
            *ctx += 1;
            if *ctx >= 2 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .count();
    // Only the first edge should be processed
    assert_eq!(collected, 1);
}
