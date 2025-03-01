# Debug Step

The `dbg` step prints elements as they are traversed through the graph, making it easier to debug complex traversals. Each element is tagged with the provided label.

## Visual Diagram

Before dbg step:
```
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After dbg step (elements continue in traversal, but are also printed to console):
```
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
// Debug vertices in a traversal
graph.walk().vertices(VertexSearch::scan())
    .dbg("stage1") // elements are printed to the console 
    .count();

// Use multiple debug points in a traversal
graph.walk().vertices(VertexSearch::scan())
    .dbg("vertices") // prints all vertices
    .out_edges(EdgeSearch::scan())
    .dbg("outgoing_edges") // prints all outgoing edges
    .head()
    .dbg("destinations") // prints destination vertices
    .count();
```

## Notes

- The `dbg` step is non-destructive - it does not modify the traversal path
- Debug output goes to the console using the standard Debug trait implementation
- Remember that traversals are typically depth-first, so the output order may not be immediately intuitive
- For complex graphs, consider using more descriptive tags at each debug point
- This step is particularly useful for understanding how graph elements flow through a complex traversal
- The `dbg` step has minimal performance impact when not in debug mode