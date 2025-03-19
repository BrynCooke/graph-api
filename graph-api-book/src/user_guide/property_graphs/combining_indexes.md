# Combining Indexes

The true power of Graph API's indexing system emerges when combining different index types with traversal operations.
This approach enables complex, efficient queries across your graph.

## Multi-Index Query Patterns

By applying different index types to various properties, you can create a rich query interface that supports diverse
search patterns:

```rust,noplayground
#![function_body!("combining_indexes.rs", define_combined_indexes)]
```

## Creating Multi-Step Traversals

Graph traversals can start with indexed lookups and then follow relationships to find connected entities:

```rust,noplayground
#![function_body!("combining_indexes.rs", multi_step_traversal)]
```

## Compound Queries

For more complex requirements, you can combine multiple indexed lookups with filtering:

```rust,noplayground
#![function_body!("combining_indexes.rs", compound_queries)]
```

## Hierarchical Data Navigation

When working with hierarchical data structures, indexes can dramatically improve traversal efficiency:

```rust,noplayground
#![function_body!("combining_indexes.rs", hierarchical_navigation)]
```

## Performance Considerations

When combining indexes:

1. **Start with the most selective index**: Begin traversals with the indexes that reduce the result set most
   dramatically
2. **Consider query composition**: Structure queries to take advantage of indexed properties early in the traversal
3. **Balance index coverage**: Too many indexes increase memory use and update overhead
4. **Monitor performance**: Evaluate real-world performance and adjust index strategy based on actual usage patterns

## Index Maintenance Strategy

As your application evolves:

1. **Respond to query patterns**: Add indexes for frequently used query patterns
2. **Remove unused indexes**: Drop indexes that aren't providing value
3. **Monitor index size**: Large indexes may indicate a need to refine indexing strategy
4. **Consider selective indexing**: Not every instance of a property needs to be indexed

## Best Practices

To get the most out of combining indexes:

1. **Design for traversal patterns**: Think about how users will navigate your graph
2. **Layer index types appropriately**: Match index types to data characteristics
3. **Consider the full traversal**: Indexes are most valuable at the start of traversals
4. **Balance coverage and overhead**: Only index what provides meaningful performance benefits

By thoughtfully combining different index types and traversal operations, you can create a highly efficient property
graph that supports complex, real-world query patterns.