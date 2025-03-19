# Detour Step

The `detour` step allows you to apply a sub-traversal to the current traversal, providing a way to create more complex
traversal patterns.

## Syntax

```rust,noplayground
walker.detour(|sub_walker| {
    // Configure the sub-walker
    sub_walker.filter(...).another_step(...)
})
```

## Parameters

- `detour_fn`: A function that takes the current walker and returns a modified walker.

## Return Value

Returns a new walker that continues from where the detour ends.

## Diagram

```
Before:
  Graph: [A]---[B]---[C]
  Position: [A]*, [B]*, [C]* (all vertices)

During detour (filter where age > 30):
  Sub-traversal: [A]*, [C]* (only vertices with age > 30)

After:
  Graph: [A]---[B]---[C]
  Position: [A]*, [C]* (continues with the filtered result)
```

## Example

```rust,noplayground
{{#include detour/detour_example.rs:all}}
```

## Notes

- The detour step is like temporarily branching into a sub-traversal
- Allows for complex traversal logic that would be difficult with a single chain
- Perfect for conditional traversal paths or complex filtering scenarios
- The sub-traversal receives the current walker position
- Any modifications to the traversal (filtering, etc.) persist after the detour
- Can be nested for highly complex traversal patterns
- Helpful for readability when traversal logic becomes complex
- Returns the same type of walker, allowing you to chain additional steps