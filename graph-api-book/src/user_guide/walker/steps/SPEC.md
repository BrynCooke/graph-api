# Walker Steps Documentation

This directory contains documentation for all available steps in the Graph API walker interface. Walker steps are the
building blocks for graph traversals.

## What are Walker Steps?

Walker steps are chainable operations that build a graph traversal. Each step performs a specific operation on the
traversal, such as:

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

```bob
Before step:
  [A]* --- knows ---> [B] --- created ---> [C]

After step:
  [A] --- knows --->* [B] --- created ---> [C]
```

## Examples

Make sure that examples are:

1. Concise - They should demonstrate one thing only.
2. Use the standard model - We don't need to create a new model for every step.
3. Commented - So the user knows what is happening in the example.
4. Externalized - No inline code, we want to ensure that example code always compiles.
5. Compiled without errors or warnings - Examples must be clean.
6. **Use generated helpers and projections** - Examples MUST:
    - Use `filter_person()`, `filter_project()` to filter by label type (no closure needed)
    - Use `filter_by_person(|person, _| ...)` when filtering by properties with a typed projection
    - Use projections like `vertex.project::<Person<_>>()` for type-safe access to properties
    - Use the projection's accessor methods (e.g., `person.name()`, `person.age()`) for field access
    - Use generated index methods like `VertexIndex::person_by_username()` for efficient lookups
    - Remember that map steps don't automatically provide projections - use project explicitly
    - Show the type-safety benefits of these features, as they are core advantages of the Graph API
7. Showcase type safety - Explicitly demonstrate how the API provides type-safe access to vertex and edge properties.

## Model example

The filter step is the model example for a documented step. Make sure to use it when writing or modifying step
documentation. 