# Filter Step

The `filter` step allows you to keep only elements that match a predicate function. Elements that don't match the predicate are excluded from further traversal.

## Visual Diagram

Before filter step (all vertices in traversal):
```text
  [Person A]* --- knows ---> [Person B]* --- created ---> [Project]*
   ^                                         
   |                                         
  owns                                       
   |                                         
  [Company C]*                                        
```

After filter(is_person) step (only Person vertices remain in traversal):
```text
  [Person A]* --- knows ---> [Person B]* --- created ---> [Project]
   ^                                         
   |                                         
  owns                                       
   |                                         
  [Company C]                                        
```

## Parameters

- `predicate`: A function that takes a reference to an element and returns a boolean. Only elements for which this function returns `true` will be included in the traversal.

## Return Value

A new walker containing only the elements that matched the predicate.

## Examples

```rust
// Filter to keep only Person vertices
let people = graph
    .walk()
    .vertices(VertexSearch::scan())
    .filter(|v| v.label() == Vertex::person_label())
    .collect::<Vec<_>>();

// More complex filtering with projection
let adult_developers = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .filter(|v| {
        // Project the vertex to a Person
        if let Ok(person) = v.project::<Person<_>>() {
            person.age() >= 18 && person.has_skill("Rust")
        } else {
            false
        }
    })
    .collect::<Vec<_>>();
```

## Notes

- The filter step does not modify the graph, only the traversal
- For complex filtering logic, consider breaking into multiple filter steps for better readability
- Use type projections when filtering to access type-specific methods and properties
- The filter is applied lazily during traversal, not when the step is added to the walker