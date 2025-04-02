# Range Indexes

Range indexes enable efficient querying for values within a specific range, such as finding people of a certain age
range or products within a price bracket.

## What are Range Indexes?

A range index organizes data in a way that optimizes searches for values within ranges rather than exact matches. This
allows for efficient "greater than," "less than," and "between" queries.

Consider an index on an `age` property:

<object type="image/svg+xml" data="./range_index/image.svg" title="Diagram showing a range index on age, highlighting nodes with age >= 35"></object>

In this diagram:

- The **graph** on the right contains vertices (A, B, C, D) with an `age` property.
- The **range index** on the left stores `(age, vertex)` pairs, crucially **sorted by age**. Notice how `age: 35`
  appears twice, pointing to both `B` and `D`.
- When a **range query** like `age >= 35` is executed:
    - The index efficiently locates the starting point for the value `35`.
    - It then scans forward through the sorted index entries (35, 35, 42) until the condition is no longer met.
    - The vertices associated with these index entries (`B`, `D`, `C`) are identified as the result.
- The **orange highlighting** shows the portion of the index scanned (`age >= 35`) and the resulting vertices (`B`, `D`, `C`) in the graph.
- **Blue arrows** point from the selected index entries to the corresponding graph vertices.

This is much faster than checking the `age` property of every single vertex in the graph for large datasets.

## Defining Range Indexes

In Graph API, you define a range index by adding the `#[index(range)]` attribute to a field in your vertex or edge enum:

```rust,noplayground
{{#include range_index/range_index_example.rs:define_range_index}}
```

## How Range Indexes Work

When you apply the `#[index(range)]` attribute to a field:

1. The derive macro generates a range index entry for that field
2. The graph implementation maintains an ordered data structure mapping property values to vertices
3. When you make a range query, the graph can efficiently find all values within the specified range

Range indexes typically use ordered data structures like B-trees or skip lists to enable efficient range lookups.

## Querying with Range Indexes

The primary benefit of range indexes is the ability to perform efficient range queries:

```rust,noplayground
{{#include range_index/range_index_example.rs:range_queries}}
```

## Sorting and Pagination

While Graph API doesn't directly support sorted indexes, you can use range indexes to optimize sorted query results:

```rust,noplayground
{{#include range_index/range_index_example.rs:range_sort}}
```

## Performance Benefits

Range indexes offer significant performance advantages for range-based queries:

1. **Logarithmic lookup time**: O(log n) rather than O(n) for full scans
2. **Reduced memory pressure**: Only relevant vertices are loaded
3. **Efficient for slices**: Get only the elements within a specific range

## When to Use Range Indexes

Range indexes are ideal for:

- **Numeric properties**: Age, price, quantity, ratings
- **Date/time values**: Timestamps, creation dates, expiration dates
- **Sequential identifiers**: Version numbers, sequential IDs
- **Properties used in inequality filters**: Where you need >, <, >=, <= comparisons

## Best Practices

When using range indexes:

1. **Apply to ordered data**: Only index fields where ordering is meaningful
2. **Consider data distribution**: Range indexes work best with evenly distributed values
3. **Time vs. space tradeoff**: Range indexes may use more space than hash indexes
4. **Use for common queries**: Index fields frequently used in range filters
5. **Choose proper data types**: Use comparable types that have a meaningful ordering

## Index Limitations

Range indexes have some limitations:

1. **Storage overhead**: Typically larger than hash indexes
2. **Update cost**: Maintaining order requires more work when updating
3. **Not for full-text search**: For text search use [full-text indexes](./full_text_index.md) instead

## Supported Types

Range indexes can be applied to:

- **Numeric types**: integers (`u8`, `i32`, etc.), floating-point numbers (`f32`, `f64`)
- **String types**: `String`, `&str` (lexicographic ordering)
- **Date/time types**: When represented as ISO-8601 strings or numeric timestamps
- **Other ordered types**: Any type that implements `Ord` and appropriate serialization
