# Head Step

The `head` step navigates from edges to their **target** (destination) vertices, allowing traversal to where the edges point to. It transforms a stream of edges into a stream of vertices.

<object type="image/svg+xml" data="head/image.svg" title="Head Step Diagram">
Head step diagram showing traversal from edge to target vertex
</object>

In this diagram:

- An **Input Stream** contains edge elements (e.g., **A->B**, **C->D**).
- The **`.head()` step** processes each edge.
- The **Output Stream** contains the corresponding **target (head) vertices** (**B**, **D**) for each input edge.

## Syntax

```rust,noplayground
walker.head()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker positioned at the **target** vertices of the edges in the current traversal.

## Example

Find projects created by people known by a starting person:

```rust,noplayground
{{#include head/head_example.rs:head_example}}
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
