# Full-text Indexes

Full-text indexes enable powerful text search capabilities, allowing you to find vertices containing specific words or
phrases within a text field.

## What are Full-text Indexes?

A full-text index is a specialized index that processes text fields to enable efficient searching based on word content.
Unlike standard indexes that require exact matches, full-text indexes allow you to find vertices that contain specific
words, regardless of their position within the text.

Consider an index on a `description` property:
<object type="image/svg+xml" data="./full_text_index/image.svg" title="Diagram showing a full-text index mapping words to graph vertices, highlighting a search for 'traversal'"></object>

In this diagram:

- The **graph** on the right has vertices (A, B, C) with text `description` properties.
- The **full-text index** on the left is an inverted index:
    - It lists processed **tokens** (like 'fast', 'graph', 'traversal').
    - For each token, it points to the **vertices** (`A`, `B`, `C`) whose `description` contains that token after
      processing. Note how 'graph' points to both `A` and `B`.
- When a **query** like `description CONTAINS 'traversal'` is performed:
    - The index is used to look up the token 'traversal'.
    - The index directly provides the list of matching vertices: `[ B, C ]`.
- The **orange highlighting** shows the index entry for 'traversal' being used and the resulting vertices (`B`, `C`)
  identified in the graph.
- **Blue arrows** point from the selected index entry to the corresponding graph vertices.

This approach is fundamental to searching documentation, product descriptions, user comments, or any unstructured text
associated with graph elements.

## Defining Full-text Indexes

In Graph API, you define a full-text index by using the `#[index(full_text)]` attribute on string fields:

```rust,noplayground
{{#include full_text_index/full_text_index_example.rs:define_full_text_index}}
```

## How Full-text Indexes Work

Behind the scenes, full-text indexes:

1. Process text by splitting into words (tokenization)
2. Normalize words (lowercasing, removing punctuation)
3. Create an inverted index mapping words to vertices
4. Enable efficient lookup by word or phrase

## Querying with Full-text Indexes

Full-text indexes dramatically simplify text search operations:

```rust,noplayground
{{#include full_text_index/full_text_index_example.rs:full_text_queries}}
```

## Performance Benefits

Full-text indexes provide significant advantages for text search:

1. **Efficient keyword matching**: Find text containing specific words without scanning
2. **Reduced memory requirements**: Only load relevant vertices
3. **Better user experience**: Enable natural language search patterns
4. **Improved relevance**: Return results based on word presence rather than exact matches

## When to Use Full-text Indexes

Full-text indexes are ideal for:

- **Content search**: Articles, posts, descriptions
- **User profiles**: Biographies, skills, interests
- **Product descriptions**: Features, benefits, specifications
- **Documentation**: API details, manuals, guides
- **Search functionality**: Implementing search features in your application

## Best Practices

When using full-text indexes:

1. **Choose appropriate fields**: Apply to content-rich text fields
2. **Consider search patterns**: Think about how users will search
3. **Balance with standard indexes**: Use standard indexes for fields requiring exact matches
4. **Be mindful of size**: Full-text indexes can be larger than standard indexes

## Limitations

Full-text indexes have some limitations:

1. **String fields only**: Only applicable to string properties
2. **Implementation dependent**: Search capabilities vary by graph implementation
3. **Tokenization limitations**: Basic word splitting may not handle all languages equally
4. **Update complexity**: Maintaining the index adds overhead during updates

For range-based queries, see [range indexes](./range_index.md), and for combining different index types,
see [combining indexes](./combining_indexes.md).
