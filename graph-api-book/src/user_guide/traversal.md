# Graph Traversal

The Graph API provides a powerful traversal interface called "walkers" that enables you to navigate and analyze your graph in a flexible, type-safe way.

## Walker Overview

Walkers are built by chaining steps that define how to traverse the graph. Each step performs a specific operation, such as:

- Starting at specific vertices
- Moving to connected edges
- Filtering elements
- Collecting data
- Modifying the graph

## Basic Traversal Example

```rust
// Find all Person vertices who know someone who created a Project
let results = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))  // Start with Person vertices
    .out_edges(EdgeSearch::scan().with_label(Edge::knows_label()))      // Follow "knows" edges
    .tail()                                                             // Move to the target Person
    .out_edges(EdgeSearch::scan().with_label(Edge::created_label()))    // Follow "created" edges
    .tail()                                                             // Move to the Project
    .filter(|v| v.label() == Vertex::project_label())                   // Ensure it's a Project
    .collect::<Vec<_>>();                                               // Collect results
```

## Walker Steps

The Graph API provides many steps that can be combined to create complex traversals. Each step is documented with:

- A description of what it does
- Visual diagrams showing before and after states
- Examples of how to use it
- Information about parameters and return values

See the [Walker Steps](./walker/steps.md) section for detailed documentation on each available step.

## Common Step Categories

### Traversal Position Steps
- [vertices](./walker/steps/vertices.md) - Start traversal from vertices
- [edges](./walker/steps/edges.md) - Move to edges
- [head](./walker/steps/head.md) - Move to source vertices of edges
- [tail](./walker/steps/tail.md) - Move to target vertices of edges

### Filtering Steps
- [filter](./walker/steps/filter.md) - Keep only elements that match a predicate
- [limit](./walker/steps/limit.md) - Limit the number of elements
- [first](./walker/steps/first.md) - Take only the first element

### Context Steps
- [context](./walker/steps/context.md) - Store data in context
- [default_context](./walker/steps/default_context.md) - Set default context

### Collection Steps
- [collect](./walker/steps/collect.md) - Collect elements into a collection
- [count](./walker/steps/count.md) - Count elements
- [into_iter](./walker/steps/into_iter.md) - Iterate over elements

### Advanced Steps
- [detour](./walker/steps/detour.md) - Create sub-traversals
- [mutate](./walker/steps/mutate.md) - Modify the graph during traversal
- [probe](./walker/steps/probe.md) - Debug traversal state
- [map](./walker/steps/map.md) - Transform elements during traversal

## Working with Context

Walkers have a built-in context system that allows you to store and transform data as you traverse the graph. This is particularly useful for:

1. Collecting metadata during traversal
2. Building complex data structures
3. Tracking traversal history
4. Performing calculations based on visited elements

Example using context:
```rust
// Calculate total age of all friends of a person
let total_age = graph
    .walk()
    .vertices_by_id(vec![person_id])
    .out_edges(EdgeSearch::scan().with_label(Edge::knows_label()))
    .tail()
    .push_context(|v, _| {
        // Store age in context
        if let Ok(person) = v.project::<Person<_>>() {
            person.age()
        } else {
            0
        }
    })
    .iter()
    .fold(0, |acc, (_, age)| acc + *age);
```

See the [Context System](./walker/context.md) page for more details.

## Best Practices

For more information about effectively using walkers, see the [Best Practices](./walker/best_practices.md) guide.