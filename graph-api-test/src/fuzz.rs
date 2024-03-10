use graph_api_lib::EdgeReference;
use graph_api_lib::Graph;
use proptest::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum GraphOperation {
    AddVertex(u32),
    RemoveVertex(usize),
    AddEdge(usize, usize),
    RemoveEdge(usize),
}

prop_compose! {
    pub fn arb_graph_operation()(
        op_type in 0..4u8,
        vertex_weight in any::<u32>(),
        index in any::<usize>(),
        from_idx in any::<usize>(),
        to_idx in any::<usize>(),
    ) -> GraphOperation {
        match op_type {
            0 => GraphOperation::AddVertex(vertex_weight),
            1 => GraphOperation::RemoveVertex(index),
            2 => GraphOperation::AddEdge(from_idx, to_idx),
            _ => GraphOperation::RemoveEdge(index),
        }
    }
}

pub fn test_fuzz(
    mut graph: impl Graph<Vertex = u32, Edge = ()>,
    operations: Vec<GraphOperation>,
) -> bool {
    let mut vertex_ids = HashSet::new();
    let mut edge_ids = HashSet::new();

    for op in operations {
        match op {
            GraphOperation::AddVertex(weight) => {
                let id = graph.add_vertex(weight);
                vertex_ids.insert(id);
            }
            GraphOperation::RemoveVertex(idx) => {
                if let Some(id) = vertex_ids.iter().nth(idx % (vertex_ids.len() + 1)).cloned() {
                    //println!("Removing vertex: {:?}", id);
                    // First find all edges connected to this vertex
                    let connected_edges: HashSet<_> = edge_ids
                        .iter()
                        .filter(|&&edge_id| {
                            let edge = graph.edge(edge_id).unwrap();
                            let head = edge.head();
                            let tail = edge.tail();
                            head == id || tail == id
                        })
                        .cloned()
                        .collect();

                    // Remove the vertex
                    graph.remove_vertex(id);
                    vertex_ids.remove(&id);

                    // Remove all connected edges from our tracking set
                    edge_ids = &edge_ids - &connected_edges;
                }
            }
            GraphOperation::AddEdge(from_idx, to_idx) => {
                let vertices: Vec<_> = vertex_ids.iter().collect();
                if vertices.len() >= 2 {
                    let from = vertices[from_idx % vertices.len()];
                    let to = vertices[to_idx % vertices.len()];
                    let edge_id = graph.add_edge(*from, *to, ());
                    edge_ids.insert(edge_id);
                }
            }
            GraphOperation::RemoveEdge(idx) => {
                if let Some(id) = edge_ids.iter().nth(idx % (edge_ids.len() + 1)).cloned() {
                    //println!("Removing edge {:?}", id);
                    graph.remove_edge(id);
                    edge_ids.remove(&id);
                }
            }
        }

        //println!("Vertices: {:?}, Edges: {:?}", vertex_ids, edge_ids);
        //println!(
        //    "Graph vertices: {:?}, edges {:?}\n",
        //    graph.walk().vertices(None).collect::<Vec<_>>(),
        //   graph
        //        .walk()
        //        .vertices(None)
        //         .all_edges(None)
        //         .collect::<HashSet<_>>()
        // );

        // Validate no dangling edges
        for edge_id in &edge_ids {
            let edge = graph.edge(*edge_id).expect("Edge must exist");
            assert!(vertex_ids.contains(&edge.head()));
            assert!(vertex_ids.contains(&edge.tail()));
        }

        // Validate we can't walk to deleted elements
        for start in &vertex_ids {
            let reachable: HashSet<_> = graph
                .walk()
                .vertices_by_id([*start])
                .out_edges(None)
                .head()
                .collect();

            assert!(reachable.is_subset(&vertex_ids));
        }
    }
    true
}
