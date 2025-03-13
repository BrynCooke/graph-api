# Walker Steps

Walker steps are the building blocks for graph traversals. Each step performs a specific operation on the traversal, such as moving the position, filtering elements, or collecting results.

## What are Walker Steps?

Walker steps are chainable operations that build a graph traversal. Each step performs a specific operation on the traversal, such as:

- Moving the traversal position (vertices, edges, head, tail)
- Filtering elements (filter)
- Limiting results (limit, first)
- Collecting results (collect)
- Modifying the graph (mutate)
- And many more

## Step Documentation Format

Each walker step documentation follows a consistent format:

1. **Title and Description**: Brief overview of what the step does
2. **Visual Diagram**: Illustration showing the before and after states of the traversal
3. **Parameters**: Description of all parameters
4. **Return Value**: What the step returns
5. **Examples**: Code examples showing the step in use
6. **Notes**: Additional information, edge cases, and common patterns

## Visual Diagrams

All walker step documentation includes ASCII diagrams that show:
- The graph structure
- The traversal position (marked with *)
- How the step affects the traversal

Example diagram:
```
Before step:
  [A]* --- knows ---> [B] --- created ---> [C]

After step:
  [A] --- knows --->* [B] --- created ---> [C]
```

## Available Steps

### Traversal Initiation
- [vertices](vertices.md) - Start traversal from vertices matching criteria
- [vertices_by_id](vertices_by_id.md) - Start traversal from vertices with specific IDs

### Traversal Movement
- [edges](edges.md) - Traverse along edges
- [head](head.md) - Move to source vertices of edges
- [tail](tail.md) - Move to target vertices of edges

### Filtering and Limiting
- [filter](filter.md) - Filter elements based on a predicate
- [limit](limit.md) - Limit the number of elements in the traversal
- [first](first.md) - Get only the first element

### Context and Data Handling
- [context](context.md) - Associate custom data with traversal elements
- [default_context](default_context.md) - Use predefined context for common patterns
- [map](map.md) - Transform elements during traversal

### Terminal Operations
- [collect](collect.md) - Gather results into a collection
- [count](count.md) - Count elements in the traversal
- [iter](iter.md) - Iterate over elements in the traversal

### Advanced Operations
- [detour](detour.md) - Create a sub-traversal from the current position
- [mutate](mutate.md) - Modify the graph during traversal
- [probe](probe.md) - Inspect elements during traversal

### Debugging
- [dbg](dbg.md) - Print debug information during traversal