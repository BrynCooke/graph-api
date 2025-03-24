# First Step

The `first` step retrieves only the first element from a traversal and immediately terminates. This is a terminal
operation that short-circuits the traversal for efficiency.

```pikchr
# Graph structure with all vertices potentially in traversal
A: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
B: box same at 1 right of A "B" fill lightgreen
C: box same at 1 right of B "C" fill lightgreen
D: box same at 1 right of C "D" fill lightgreen

# Connect vertices with edges
line from A.e to B.w
MID: line from B.e to C.w
line from C.e to D.w

text at 0.4 below MID "Before first(): Multiple vertices in traversal (all highlighted)"

# Second diagram - after first() step, only the first element is taken
Aprime: box rad 10px width 0.5 height 0.3 at 1.2 below A "A" fill lightgreen
Bprime: box same at 1 right of Aprime "B" fill white
Cprime: box same at 1 right of Bprime "C" fill white
Dprime: box same at 1 right of Cprime "D" fill white

# Connect vertices with edges
line from Aprime.e to Bprime.w
MID: line from Bprime.e to Cprime.w
line from Cprime.e to Dprime.w

# Show the result with an Option box
ResultBox: box height 0.3 width 1 rad 10px at 0.5 below Bprime "Some(A)" fill lightyellow

# Arrow showing element is taken
arrow from Aprime.s down until even with ResultBox then right to ResultBox.w rad 10px

text at 1 below MID "After first(): Returns Option with first element (traversal terminates)"
```

## Syntax

```rust,noplayground
walker.first()
```

## Parameters

This step takes no parameters.

## Return Value

Returns an `Option` containing the first element from the traversal, or `None` if the traversal is empty.

## Examples

### Basic Usage

Retrieve the first person vertex from the graph:

```rust,noplayground
{{#include first/first_example.rs:basic_usage}}
```

### With Filtering

Get the first person matching specific criteria:

```rust,noplayground
{{#include first/first_example.rs:with_filter}}
```

### Existence Check

Check if any elements match a condition:

```rust,noplayground
{{#include first/first_example.rs:existence_check}}
```

## Notes

- The first step is a terminal operation that consumes the walker
- More efficient than `limit(1).collect()` when you only need one element
- Useful patterns:
    - Getting a single result when you know it exists
    - Checking if any elements match a condition (using `is_some()`)
    - Finding an example of a specific type of vertex or edge
- The "first" element depends on the graph implementation and traversal order
    - If you need a specific element, use filtering or ordering before first