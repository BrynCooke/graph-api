# Head Step

The `head` step navigates from edges to their source (origin) vertices, allowing traversal back to where the edges start
from.

<object type="image/svg+xml" data="head_tail/image_head.svg">
Head step diagram showing traversal from edge to source vertex
</object>

## Syntax

```rust,noplayground
walker.head()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker positioned at the source vertices of the edges in the current traversal.

## Example

Find people who created projects by getting back to the source vertex:

```rust,noplayground
{{#include head_tail/head_tail_examples.rs:head_example}}
```

## Best Practices

- Always follow edge traversals with either head() or tail() to return to vertices
- Use contexts to retain information about the edge when moving to vertices
- Remember that head() returns the source vertex (where the edge starts)
- For retrieving both endpoints, consider using context to store one while visiting the other

## Common Use Cases

- **Author identification**: Finding who created something by looking at edge sources
- **Relationship sources**: Identifying the initiators of connections
- **Backtracking**: Returning to source vertices after examining edges
- **Edge-based filtering**: Finding vertices that have specific outgoing edges
