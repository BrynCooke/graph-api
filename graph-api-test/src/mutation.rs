use crate::{assert_elements_eq, populate_graph, Edge, Vertex};
use graph_api_lib::Graph;

pub fn test_mutation<T>(graph: &mut T)
where
    T: Graph<Vertex = Vertex, Edge = Edge>,
{
    let refs = populate_graph(graph);
    assert_eq!(
        graph
            .walk_mut()
            .vertices_by_id(vec![refs.julia])
            .mutate(|graph, _vertex_id, _context| {
                graph.add_edge(refs.julia, refs.graph_api, Edge::Created);
            }),
        1
    );

    let collected = graph
        .walk()
        .vertices_by_id(vec![refs.julia])
        .out_edges(None)
        .head()
        .collect::<Vec<_>>();
    assert_elements_eq!(graph, collected, vec![refs.graph_api, refs.bryn]);
}
