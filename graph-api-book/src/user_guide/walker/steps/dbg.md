# Debug Step

The `dbg` step provides a simple way to inspect the elements flowing through a traversal without altering them. It prints detailed information about each element to the console as it passes through the step.

<object type="image/svg+xml" data="dbg/image.svg" title="Dbg Step Diagram">
Debug step diagram showing elements flowing through, console output, and unchanged output stream
</object>

In this diagram:

- **Input Elements**: The walker starts with elements **A, B, C**.
- The **`.dbg("Label")`** step processes each element.
- **Console Output**: As each element (A, B, C) passes through the `dbg` step, its details are printed to the console, prefixed with the provided label. This is shown as a side effect.
- **Output Elements (Unchanged)**: The elements **A, B, C** continue to the next step in the traversal, completely unaffected by the `dbg` operation.

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
