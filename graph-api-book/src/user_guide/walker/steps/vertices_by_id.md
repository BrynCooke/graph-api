# Vertices By ID Step

The `vertices_by_id` step starts a traversal from a specific set of vertices identified by their IDs.

## Syntax

```rust,noplayground
graph.walk().vertices_by_id(ids)
```

Where `ids` is an iterator yielding vertex IDs.

## Parameters

- `ids`: An iterator yielding vertex IDs to include in the traversal

## Return Value

Returns a new walker positioned at the vertices with the specified IDs.

## Diagram

```bob
Before step:
  [A(id=1)] --- [B(id=2)] --- [C(id=3)]
  Position: None (traversal not started)

After step (with ids=[1,3]):
  [A(id=1)]* --- [B(id=2)] --- [C(id=3)]*
  Position: Vertices with ID=1 and ID=3
```

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