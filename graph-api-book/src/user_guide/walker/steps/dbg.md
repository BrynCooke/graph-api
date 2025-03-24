# Debug Step

The `dbg` step prints detailed information about the current traversal state, making it easier to debug complex graph
operations. Like a checkpoint in your traversal, it lets you see what's happening without changing the traversal flow.

```pikchr
# Graph structure with all vertices active in traversal
A: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
B: box same at 1 right of A "B" fill lightgreen
C: box same at 1 right of B "C" fill lightgreen

# Connect vertices with edges
line from A.e to B.w
line from B.e to C.w

# Show mapping transformation with arrows
Debug: box height 0.4 width 1.1 at 0.7 below B   "- Vertex { id: 1, label: Person { name: \"A\" } }" italic \
  "- Vertex { id: 2, label: Person { name: \"B\" } }" italic \
  "- Vertex { id: 3, label: Person { name: \"C\" } }" italic fit fill lightyellow

arrow from A.s down 0.1 then right until even with B then down to Debug.n rad 10px
arrow from B.s to Debug.n
arrow from C.s down 0.1 then left until even with B then down to Debug.n rad 10px


text at 0.5 below Debug "After dbg(): Traversal prints to console"
```

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

## Notes

- The debug step is non-terminal - it allows the traversal to continue unchanged
- Provides more comprehensive output than `probe` - shows complete traversal state
- Perfect for troubleshooting traversals that don't behave as expected
- You can add multiple debug steps at different points in a traversal
- The output format depends on the graph implementation's Debug trait
- Adding labels helps identify where in the traversal the debug output comes from
- For production code, consider using `probe` instead for custom logging
- Has no effect on the actual traversal logic - purely for developer insight