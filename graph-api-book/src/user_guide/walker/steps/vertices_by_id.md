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
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::VertexIndex;
# use graph_api_test::EdgeIndex;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use uuid::Uuid;
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with some test data
# let refs = populate_graph(&mut graph);
#
// Simple example
let result = graph
    .walk()
    .vertices_by_id(vec![refs.bryn, refs.julia])
    .collect::<Vec<_>>();
assert_eq!(result.len(), 2);
```

## Notes

- This step is efficient when you already know the exact IDs of vertices you want to work with
- The order of vertices in the traversal will match the order of IDs in the input iterator
- For vertices that don't exist in the graph, they will be skipped without error
- This step is often used after a previous traversal has produced vertex IDs of interest
- When working with a large number of IDs, consider using a `HashSet` for deduplication if needed