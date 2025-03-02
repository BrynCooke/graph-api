# Limit Step

The `limit` step restricts a traversal to return at most a specified number of elements. This is useful for pagination, performance optimization, or when you only need a subset of results.

## Visual Diagram

Before limit step (with multiple elements in traversal):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After limit(2) step (only first 2 elements remain in traversal):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

## Parameters

- `limit`: A usize value specifying the maximum number of elements to include in the traversal

## Return Value

Returns a traversal containing at most the specified number of elements.

## Examples

```rust
// Get at most 10 vertices
let some_vertices = graph
    .walk()
    .vertices(VertexSearch::scan())
    .limit(10)
    .collect::<Vec<_>>();

// Implement simple pagination
let page_size = 20;
let page_number = 2; // 0-indexed
let page = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .limit(page_size * (page_number + 1)) // Get enough elements for all pages up to current
    .collect::<Vec<_>>() // Collect all elements
    .iter()
    .skip(page_size * page_number) // Skip previous pages
    .take(page_size) // Take only current page
    .collect::<Vec<_>>();

// Find the two oldest people
let two_oldest = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .filter_by_person(|p| true) // Convert to person type
    .sort_by(|a, b| b.age().cmp(&a.age())) // Sort descending by age
    .limit(2)
    .collect::<Vec<_>>();
```

## Notes

- The `limit` step is generally applied after other filtering operations
- It does not guarantee which elements will be returned, just how many
- For predictable results, combine with sorting operations or ordered indexes
- Can significantly improve performance by avoiding unnecessary traversal
- Particularly useful for large graphs where full traversal would be expensive
- If the traversal contains fewer elements than the limit, all elements are returned
- Different from `first()` which returns only a single element as an Option