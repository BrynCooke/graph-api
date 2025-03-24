# Vertices Step

The `vertices` step starts a traversal by selecting a set of vertices from the graph based on specified criteria. This
is typically the first step in a traversal chain.

```pikchr
# Graph structure - vertices and edges with no highlighting
A: box rad 10px width 0.5 height 0.3 "A" fill white
B: box same at 1 right of A "B" fill white
C: box same at 1 right of B "C" fill white

# Connect vertices with edges
line from A.e to B.w
line from B.e to C.w

text at 0.4 below B "Before vertices(): No traversal position (not started)"

# Second diagram - after vertices step with A and C selected
Aprime: box rad 10px width 0.5 height 0.3 at 1 below A "A" fill lightgreen
Bprime: box same at 1 right of Aprime "B" fill white
Cprime: box same at 1 right of Bprime "C" fill lightgreen

# Connect vertices with edges
line from Aprime.e to Bprime.w
line from Bprime.e to Cprime.w

text at 0.4 below Bprime "After vertices(criteria): Position at matching vertices (highlighted green)"
```

## Syntax

```rust,noplayground
graph.walk().vertices(search_criteria)
```

Where `search_criteria` is a `VertexSearch` object or a predefined search from an index.

## Parameters

- `search_criteria`: A `VertexSearch` object that defines the criteria for selecting vertices

## Return Value

Returns a new walker positioned at the vertices matching the search criteria.

## Examples

### Full Scan

When you need to find all vertices in a graph:

```rust,noplayground
{{#include vertices/vertices_examples.rs:scan_all}}
```

### Using a Label Index

For more efficient queries, use label-based indexes:

```rust,noplayground
{{#include vertices/vertices_examples.rs:label_index}}
```

### Property-Based Filtering

Find vertices based on their properties:

```rust,noplayground
{{#include vertices/vertices_examples.rs:property_filter}}
```

### Combined Filtering

Chain multiple conditions for complex queries:

```rust,noplayground
{{#include vertices/vertices_examples.rs:combined_filter}}
```

## Notes

- The `vertices` step is typically the first step in a traversal
- Performance varies based on the search criteria:
    - Using a label index (e.g., `VertexIndex::person()`) is faster than a full scan
    - Property-based searches are efficient when properly indexed
    - Full scans with `VertexSearch::scan()` should be avoided for large graphs
- The order of vertices in the traversal is not guaranteed unless using an range index