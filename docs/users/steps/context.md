# Context Step

The `push_context` step allows you to associate additional data with each element in the traversal. This is useful for carrying information along as you traverse, preserving state between traversal steps, or accumulating results.

## Visual Diagram

Before push_context step (traversal with regular elements):
```text
  [Person A]* --- created ---> [Project X]*  
   |
  knows
   |
  [Person B]*
```

After push_context step (elements now have associated context data):
```text
  [Person A]* + {age: 30} --- created ---> [Project X]* + {name: "Graph API"}
   |
  knows
   |
  [Person B]* + {age: 25}
```

## Parameters

- `callback`: A function that takes the current element and its existing context, and returns a new context value to associate with that element

## Return Value

Returns a traversal with the same elements, but with additional context information attached to each element.

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

// Store vertex ages in context
let results = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .push_context(|person, ctx| person.project::<Person<_>>().unwrap().age())
    .collect::<Vec<_>>();
    
// Access both vertex and context
for (vertex, age) in results {
    println!("Person ID: {:?}, Age: {}", vertex, age);
}

// Accumulate data across multiple traversal steps
let total_age = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .push_context(|_, _| 0) // Initialize counter
    .out_edges(EdgeSearch::scan().with_label(Edge::knows_label()))
    .head()
    .push_context(|person, ctx| {
        *ctx + person.project::<Person<_>>().unwrap().age()
    })
    .collect::<Vec<_>>()
    .into_iter()
    .map(|(_, age)| age)
    .sum::<u64>();
```

## Notes

- Context is carried through the entire traversal, even across different graph elements
- Each push_context call creates a new context layer, with the previous context available as `ctx.parent()`
- For complex traversals, you can build a nested context structure
- The context is cloned for each element, so keep context objects relatively small for performance
- Use `push_default_context()` for common patterns like storing the element's ID and data
- Context persists even when traversing to different elements (e.g., from vertex to connected edge)
- When retrieving results, both the element and its context are returned in a tuple