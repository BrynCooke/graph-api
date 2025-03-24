# Head Step

The `head` step navigates from edges to their source (origin) vertices, allowing traversal back to where the edges start
from.

```pikchr
# Graph structure - vertices with edge highlighted
A: box rad 10px width 0.5 height 0.3 "A" fill white
B: box same at 1.5 right of A "B" fill white

# Connect vertices with labeled edges, highlight follows edge
FOLLOWS: arrow thick color green from A.e to B.w "follows" above

text at 0.4 below FOLLOWS "Before head(): Position at 'follows' edge (highlighted green)"

# Second diagram - after head step, highlight source vertex
Aprime: box rad 10px width 0.5 height 0.3 at 1 below A "A" fill lightgreen
Bprime: box same at 1.5 right of Aprime "B" fill white

# Connect vertices with labeled edges
FOLLOWS: arrow from Aprime.e to Bprime.w "follows" above

text at 0.4 below FOLLOWS "After head(): Position at source vertex A (highlighted green)"
```

## Syntax

```rust,noplayground
walker.head()
```

## Parameters

This step takes no parameters.

## Return Value

Returns a new walker positioned at the source vertices of the edges in the current traversal.

## Example

Find people who created projects by getting back to the source vertex:

```rust,noplayground
{{#include head_tail/head_tail_examples.rs:head_example}}
```

## Notes

- The `head` step can only be used after an edge traversal step
- For directed edges, the "head" is the source or origin vertex
- This step is useful for finding:
    - Who created something (when traversing incoming "created" edges)
    - The starting point of a relationship
    - The author of an action
- Common pattern: `vertices(...).edges(...).head()` finds vertices with specific outgoing edges