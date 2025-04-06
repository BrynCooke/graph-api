# Tail Step

The `tail` step navigates from edges to their **source** (origin) vertices, allowing traversal back to where the edges start from. It transforms a stream of edges into a stream of vertices.

<object type="image/svg+xml" data="tail/image.svg" title="Tail Step Diagram">
Tail step diagram showing traversal from edge to source vertex
</object>

In this diagram:

- An **Input Stream** contains edge elements (e.g., **A->B**, **C->D**).
- The **`.tail()` step** processes each edge.
- The **Output Stream** contains the corresponding **source (tail) vertices** (**A**, **C**) for each input edge.

## Syntax

```rust,noplayground
walker.tail()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker positioned at the **source** vertices of the edges in the current traversal.

## Examples

### Basic Tail Step

Find people who created projects by getting back to the source vertex:

```rust,noplayground
{{#include tail/head_tail_examples.rs:head_example}}
```

### Multi-Step Traversal

Traverse multiple relationships to find indirect connections:

```rust,noplayground
{{#include tail/head_tail_examples.rs:multi_step}}
```

## Best Practices

- Always follow edge traversals with either `head()` or `tail()` to return to vertices.
- Use contexts to retain information about the edge when moving back to the source vertex via `tail()`.
- Remember that `tail()` returns the **source** vertex (where the edge starts).
- For retrieving both endpoints, consider using context to store one while visiting the other.

## Common Use Cases

- **Author identification**: Finding who created something by looking at edge sources (`tail()`).
- **Relationship sources**: Identifying the initiators of connections (`tail()`).
- **Backtracking**: Returning to source vertices after examining edges (`tail()`).
- **Edge-based filtering**: Finding vertices that have specific *incoming* edges (by traversing the edge and using `tail()`).
