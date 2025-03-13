# Debug Step

The `dbg` step prints detailed information about the current traversal state, making it easier to debug complex graph operations.

## Syntax

```rust
walker.dbg("Custom label")
```

## Parameters

- `label`: An optional string label to identify the debug output.

## Return Value

Returns the same traversal unchanged, allowing you to continue chaining steps.

## Diagram

```
Before:
  Graph: [A]---[B]---[C]
  Position: [A]*, [B]*, [C]* (all vertices)

During dbg:
  Console output:
  [dbg "After filter"] Traversal contains 3 elements:
  - Vertex { id: 1, label: Person { name: "A", age: 32 } }
  - Vertex { id: 2, label: Person { name: "B", age: 28 } }
  - Vertex { id: 3, label: Person { name: "C", age: 45 } }

After:
  Graph: [A]---[B]---[C]
  Position: [A]*, [B]*, [C]* (unchanged)
```

## Example

{% include_fn ./dbg.rs:dbg_example %}

## Notes

- The debug step is non-terminal - it allows the traversal to continue unchanged
- Provides more comprehensive output than `probe` - shows complete traversal state
- Perfect for troubleshooting traversals that don't behave as expected
- You can add multiple debug steps at different points in a traversal
- The output format depends on the graph implementation's Debug trait
- Adding labels helps identify where in the traversal the debug output comes from
- For production code, consider using `probe` instead for custom logging
- Has no effect on the actual traversal logic - purely for developer insight