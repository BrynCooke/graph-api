# Tail Step

The `tail` step transforms an edge traversal into a vertex traversal by moving to the tail vertex of each edge. In graph theory, the tail is the destination/target vertex that the edge points to.

## Visual Diagram

Before tail step (with edges as current elements):
```text
  [A] --- edge1* ---> [B] --- edge2* ---> [C]  
   ^                                         
   |                                         
  edge3*                                       
   |                                         
  [D]                                        
```

After tail step (moved to target vertices of edges):
```text
  [A] --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

## Parameters

None

## Return Value

A vertex walker that will traverse the destination/target vertices of the edges from the previous step.

## Examples

```rust
// Find all projects created by a person
let projects = graph
    .walk()
    .vertices_by_id(vec![person_id])
    .out_edges(EdgeSearch::scan().with_label(Edge::created_label()))
    .tail()
    .collect::<Vec<_>>();

// Find all friends of a person (people they know)
let friends = graph
    .walk()
    .vertices_by_id(vec![person_id])
    .out_edges(EdgeSearch::scan().with_label(Edge::knows_label()))
    .tail()
    .collect::<Vec<_>>();

// Find second-degree connections (friends of friends)
let friends_of_friends = graph
    .walk()
    .vertices_by_id(vec![person_id])
    .out_edges(EdgeSearch::scan().with_label(Edge::knows_label()))
    .tail()
    .out_edges(EdgeSearch::scan().with_label(Edge::knows_label()))
    .tail()
    .filter(|v| v.id() != person_id) // Exclude the original person
    .collect::<Vec<_>>();
```

## Notes

- The `tail` step can only be used after an edge traversal step
- Transforms the traversal type from EdgeWalker to VertexWalker
- For directed graphs, tail refers to the destination/target vertex
- For undirected graphs, the distinction between head and tail may depend on implementation
- Commonly used in conjunction with `out_edges` to follow outgoing relationships
- The head-tail terminology follows standard graph theory convention
- When working with edges, remember that `tail()` gives you "where the edge points to" (destination)
- The combination of `out_edges().tail()` is so common that there's a shorthand `out_vertices()` step available in some graph implementations