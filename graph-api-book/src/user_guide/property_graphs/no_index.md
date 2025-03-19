# Exploring Without Indexes

Before we dive into indexes, let's understand why they're necessary by exploring graph traversal without them.

## The Challenge of Finding Starting Points

In a property graph, one of the biggest challenges is finding the right starting points for your traversal. Without
indexes, your only option is to scan the entire graph, examining each vertex to find matches.

## Example: Graph Without Indexes

Let's consider a simple social network model without any indexes:

```rust,noplayground
{{#include ../standard_model.rs:model_definition}}
```

## Scanning the Entire Graph

Without indexes, the only way to find vertices matching specific criteria is to scan the entire graph:

```rust,noplayground
{{#include no_index.rs:non_indexed_fields}}
```

### Finding by Name (Without Index)

When a field isn't indexed (like `name` in the `Person` vertex), we have to scan all vertices to find matches:

```rust,noplayground
{{#include no_index.rs:scan_for_name}}
```

### Finding Projects (Without Index)

Similarly, to find a project by name, we need to scan the entire graph:

```rust,noplayground
{{#include no_index.rs:scan_for_project}}
```

### Performance Comparison

Let's compare the performance of a full scan versus using an index:

1. **Inefficient approach** - scanning all vertices:

```rust,noplayground
{{#include no_index.rs:comparison_scan}}
```

2. **Efficient approach** - using the index:

```rust,noplayground
{{#include no_index.rs:comparison_index}}
```

## Performance Implications

For small graphs, scanning might be acceptable, but as graphs grow, scanning becomes increasingly inefficient:

1. **Linear Time Complexity**: Scanning requires examining every vertex, making it O(n) where n is the number of
   vertices
2. **Resource Intensive**: Needs to load and process every vertex, even those not matching criteria
3. **Poor Scalability**: Performance degrades linearly as the graph grows

## When to Use Full Scans

Despite their inefficiency, full scans do have legitimate uses:

- **During initial development**: When you're still figuring out your data model
- **For admin or maintenance tasks**: When completeness is more important than speed
- **For small graphs**: When the overhead of maintaining indexes exceeds their benefit
- **For exploratory analysis**: When you need to examine all data without preconceptions

## Moving Beyond Scans

In the following sections, we'll explore how different types of indexes can dramatically improve traversal performance
by allowing you to:

1. **Find vertices by exact property values** using standard indexes
2. **Search for vertices within ranges** using ordered indexes
3. **Find vertices containing specific text** using full-text indexes

Indexes provide the entry points for efficient graph traversal, turning potentially expensive operations into
near-constant time lookups.