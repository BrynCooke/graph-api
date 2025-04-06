# ControlFlow Step

The `control_flow` step provides precise control over traversal by evaluating each element with a predicate that returns a `std::ops::ControlFlow` value. This allows for both filtering and conditional traversal termination in a single operation.

<object type="image/svg+xml" data="control_flow/image.svg" title="ControlFlow Step Diagram">
ControlFlow step diagram showing elements being kept, skipped, or causing traversal termination
</object>

In this diagram:

- **Before `control_flow()`**: The walker contains all highlighted vertices **A**, **B**, and **C**.
- The **`.control_flow(predicate)`** step is applied, which makes decisions for each vertex:
  - Continue and keep the vertex (Vertex **B**)
  - Continue but skip the vertex (Vertex **A**, which is faded)
  - Break traversal and optionally keep a final vertex (Vertex **C**, which terminates the traversal)

## Syntax

```rust,noplayground
walker.control_flow(|element, context| /* control flow logic */)
```

## Parameters

- `predicate`: A function that takes a reference to a graph element and a mutable reference to its context, and returns a `std::ops::ControlFlow` value:
  - `ControlFlow::Continue(Some(element))`: Include the element and continue traversal
  - `ControlFlow::Continue(None)`: Skip the element and continue traversal
  - `ControlFlow::Break(Some(element))`: Include the element and stop traversal
  - `ControlFlow::Break(None)`: Stop traversal without including any more elements

## Return Value

Returns a new walker that applies the control flow logic to the traversal.

## Examples

### Vertex Control Flow

```rust,noplayground
{{#include control_flow/control_flow_examples.rs:vertex_example}}
```

### Edge Control Flow

```rust,noplayground
{{#include control_flow/control_flow_examples.rs:edge_example}}
```

## Best Practices

- Use `control_flow` when you need to conditionally terminate traversal based on finding specific elements
- Prefer `filter` for simple inclusion/exclusion if you don't need to stop traversal
- Use the context parameter to track state or accumulate data during traversal
- Return `Break` as soon as you find what you're looking for to optimize performance

## Common Use Cases

- **Early termination**: Stop traversal as soon as a match is found
- **Conditional processing**: Apply different logic based on element properties
- **Limited collection**: Gather elements until a specific condition is met
- **State-driven traversal**: Use context to make decisions based on previously seen elements
- **Performance optimization**: Avoid unnecessary traversal when a sufficient result is found