# Range Indexes

Range indexes extend beyond exact matches, enabling efficient range queries on numeric or otherwise range
properties.

## What are Range Indexes?

An range index maintains properties in a sorted order, allowing efficient lookup of vertices whose property values
fall within a specific range. This is particularly useful for numeric fields like age, date, price, or any other
property that has a natural ordering.

## Defining Range Indexes

In Graph API, you define an range index by using the `#[index(range)]` attribute:

```rust,noplayground
{{#include range_index.rs:define_range_index}}
```

## How Range Indexes Work

The range index works by:

1. Maintaining a sorted data structure (typically a B-tree) of property values
2. Storing vertices in a way that preserves the ordering relationship
3. Enabling efficient traversal of ranges within the sorted structure

## Querying with Range Indexes

Range indexes shine when used for range queries:

```rust,noplayground
{{#include range_index.rs:range_index_queries}}
```

## Performance Benefits

Range indexes provide significant advantages for range-based operations:

1. **Logarithmic lookup time**: O(log n) for finding range boundaries
2. **Efficient range traversal**: Only process vertices within the range
3. **Support for inequality operators**: <, <=, >, >=, not just exact matches
4. **Sorted results**: Results can be returned in sorted order without additional sorting

## When to Use Range Indexes

Range indexes are ideal for:

- **Age ranges**: Finding users between certain ages
- **Date ranges**: Events within a time period
- **Price ranges**: Products within a price band
- **Numeric properties**: Any property where ranges are meaningful
- **Properties used for sorting**: When results need to be returned in a specific order

## Best Practices

When using range indexes:

1. **Use for range queries**: Apply range indexes to fields commonly used in range predicates
2. **Consider data distribution**: Fields with evenly distributed values benefit most
3. **Balance with standard indexes**: Standard indexes are more efficient for exact matches
4. **Be mindful of overhead**: Range indexes typically have higher maintenance costs than standard indexes

## Limitations

Range indexes have some limitations:

1. **Higher overhead**: More complex than standard indexes
2. **Type constraints**: The property must implement Rust's `Ord` trait (for ordering)
3. **Update complexity**: Maintaining sorted order adds overhead during updates

For text search capabilities, see [full-text indexes](./full_text_index.md), and for combining different index types,
see [combining indexes](./combining_indexes.md).