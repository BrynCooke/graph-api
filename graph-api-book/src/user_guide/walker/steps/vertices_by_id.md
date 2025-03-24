# Vertices By ID Step

The `vertices_by_id` step starts a traversal from a specific set of vertices identified by their IDs. This is typically
the most efficient way to start a traversal when you know exactly which vertices you want.

```pikchr
# Graph structure - vertices and edges with no highlighting, showing IDs
A: box rad 10px width 0.5 height 0.3 "A" "id=1" fill white
B: box same at 1.5 right of A "B" "id=2" fill white
C: box same at 1.5 right of B "C" "id=3" fill white

# Connect vertices with edges
line from A.e to B.w
line from B.e to C.w

TEXT: text at 0.4 below B "Before vertices_by_id(): No traversal position (not started)"

# IDs to look up
IdBox: box height 0.3 width 0.8 at 0.5 below TEXT "[1, 3]" fill lightyellow


# Second diagram - after vertices_by_id step with specific vertices selected
Aprime: box rad 10px width 0.5 height 0.3 at 1.6 below A "A" "id=1" fill lightgreen
Bprime: box same at 1.5 right of Aprime "B" "id=2" fill white
Cprime: box same at 1.5 right of Bprime "C" "id=3" fill lightgreen

arrow from IdBox.s down 0.2 then left until even with Aprime then to Aprime.n rad 10px
arrow from IdBox.s down 0.2 then right until even with Cprime then to Cprime.n rad 10px

# Connect vertices with edges
line from Aprime.e to Bprime.w
line from Bprime.e to Cprime.w

text at 0.4 below Bprime "After vertices_by_id([1,3]): Only matching vertices in traversal"
```

## Syntax

```rust,noplayground
graph.walk().vertices_by_id(ids)
```

Where `ids` is an iterator yielding vertex IDs.

## Parameters

- `ids`: An iterator yielding vertex IDs to include in the traversal

## Return Value

Returns a new walker positioned at the vertices with the specified IDs.

## Examples

### Basic Usage

Start a traversal with specific vertex IDs:

```rust,noplayground
{{#include vertices_by_id/vertices_by_id_examples.rs:basic_usage}}
```

### Following Relationships

Start from a specific vertex and follow its relationships:

```rust,noplayground
{{#include vertices_by_id/vertices_by_id_examples.rs:followed_by_steps}}
```

### Using Dynamically Collected IDs

Use IDs collected from a previous traversal:

```rust,noplayground
{{#include vertices_by_id/vertices_by_id_examples.rs:dynamic_ids}}
```

## Notes

- This step is the most efficient way to start a traversal when you already know the exact vertex IDs
- The traversal will contain vertices in the same order as the IDs in the input iterator
- If an ID doesn't correspond to any vertex in the graph, it is simply skipped (no error is thrown)
- Commonly used for:
    - Following up on previous traversal results
    - Starting from known entry points (e.g., a user's profile vertex)
    - Building multi-hop traversals from a specific starting point