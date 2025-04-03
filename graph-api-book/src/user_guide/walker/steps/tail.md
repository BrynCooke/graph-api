# Tail Step

The `tail` step navigates from edges to their **source** (origin) vertices, allowing traversal back to where the edges start from.

<object type="image/svg+xml" data="head_tail/image_head.svg" title="Tail Step Diagram">
Tail step diagram showing traversal from edge to source vertex
</object>

In this diagram:

- **Before `tail()`**: The walker is positioned on the highlighted edge **A -> B**.
- The **`.tail()` step** is applied.
- **After `tail()`**: The walker moves to the **source vertex** of the edge, so vertex **A** is now highlighted. The edge and vertex B are no longer highlighted.

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
{{#include head_tail/head_tail_examples.rs:head_example}}
```

### Multi-Step Traversal

Traverse multiple relationships to find indirect connections (example needs adjustment if logic depends on tail meaning source):

```rust,noplayground
{{#include head_tail/head_tail_examples.rs:multi_step}}
```
*(Note: The multi-step example might need logical review depending on the intended query, as `tail()` now means source)*

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
