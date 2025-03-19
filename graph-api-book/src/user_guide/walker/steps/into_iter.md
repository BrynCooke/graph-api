# Into Iterator

The `into_iter` step converts a traversal into a standard Rust iterator, allowing you to use all of Rust's iterator
methods directly with graph traversals.

## Syntax

```rust,noplayground
walker.into_iter()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a standard Rust iterator that yields vertex or edge IDs from the traversal.

## Diagram

```bob
Before step:
  [A]* --- [B]* --- [C]*
  Position: Traversal with walker API

After step:
  [A] --- [B] --- [C]
  Result: Standard Rust iterator yielding IDs
  Position: Traversal converted to Rust Iterator
```

## Examples

### Basic Usage

Convert a traversal to an iterator and collect results:

```rust,noplayground
{{#include into_iter/into_iter_example.rs:basic_usage}}
```

### Filtering with Standard Iterators

Use standard Rust iterator methods:

```rust,noplayground
{{#include into_iter/into_iter_example.rs:filtering}}
```

### Comparing with Walker Methods

Walker methods vs standard iterator methods:

```rust,noplayground
{{#include into_iter/into_iter_example.rs:comparison}}
```

## Notes

- The `into_iter` step is terminal - it consumes the walker and returns a standard Rust iterator
- The iterator yields vertex or edge IDs, not references to graph elements
- You can use all standard Rust iterator methods (filter, map, fold, etc.) on the resulting iterator
- Unlike walker steps like `map` and `filter`, standard iterator methods don't have access to graph context
- To access full vertex or edge data, you need to look up the elements by ID using the graph methods
- Useful when you:
    - Need to integrate with code that expects standard iterators
    - Want to use iterator adaptors that aren't available as walker steps
    - Prefer working with IDs rather than references for efficiency
    - Need to collect or transform graph elements using Rust's extensive iterator ecosystem
