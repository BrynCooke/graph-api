# Collect Step

The `collect` step finalizes a traversal and gathers the results into a collection. This is a terminal operation that ends the traversal and provides access to the traversed elements.

## Visual Diagram

Before collect step (with elements in traversal):
```
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After collect step (all elements consumed and collected):
```
  [A] --- edge1 ---> [B] --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        

  Collection: [A, B, C, D]
```

## Parameters

None - but the resulting collection type is determined by the type parameter provided to the collect call.

## Return Value

Returns a collection of the traversed elements. The exact type depends on what you're collecting into, commonly:
- `Vec<ElementId>` for simple ID collection
- `Vec<(ElementId, Context)>` when context is used
- Custom types when implementing `FromVertexWalker` or `FromEdgeWalker`

## Examples

```rust
// Collect vertex IDs into a vector
let vertex_ids = graph
    .walk()
    .vertices(VertexSearch::scan())
    .collect::<Vec<_>>();

// Collect vertices with their context
let vertices_with_age = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .push_context(|person, _| person.project::<Person<_>>().unwrap().age())
    .collect::<Vec<_>>();

// Process collected results
for (vertex_id, age) in vertices_with_age {
    println!("Person ID: {:?}, Age: {}", vertex_id, age);
}

// Collect into a custom type
let person_map = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .collect::<HashMap<_, _>>();
```

## Notes

- The `collect` step is a terminal operation - no further traversal steps can be added after it
- When collecting with context, the result will be pairs of (element_id, context)
- The collect step fully consumes the traversal
- You can implement custom `FromVertexWalker` or `FromEdgeWalker` traits to collect into specialized types
- Most commonly used with `Vec<_>`, but can collect into any type that implements the appropriate From traits
- Consider using `limit` before `collect` for large graphs to avoid excessive memory use
- For single-element queries, consider using `first()` instead of `collect` for efficiency