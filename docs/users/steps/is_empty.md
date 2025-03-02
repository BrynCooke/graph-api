# Is Empty Step

The `is_empty` step checks if a traversal contains any elements. It's a terminal operation that returns a boolean value indicating whether the traversal is empty.

## Visual Diagram

Before is_empty step (with multiple elements in traversal):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After is_empty step (returns false, traversal consumed):
```text
  [A] --- edge1 ---> [B] --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

Before is_empty step (with empty traversal):
```text
  [A] --- edge1 ---> [B] --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

After is_empty step (returns true, traversal consumed):
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

Returns a boolean: `true` if the traversal contains no elements, `false` otherwise.

## Examples

```rust
// Check if the graph has any vertices
let has_vertices = !graph
    .walk()
    .vertices(VertexSearch::scan())
    .is_empty();

// Check if there are any people over age 30
let has_people_over_30 = !graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .filter(|person| person.project::<Person<_>>().unwrap().age() > 30)
    .is_empty();

// Verify if a specific person has any friends
let has_friends = !graph
    .walk()
    .vertices_by_id(vec![person_id])
    .out_edges(EdgeSearch::scan().with_label(Edge::knows_label()))
    .is_empty();
```

## Notes

- The `is_empty` step is implemented by calling `.count() == 0` internally
- This is a terminal operation - no further traversal steps can be added after it
- More efficient than collecting the results when you only need to check existence
- Semantically clearer than using `.first().is_none()` for existence checks
- Useful for validation logic and conditional processing
- Traverses the entire graph until it finds at least one element or exhausts the traversal