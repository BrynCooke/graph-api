The `mutate` step allows you to modify the graph as you traverse it. For each element (vertex or edge) visited during the traversal, the provided callback function is executed, allowing you to make modifications to the graph.

The callback receives:
- A mutable reference to the graph
- The element ID (vertex ID or edge ID)
- The context for the current element

This step can only be used with a mutable graph reference (`walk_mut()`).

Returns the number of elements that were modified.

### Example

```rust
// Add a 'Created' edge from each person to the graph_api project
let mutations = graph
    .walk_mut()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .mutate(|graph, vertex_id, _context| {
        graph.add_edge(vertex_id, graph_api_id, Edge::Created);
    });

// Add a 'Created' edge for each source vertex of a 'Knows' edge
let mutations = graph
    .walk_mut()
    .vertices_by_id(vec![person_id])
    .edges(EdgeSearch::scan().outgoing().with_label(Edge::knows_label()))
    .mutate(|graph, edge_id, _context| {
        let edge = graph.edge(edge_id).unwrap();
        let source = edge.source_id();
        graph.add_edge(source, project_id, Edge::Created);
    });
```