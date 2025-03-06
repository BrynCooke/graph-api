# Iter Step

While there is no explicit `iter` step method in the walker API, the walker builders implement the `IntoIterator` trait, allowing you to convert a traversal into a standard Rust iterator with the `.iter()` or `.into_iter()` methods. This enables using standard iterator methods like `map`, `filter`, and `fold` on your graph traversal results.

## Visual Diagram

Before iter (walker traversal):
```text
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After iter (converted to standard Rust iterator):
```text
  Iterator<Item = (reference, context)>
  [A, ctx] -> [B, ctx] -> [C, ctx] -> [D, ctx]
```

## Parameters

None (when using `.into_iter()` or `.iter()`)

## Return Value

An iterator that yields tuples of `(reference, context)` where:
- `reference` is either a vertex or edge reference depending on the walker type
- `context` contains any accumulated context data from the traversal

## Examples

```rust
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_derive::VertexExt;
# use graph_api_derive::EdgeExt;
# use uuid::Uuid;
# use graph_api_lib::Id;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use std::ops::Deref;
# use graph_api_lib::VertexSearch;
# let mut graph = SimpleGraph::new();

// Map traversal results to extract vertex properties
let names = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .into_iter()
    .map(|(vertex, _)| vertex.project::<Person<_>>().unwrap().name().to_string())
    .collect::<Vec<_>>();

// Filter iterator results after traversal
let adult_names = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .into_iter()
    .filter(|(vertex, _)| {
        vertex.project::<Person<_>>().map_or(false, |p| p.age() >= 18)
    })
    .map(|(vertex, _)| vertex.project::<Person<_>>().unwrap().name().to_string())
    .collect::<Vec<_>>();

// Using fold to calculate a value from traversal
let total_age = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .push_context(|v, _| {
        // Store age in context
        if let Ok(person) = v.project::<Person<_>>() {
            person.age()
        } else {
            0
        }
    })
    .into_iter()
    .fold(0, |acc, (_, age)| acc + *age);
```

## Notes

- The iterator yields both the element reference and its associated context
- Using `.into_iter()` consumes the walker, while some implementations may provide a `.iter()` method that borrows it
- This is the bridge between Graph API's walker system and Rust's standard iterator ecosystem
- After converting to an iterator, you can use all standard Rust iterator methods
- Context data is accessible as the second element of the tuple
- Prefer using walker steps for graph traversal logic, and iterator methods for post-traversal processing
- Using iterator methods allows for more complex transformations than the provided walker steps
- The context system is particularly powerful when combined with iterator operations
- Unlike walker steps, iterator operations are not lazy - the traversal is executed when iterator methods are called