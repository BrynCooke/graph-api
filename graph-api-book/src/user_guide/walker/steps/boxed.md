# Boxed Step

The `boxed` step performs type erasure to reduce monomorphization and improve compile times. It wraps the current walker in a `SmallBox`, breaking the chain of nested generic types that can grow exponentially in complex traversals.

The boxed walker preserves all functionality including context access and modification, making it a drop-in replacement for any walker step.

<object type="image/svg+xml" data="boxed/image.svg" title="Boxed Step Diagram">
Boxed step diagram showing type complexity reduction through type erasure
</object>

In this diagram:

- An **Input Stream** contains a walker with complex nested types like `Endpoints<Edges<Vertices<Empty>>>`.
- The **`.boxed()` step** performs type erasure, wrapping the walker in a `SmallBox`.
- The **Output Stream** contains a simplified `BoxedVertexWalker` type that hides the complexity.
- **Type Complexity**: Before boxing shows deeply nested generics; after boxing shows a simple erased type.

## Syntax

```rust,noplayground
walker.boxed()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker with erased types that behaves identically to the original but with simplified type signatures.

## Examples

### Basic Type Complexity Reduction

Reduce type complexity in multi-step traversals:

```rust,noplayground
{{#include boxed/boxed_example.rs:basic_boxing}}
```

### Strategic Boxing in Complex Traversals

Use boxing at logical checkpoints in long traversals:

```rust,noplayground
{{#include boxed/boxed_example.rs:complex_traversal}}
```

### Storage in Collections

Enable storage of walkers in data structures:

```rust,noplayground
{{#include boxed/boxed_example.rs:walker_collection}}
```

### Builder Pattern Integration

Use boxing to enable flexible walker construction:

```rust,noplayground
{{#include boxed/boxed_example.rs:builder_pattern}}
```

### Context Preservation

Boxing preserves context functionality - all context operations work normally:

```rust,noplayground
{{#include boxed/boxed_example.rs:context_preservation}}
```

## Best Practices

- Use `boxed()` after 4+ chained operations to prevent type explosion
- Place boxing at logical boundaries in complex traversals (after filtering, major operations)
- Apply when compile times become slow due to deep walker nesting
- Consider boxing when storing walkers in collections or data structures
- Avoid boxing simple 2-3 step chains where type complexity is manageable
- Measure the 5-15% runtime overhead against compilation benefits for your use case

## Common Use Cases

- **Compile-time optimization**: Reducing build times for applications with many complex walker chains
- **Type simplification**: Making complex walker types manageable in library interfaces
- **Collection storage**: Enabling storage of different walker types in Vec or other collections
- **Builder patterns**: Simplifying type signatures in reusable walker construction utilities
- **Library development**: Providing clean APIs without exposing deeply nested generic types