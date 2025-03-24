# Edges Step

The `edges` step traverses from vertices to their connecting edges, allowing navigation along relationships in the
graph. This step shifts the walker's position from vertices to their adjacent edges.

```pikchr
# Graph structure - vertices
A: box rad 10px width 0.5 height 0.3 "A" fill lightgreen
B: box same at 1.5 right of A "B" fill white


# Connect vertices with labeled edges
LIKES: arrow from A.e to B.w "likes" above

text at 0.4 below LIKES "Before edges(): Position at vertex A (highlighted green)"

# Second diagram - after edges step
Aprime: box rad 10px width 0.5 height 0.3 at 1 below A "A" fill white
Bprime: box same at 1.5 right of Aprime "B" fill white

# Connect vertices with labeled edges, highlight the edge
LIKES: arrow thick color green from Aprime.e to Bprime.w  "likes" above 

text at 0.4 below LIKES "After edges(): Position changes to 'likes' edge (highlighted green)"
```

## Syntax

```rust,noplayground
walker.edges(search_criteria)
```

Where `search_criteria` is an `EdgeSearch` object or a predefined search from an index.

## Parameters

- `search_criteria`: An `EdgeSearch` object that defines criteria for selecting edges, including:
    - Edge labels
    - Direction (incoming, outgoing, or both)
    - Property values (when supported)

## Return Value

Returns a new walker positioned at the edges matching the search criteria.

## Examples

### Finding All Connected Edges

Get all edges connected to a vertex:

```rust,noplayground
{{#include edges/edges_examples.rs:all_edges}}
```

### Directional Edge Queries

Specify whether you want incoming or outgoing edges:

```rust,noplayground
{{#include edges/edges_examples.rs:directional}}
```

### Label-Based Edge Filtering

Filter edges by their label:

```rust,noplayground
{{#include edges/edges_examples.rs:label_filter}}
```

### Combined Filtering

Combine direction and label filtering:

```rust,noplayground
{{#include edges/edges_examples.rs:combined_filter}}
```

## Notes

- The `edges` step changes the traversal position from vertices to edges
- Use `head()` or `tail()` steps afterward to navigate back to vertices
- Direction matters:
    - `.outgoing()`: Edges where the current vertex is the source
    - `.incoming()`: Edges where the current vertex is the target
    - Default (no direction specified): Both incoming and outgoing edges
- Using label-based indexes (e.g., `EdgeIndex::created()`) is much more efficient than scanning
- Filtering by edge properties is possible but requires those properties to be indexed