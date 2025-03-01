# Head Step

The `head` step transforms an edge traversal into a vertex traversal by moving to the head vertex of each edge. In graph theory, the head is the source/origin vertex that the edge comes from.

## Visual Diagram

Before head step (with edges as current elements):
```
  [A] --- edge1* ---> [B] --- edge2* ---> [C]  
   ^                                         
   |                                         
  edge3*                                       
   |                                         
  [D]                                        
```

After head step (moved to source vertices of edges):
```
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

## Parameters

None

## Return Value

A vertex walker that will traverse the source/origin vertices of the edges from the previous step.

## Examples

```rust
// Find the authors of all comments on a post
let authors = graph
    .walk()
    .vertices_by_id(vec![post_id])
    .in_edges(EdgeSearch::scan().with_label(Edge::commented_label()))
    .head()
    .collect::<Vec<_>>();

// Find all people who created projects
let creators = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::project_label()))
    .in_edges(EdgeSearch::scan().with_label(Edge::created_label()))
    .head()
    .collect::<Vec<_>>();

// Finding common friends between two people
let common_friends = graph
    .walk()
    .vertices_by_id(vec![person_a_id, person_b_id])
    .out_edges(EdgeSearch::scan().with_label(Edge::knows_label()))
    .tail()
    .filter(|v| {
        // Check if this friend knows both person A and person B
        let incoming_knows = graph
            .walk()
            .vertices_by_id(vec![v.id()])
            .in_edges(EdgeSearch::scan().with_label(Edge::knows_label()))
            .head()
            .collect::<Vec<_>>();
        
        incoming_knows.len() >= 2
    })
    .collect::<Vec<_>>();
```

## Notes

- The `head` step can only be used after an edge traversal step
- Transforms the traversal type from EdgeWalker to VertexWalker
- For directed graphs, head refers to the source/origin vertex
- For undirected graphs, the distinction between head and tail may depend on implementation
- Commonly used in conjunction with `in_edges` to find vertices that point to certain destinations
- The head-tail terminology follows standard graph theory convention
- When working with edges, remember that `head()` gives you "where the edge comes from" (source)