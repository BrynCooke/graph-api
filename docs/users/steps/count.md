# Count Step

The `count` step fully traverses the graph and returns the number of elements emitted by the traversal.

## Visual Diagram

Before count step (with elements in traversal):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After count step (consumed all elements, returned count 4):
```text
  [A] --- edge1 ---> [B] --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

## Parameters

None

## Return Value

Returns an integer representing the total number of elements in the traversal.

## Examples

```rust
// Count all vertices in the graph
let vertex_count = graph
    .walk()
    .vertices(VertexSearch::scan())
    .count();

// Count vertices with a specific label
let person_count = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .count();

// Count edges of a specific type
let knows_edge_count = graph
    .walk()
    .vertices(VertexSearch::scan())
    .out_edges(EdgeSearch::scan().with_label(Edge::knows_label()))
    .count();
```

## Notes

- The `count` step consumes the entire traversal
- This is a terminal operation - no further steps can be added after `count`
- For very large graphs, be aware that this will traverse the entire graph which may be expensive
- Consider using `limit` before `count` if you only need to check up to a certain number of elements