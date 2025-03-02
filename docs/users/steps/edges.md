# Edges Step

The `edges` step allows you to traverse to the edges in a graph. It moves the traversal position from vertices to their connected edges based on the provided search criteria.

## Visual Diagram

Before edges step (traversal position on vertices):
```text
  [Person A]* --- knows ---> [Person B] --- created ---> [Project]
   ^                                         
   |                                         
  owns                                       
   |                                         
  [Company C]                                        
```

After edges step with outgoing direction (traversal position moves to edges):
```text
  [Person A] --- knows --->* [Person B] --- created ---> [Project]
   ^                                         
   |                                         
  owns -*                                        
   |                                         
  [Company C]                                        
```

## Parameters

- `search`: An `EdgeSearch` that defines which edges to include. This can filter by label, direction, and other criteria.

## Return Value

A new walker where the traversal position is on the edges matching the search criteria.

## Examples

```rust
// Get all edges in the graph
let all_edges = graph
    .walk()
    .vertices(VertexSearch::scan())
    .edges(EdgeSearch::scan())
    .collect::<Vec<_>>();

// Get only outgoing 'knows' edges from people
let knows_edges = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .edges(EdgeSearch::scan()
        .outgoing()
        .with_label(Edge::knows_label()))
    .collect::<Vec<_>>();
    
// Get edges in both directions with properties filter
let recent_edges = graph
    .walk()
    .vertices_by_id(vec![person_id])
    .edges(EdgeSearch::scan().bidirectional())
    .filter(|e| {
        if let Ok(knows) = e.project::<Knows<_>>() {
            knows.since() > 2020
        } else {
            false
        }
    })
    .collect::<Vec<_>>();
```

## Notes

- The edges step changes the traversal position from vertices to edges
- To get back to vertices after an edges step, use `head()` or `tail()`
- The search direction matters: `.outgoing()` finds edges where the current vertex is the source, `.incoming()` finds edges where the current vertex is the target, and `.bidirectional()` finds both
- The edges step can filter by label and other properties through the EdgeSearch parameter