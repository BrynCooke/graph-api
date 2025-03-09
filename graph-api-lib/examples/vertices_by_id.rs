use graph_api_simplegraph::{SimpleGraph, VertexId};
use graph_api_test::populate_graph;

fn main() {
    let mut graph = SimpleGraph::new();
    // Populate the graph with test data
    let refs = populate_graph(&mut graph);
    example(graph, refs.bryn, refs.julia);
}

fn example<Graph>(graph: Graph, bryn_id: Graph::VertexId, julia_id: Graph::VertexId)
where
    Graph: graph_api_lib::Graph<VertexId = VertexId>,
{
    // Simple example using known IDs
    let result = graph
        .walk()
        .vertices_by_id(vec![bryn_id, julia_id])
        .collect::<Vec<_>>();

    assert_eq!(result.len(), 2);

    // When using non-existent IDs, they are simply skipped (example is using simple graph vertex id)
    let made_up_id = VertexId::new(3, 4);

    let filtered_result = graph
        .walk()
        .vertices_by_id(vec![bryn_id, made_up_id])
        .collect::<Vec<_>>();

    assert_eq!(filtered_result.len(), 1); // Only bryn is found
}
