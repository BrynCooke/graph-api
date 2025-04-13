# Hash Indexes

Hash indexes are the most common type of index in a property graph. They enable fast lookups for exact matches on
property values, dramatically improving query performance.

## What are Hash Indexes?

A hash index maps property values to vertices, allowing you to quickly find all vertices with a specific property
value without scanning the entire graph. It uses a hash table data structure for O(1) lookups.

<object type="image/svg+xml" data="./hash_index/image.svg" title="Diagram showing a hash index mapping names to graph vertice"></object>

In this diagram:

- The **graph** on the right contains vertices (A, B, C, D) with properties.
- The **index** on the left is specifically for the `name` property.
- The **dashed arrows** show the mapping:
    - Looking up `"Alice"` in the index quickly leads to vertices `A` and `C`.
    - Looking up `"Bob"` leads to vertex `B`.

This allows a query like "find all vertices where name is 'Alice'" to directly access nodes A and C via the index,
instead of checking every vertex in the graph.

## Defining Hash Indexes

In Graph API, you define a hash index by adding the `#[index(hash)]` attribute to a field in your vertex enum:

```rust,noplayground
{{#include hash_index/hash_index_example.rs:model_definition}}
```

## How Hash Indexes Work

When you apply the `#[index(hash)]` attribute to a field:

1. The derive macro generates a hash index entry for that field
2. The graph implementation maintains a hash map from property values to vertices
3. When you query using the index, the graph can directly look up matching vertices in constant time

## Querying with Hash Indexes

The real power of hash indexes becomes apparent when querying the graph:

```rust,noplayground
{{#include hash_index/hash_index_example.rs:hash_index_queries}}
```

## Performance Benefits

Hash indexes offer significant performance advantages:

1. **Constant time lookups**: O(1) rather than O(n) for full scans
2. **Reduced memory pressure**: Only relevant vertices are loaded
3. **Improved scalability**: Performance remains consistent as the graph grows

## When to Use Hash Indexes

Hash indexes are ideal for:

- **Unique identifiers**: User IDs, product codes, etc.
- **Common lookup fields**: Names, titles, categories
- **Fields used in equality predicates**: Where you need exact matches

## Best Practices

When using hash indexes:

1. **Be selective**: Only index fields you frequently query
2. **Choose appropriate fields**: Index fields with high selectivity
3. **Consider cardinality**: Fields with many unique values benefit most from indexing
4. **Balance maintenance cost**: Each index adds storage and update overhead

## Index Limitations

Hash indexes have some limitations:

1. **Exact matches only**: Cannot handle range or partial text queries
2. **Memory overhead**: Each index increases memory usage
3. **Update cost**: Indexes must be maintained when data changes

For range queries or partial text matching, consider [range indexes](./range_index.md)
or [full-text indexes](./full_text_index.md) respectively.