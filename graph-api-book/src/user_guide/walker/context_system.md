# Context System

The context system is a powerful feature of the Graph API that enriches your traversals by carrying data alongside the traversal path. This allows you to maintain state, collect information, and make decisions based on previously visited elements as you explore the graph.

## What is Context?

```pikchr
# After push_context - position unchanged but context added
Aprime: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
Bprime: box same at 1.5 right of Aprime "B" fill white
Cprime: box same at 1.5 right of Bprime "C" fill white

# Connect vertices with labeled edges
arrow from Aprime.e to Bprime.w "Follows" above
arrow from Bprime.e to Cprime.w "Created" above

# Show the context carried with the traversal
ContextBox: box rad 10px height 0.2 width 1.0 fill lightyellow at 0.4 below Aprime "username: alice"

# Connect context to current position
arrow <-> from Aprime.s to ContextBox.n color black

text at 0.7 below Bprime "After push_context: Still at A, but with user data in context"

# Show how context follows the traversal to new positions
Aprime2: box rad 10px width 0.5 height 0.3 at 1.4 below Aprime "A" fill white
Bprime2: box same at 1.5 right of Aprime2 "B" fill lightgreen
Cprime2: box same at 1.5 right of Bprime2 "C" fill white

# Connect vertices with labeled edges
arrow from Aprime2.e to Bprime2.w "Follows" above
arrow from Bprime2.e to Cprime2.w "Created" above

# Show the context still carried with the traversal at new position
ContextBox2: box rad 10px height 0.2 width 1.0 fill lightyellow at 0.4 below Bprime2  "username: alice" 

# Connect context to new position
arrow <-> from Bprime2.s to ContextBox2.n color black

text at 0.7 below Bprime2 "Later in traversal: Context moves with the traversal to B"
```

Context allows you to:

- **Carry information** from previous steps to later steps in the traversal
- **Transform data** as you move through the graph
- **Make decisions** based on a combination of current and past elements
- **Build complex data structures** by accumulating information during traversal
- **Maintain state** without changing the traversal position

## Context Methods

The Graph API provides two primary methods for managing context:

### push_context

```rust,noplayground
// Add or transform context
walker.push_context(|element, current_context| {
    // Extract data from the current element
    // Optionally use the existing context
    // Return a new context value
})
```

This method creates or transforms context based on the current element and any existing context.

### push_default_context

```rust,noplayground
// Add the current vertex or edge as context
walker.push_default_context()
```

This method stores the current vertex or edge in context, making it available in later steps.

## Basic Context Usage

Here's a simple example of storing information in context and using it later:

```rust,noplayground
{{#include context_system/examples.rs:basic_context}}
```

## Nested Context with Detours

When using the `detour` step, context becomes nested, allowing you to access the parent context:

```rust,noplayground
{{#include context_system/examples.rs:nested_context}}
```

## Default Context

The Graph API provides a built-in default context that simplifies common patterns:

```rust,noplayground
{{#include context_system/examples.rs:default_context}}
```

## Type Safety

The context system is fully type-safe:

- Each context value has a specific type determined by your context function
- The compiler enforces correct handling of context types
- Context functions must return values compatible with downstream operations
- Errors in context type handling are caught at compile time, not runtime

## Context Lifecycle

1. **Creation**: Context is created when you first call a context method
2. **Transformation**: Context can be transformed at any step in the traversal
3. **Access**: Any step that accepts a closure can access the context
4. **Nesting**: Detours create nested contexts with access to parent contexts
5. **Immutability**: Context is immutable; transformations create new contexts

## Best Practices

- **Keep context small**: Large contexts can impact performance
- **Use immutable data structures**: Create new contexts rather than modifying existing ones
- **Leverage type safety**: Let the compiler ensure your context manipulations are valid
- **Consider cloning costs**: For large data, use `Arc` or similar for cheap cloning
- **Use default context**: For simple cases, `push_default_context()` is cleaner than custom context
- **Chain context operations**: Build complex data structures through multiple context steps
- **Document context types**: When using complex context chains, document the context type at each step

## Common Use Cases

- **Path tracking**: Record the path taken through the graph
- **Property collection**: Gather properties from different vertices/edges
- **Decision making**: Use information from earlier elements to influence traversal decisions
- **Aggregation**: Build composite results during traversal
- **Statistical analysis**: Calculate metrics as you traverse the graph