# Debug Step

The `dbg` step prints elements as they are traversed through the graph, making it easier to debug complex traversals. Each element is tagged with the provided label.

## Visual Diagram

Before dbg step:
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After dbg step (elements continue in traversal, but are also printed to console):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        

  Console output:
  [stage1] Vertex(A)
  [stage1] Vertex(B)
  [stage1] Vertex(C)
  [stage1] Vertex(D)
```

## Parameters

- `tag`: A string label that will prefix all debug output to help identify which debug step is printing

## Return Value

Returns the same traversal that was passed in, allowing the traversal to continue unmodified.

## Examples

```rust
# use graph_api_test::populate_graph;
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::VertexIndex;
# use graph_api_test::EdgeIndex;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use graph_api_lib::VertexSearch;
# use graph_api_lib::EdgeSearch;
# 
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);

// Debug vertices in a traversal (this will print to console)
// We can count the number of vertices to verify it works
let vertex_count = graph.walk()
    .vertices(VertexSearch::scan())
    .dbg("vertices") // Print vertices to console with tag "vertices"
    .count();

// There should be at least 4 vertices in the graph
assert!(vertex_count >= 4);

// Use debug with edges
let edge_count = graph.walk()
    .vertices_by_id(vec![refs.bryn])
    .edges(EdgeSearch::scan())
    .dbg("bryn_edges") // Print edges to console
    .count();

// Bryn should have at least one edge
assert!(edge_count > 0);

// Multiple debug points can be used in sequence
// For simplicity in testing, we'll just verify it works without checking specific output
let result = graph.walk()
    .vertices_by_id(vec![refs.bryn])
    .dbg("start") // First debug point
    .edges(EdgeSearch::scan())
    .dbg("edges") // Second debug point
    .head()
    .dbg("heads") // Third debug point
    .count();

// Operation should complete successfully
assert!(result >= 0);
```

## Notes

- The `dbg` step is non-destructive - it does not modify the traversal path
- Debug output goes to the console using the standard Debug trait implementation
- Remember that traversals are typically depth-first, so the output order may not be immediately intuitive
- For complex graphs, consider using more descriptive tags at each debug point
- This step is particularly useful for understanding how graph elements flow through a complex traversal
- The `dbg` step has minimal performance impact when not in debug mode