****# Walker Steps Documentation

This directory contains documentation for all available steps in the Graph API walker interface. Walker steps are the building blocks for graph traversals.

## What are Walker Steps?

Walker steps are chainable operations that build a graph traversal. Each step performs a specific operation on the traversal, such as:

- Moving the traversal position (vertices, edges, head, tail)
- Filtering elements (filter)
- Limiting results (limit, first)
- Collecting results (collect)
- Modifying the graph (mutate)
- And many more

## Documentation Standards

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

## Completed Documentation

The following steps have complete documentation with visual diagrams:

1. **Traversal Initiation**
   - [vertices](vertices.md) - Start traversal from vertices matching criteria

2. **Traversal Movement**
   - [edges](edges.md) - Traverse along edges
   - [detour](detour.md) - Create a sub-traversal from the current position

3. **Filtering and Limiting**
   - [filter](filter.md) - Filter elements based on a predicate
   - [limit](limit.md) - Limit the number of elements in the traversal
   - [first](first.md) - Get only the first element

4. **Context and Data Handling**
   - [context](context.md) - Associate custom data with traversal elements
   - [default_context](default_context.md) - Use predefined context for common patterns

5. **Terminal Operations**
   - [collect](collect.md) - Gather results into a collection
   - [count](count.md) - Count elements in the traversal
   - [mutate](mutate.md) - Modify the graph during traversal

6. **Debugging**
   - [dbg](dbg.md) - Print debug information during traversal

## Steps Needing Documentation

Several steps still need documentation:
- head
- tail
- iter
- probe

## Contributing

When adding or updating step documentation:

1. Use the [template.md](template.md) file as a starting point
2. Always include a visual diagram showing before and after states
3. Provide clear, runnable examples
4. Document all parameters and return values
5. Include notes about common patterns and edge cases

For more details on contributing, see the [Contributing Guidelines](../../../CONTRIBUTING.md).