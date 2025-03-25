# Range Indexes

Range indexes enable efficient querying for values within a specific range, such as finding people of a certain age
range or products within a price bracket.

## What are Range Indexes?

A range index organizes data in a way that optimizes searches for values within ranges rather than exact matches. This
allows for efficient "greater than," "less than," and "between" queries.

```pikchr
box width 4 height 1 fill lightblue "Graph Database" bold
line down from last.s
box width 5 height 0.6 fill lightyellow "Range Index (age property)" italic
line down from last.s
box width 5 height 2.5 fill lightgray

# Age index entries
Person1: box width 2 height 0.3 at 0.2, 0.3 from last.nw "Age: 21 → [Person ids]" fill white
Person2: box width 2 height 0.3 at 0.2,0.7 from Person1.sw "Age: 28 → [Person ids]" fill white
Person3: box width 2 height 0.3 at 0.2,0.7 from Person2.sw "Age: 35 → [Person ids]" fill white
Person4: box width 2 height 0.3 at 0.2,0.7 from Person3.sw "Age: 42 → [Person ids]" fill white
Person5: box width 2 height 0.3 at 0.2,0.7 from Person4.sw "Age: 50 → [Person ids]" fill white

# Range query visualization
arrow from 3,0.5 to Person2.e "Range Query:" "age ≥ 28 AND age ≤ 42" ljust color green
arrow from 3,1.2 to Person3.e color green
arrow from 3,1.9 to Person4.e color green
```

## Defining Range Indexes

In Graph API, you define a range index by adding the `#[index(range)]` attribute to a field in your vertex or edge enum:

```rust,noplayground
{{#include range_index/range_index.rs:define_range_index}}
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
{{#include range_index/range_index.rs:range_queries}}
```

## Sorting and Pagination

While Graph API doesn't directly support sorted indexes, you can use range indexes to optimize sorted query results:

```rust,noplayground
{{#include range_index/range_index.rs:range_sort}}
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