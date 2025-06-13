# Walker Steps

Walker steps are the building blocks for graph traversals. Each step performs a specific operation on the traversal,
such as moving the position, filtering elements, or collecting results.

## What are Walker Steps?

Walker steps are chainable operations that build a graph traversal. Each step performs a specific operation on the
traversal, such as:

- Moving the traversal position (vertices, edges, head, tail)
- Filtering elements (filter)
- Limiting results (limit, first)
- Collecting results (collect)
- Modifying the graph (mutate)
- And many more

## Available Steps

### Traversal Initiation

- [vertices](steps/vertices.md) - Start traversal from vertices matching criteria
- [vertices_by_id](steps/vertices_by_id.md) - Start traversal from vertices with specific IDs

### Traversal Movement

- [detour](steps/detour.md) - Create a sub-traversal from the current position
- [edges](steps/edges.md) - Traverse along edges
- [head](steps/head.md) - Move to source vertices of edges
- [tail](steps/tail.md) - Move to target vertices of edges

### Filtering and Limiting

- [filter](steps/filter.md) - Filter elements based on a predicate
- [first](steps/first.md) - Get only the first element
- [take](steps/take.md) - Take a specified number of elements

### Context and Data Handling

- [push_context](steps/push_context.md) - Associate custom data with traversal elements
- [default_context](steps/default_context.md) - Use predefined context for common patterns
- [mutate_context](steps/mutate_context.md) - Modify context during traversal

### Terminal Operations

- [collect](steps/collect.md) - Gather results into a collection
- [count](steps/count.md) - Count elements in the traversal
- [into_iter](steps/into_iter.md) - Convert traversal to an iterator
- [fold](steps/fold.md) - Fold elements into an accumulated value
- [map](steps/map.md) - Transform elements during traversal
- [reduce](steps/reduce.md) - Combine elements using a reduction function

### Control Flow

- [control_flow](steps/control_flow.md) - Control traversal flow and early termination

### Side effects

- [mutate](steps/mutate.md) - Modify the graph after traversal
- [probe](steps/probe.md) - Inspect elements during traversal

### Type Management

- [boxed](steps/boxed.md) - Reduce type complexity and improve compile times

### Debugging

- [dbg](steps/dbg.md) - Print debug information during traversal