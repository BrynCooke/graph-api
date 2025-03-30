# Tail Step

The `tail` step navigates from edges to their target (destination) vertices, allowing traversal to where the edges point
to.

<object type="image/svg+xml" data="head_tail/image_tail.svg">
Tail step diagram showing traversal from edge to target vertex
</object>

## Syntax

```rust,noplayground
walker.tail()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker positioned at the target vertices of the edges in the current traversal.

## Examples

### Basic Tail Step

Find projects created by following "created" edges to their target:

```rust,noplayground
{{#include head_tail/head_tail_examples.rs:tail_example}}
```

### Multi-Step Traversal

Traverse multiple relationships to find indirect connections:

```rust,noplayground
{{#include head_tail/head_tail_examples.rs:multi_step}}
```

## Best Practices

- Use tail() to follow relationships in their natural direction
- Chain edge-tail sequences for multi-hop traversals
- Maintain context information when necessary to preserve edge properties
- Consider traversal depth carefully in highly connected graphs

## Common Use Cases

- **Following relationships**: Finding what vertices are connected to your starting points
- **Multi-hop traversals**: Discovering indirect connections through multiple relationships
- **Graph exploration**: Navigating through the graph in a directed manner
- **Social network queries**: Implementing patterns like "friends of friends" or "recommendations"
