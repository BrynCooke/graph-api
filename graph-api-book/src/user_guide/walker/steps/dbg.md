# Debug Step

The `dbg` step prints detailed information about the current traversal state, making it easier to debug complex graph
operations. Like a checkpoint in your traversal, it lets you see what's happening without changing the traversal flow.

<object type="image/svg+xml" data="dbg/image.svg" width="500" height="250">
Debug step diagram showing traversal state being printed to console
</object>

## Syntax

```rust,noplayground
walker.dbg("Custom label")
```

## Parameters

- `label`: An optional string label to identify the debug output.

## Return Value

Returns the same traversal unchanged, allowing you to continue chaining steps.

## Example

```rust,noplayground
{{#include dbg/dbg_example.rs:all}}
```

## Best Practices

- Use meaningful labels to identify each debug checkpoint in complex traversals
- Remove or comment out debug steps before deploying to production
- Add debug steps before and after steps that might be causing issues
- For production logging, replace with `probe` steps that have custom formatting

## Common Use Cases

- **Traversal troubleshooting**: Understanding why a traversal isn't returning expected results
- **Learning**: Exploring how traversals work by seeing each element's detailed state
- **Development checkpoints**: Verifying the state of a traversal at key points
- **Context inspection**: Examining the full context structure during traversal
