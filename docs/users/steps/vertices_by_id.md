# Vertices By ID Step

The `vertices_by_id` step allows you to begin a traversal from a specific set of vertex IDs. This is useful when you already know the IDs of the vertices you want to include in your traversal.

## Visual Diagram

Before vertices_by_id step (empty traversal):
```text
  [A] --- edge1 ---> [B] --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

After vertices_by_id step (with [id_A, id_C]):
```text
  [A]* --- edge1 ---> [B] --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

## Parameters

- `vertex_ids`: An iterator that yields vertex IDs to include in the traversal

## Return Value

Returns a traversal containing all vertices with the specified IDs.

## Examples

```rust
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::Project;
# use graph_api_lib::EdgeSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use uuid::Uuid;
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Create some vertices
# let v1 = graph.add_vertex(Vertex::Person {
#     name: "Alice".to_string(),
#     age: 30,
#     unique_id: Uuid::from_u128(1),
#     username: "alice".to_string(),
#     biography: "Likes graphs".to_string(),
# });
# let v2 = graph.add_vertex(Vertex::Person {
#     name: "Bob".to_string(),
#     age: 25,
#     unique_id: Uuid::from_u128(2),
#     username: "bob".to_string(),
#     biography: "Graph enthusiast".to_string(),
# });
# let v3 = graph.add_vertex(Vertex::Project(Project {
#     name: "GraphDB".to_string(),
# }));
# // Create some edges
# graph.add_edge(v1, v2, Edge::Knows { since: 2020 });
# graph.add_edge(v1, v3, Edge::Created);

// Get specific vertices by their IDs
let specific_vertices = graph
    .walk()
    .vertices_by_id(vec![v1, v2, v3])
    .collect::<Vec<_>>();

assert_eq!(specific_vertices.len(), 3);

// Use with filter to get specific vertices by checking the vertex type
let filtered_vertices = graph
    .walk()
    .vertices_by_id(vec![v1, v2, v3])
    .filter(|vertex, _| matches!(vertex.weight(), Vertex::Person { .. }))
    .collect::<Vec<_>>();

assert_eq!(filtered_vertices.len(), 2);

// Use with adjacent edges
let related_edges = graph
    .walk()
    .vertices_by_id(vec![v1])
    .edges(EdgeSearch::scan().outgoing())
    .collect::<Vec<_>>();

assert_eq!(related_edges.len(), 2);
```

## Notes

- This step is efficient when you already know the exact IDs of vertices you want to work with
- The order of vertices in the traversal will match the order of IDs in the input iterator
- For vertices that don't exist in the graph, they will be skipped without error
- This step is often used after a previous traversal has produced vertex IDs of interest
- When working with a large number of IDs, consider using a `HashSet` for deduplication if needed