# Head Step

The `head` step navigates from edges to their **target** (destination) vertices, allowing traversal to where the edges point to.

<object type="image/svg+xml" data="head_tail/image_tail.svg" title="Head Step Diagram">
Head step diagram showing traversal from edge to target vertex
</object>

In this diagram:

- **Before `head()`**: The walker is positioned on the highlighted edge **A -> B**.
- The **`.head()` step** is applied.
- **After `head()`**: The walker moves to the **target vertex** of the edge, so vertex **B** is now highlighted. The edge and vertex A are no longer highlighted.

## Syntax

```rust,noplayground
walker.head()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker positioned at the **target** vertices of the edges in the current traversal.

## Example

Find projects created by following "created" edges to their target:

```rust,noplayground
{{#include head_tail/head_tail_examples.rs:tail_example}}
```

## Best Practices

- Use `head()` to follow relationships in their natural direction (to the target).
- Chain edge-head sequences for multi-hop traversals towards targets.
- Maintain context information when necessary to preserve edge properties while moving to the target.
- Consider traversal depth carefully in highly connected graphs when following edges.

## Common Use Cases

- **Following relationships**: Finding what vertices are connected *to* your starting points (targets).
- **Multi-hop traversals**: Discovering indirect connections through multiple relationships towards targets.
- **Graph exploration**: Navigating through the graph in a directed manner towards targets.
- **Social network queries**: Implementing patterns like "friends of friends" or "recommendations" by moving to targets.
