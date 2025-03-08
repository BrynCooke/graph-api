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

See the [steps directory](./steps/README.md) for detailed documentation on each available step.

## Common Step Categories

### Traversal Position Steps
- [vertices](./steps/vertices.md) - Start traversal from vertices
- [edges](./steps/edges.md) - Move to edges
- [head](./steps/head.md) - Move to source vertices of edges
- [tail](./steps/tail.md) - Move to target vertices of edges

### Filtering Steps
- [filter](./steps/filter.md) - Keep only elements that match a predicate
- [limit](./steps/limit.md) - Limit the number of elements
- [first](./steps/first.md) - Take only the first element

### Context Steps
- [push_context](./steps/context.md) - Store data in context
- [map_context](./steps/context.md) - Transform context data
- [default_context](./steps/default_context.md) - Set default context

### Collection Steps
- [collect](./steps/collect.md) - Collect elements into a collection
- [count](./steps/count.md) - Count elements
- [is_empty](./steps/is_empty.md) - Check if traversal is empty

### Advanced Steps
- [detour](./steps/detour.md) - Create sub-traversals
- [mutate](./steps/mutate.md) - Modify the graph during traversal
- [probe](./steps/probe.md) - Debug traversal state

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

## Best Practices

1. **Chain steps logically**: Build your traversal in a logical sequence that mirrors how you would describe the path
2. **Use appropriate search criteria**: Limit vertices and edges early in the traversal
3. **Leverage type projections**: Use `.project::<Type<_>>()` to access type-specific methods
4. **Use context for data collection**: Store intermediate results in context rather than using external collections
5. **Consider performance**: For very large graphs, filter early to reduce the traversal set

## Further Reading

- [Basic Operations](./basic_operations) - Core graph operations
- [Defining a Model](./defining_a_model) - Creating vertex and edge types
- [Walker Steps Directory](./steps/README.md) - Detailed documentation for all steps