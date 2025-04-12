# Implementing Indexes

Indexes are a critical component for efficient graph operations, allowing for quick lookups of elements based on
property values. This chapter guides you through implementing different types of indexes in your graph backend.

## Index Types Overview

The Graph API supports four main types of indexes:

1. **Label Indexes**: Index vertices and edges by their label (enum variant)
2. **Hash Indexes**: Lookup elements by a property value using exact matching
3. **Range Indexes**: Find elements with property values in a specific range
4. **Full-Text Indexes**: Search text properties using more complex matching

## The Index Infrastructure

### The Index Trait

The Graph API defines the `Index` trait to represent element indexes:

```rust
pub trait Index
where
    Self: Sized + Copy + Eq + Hash + Debug,
{
    /// The type of the element being indexed
    fn ty(&self) -> TypeId;

    /// The index ordinal
    fn ordinal(&self) -> usize;

    /// The type of index
    fn index_type(&self) -> IndexType;
}
```

### Index Types

The `IndexType` enum defines the supported index types:

```rust
pub enum IndexType {
    Hash,       // Exact matching
    Range,      // Range queries
    FullText,   // Text search
}
```

### Declare Index Support

To support indexes in your implementation, implement the appropriate support traits:

```rust
impl<Vertex, Edge> Graph for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    // Core Graph functionality
    // ...
}

// Implement support traits for the indexing features you want to provide
impl<Vertex, Edge> SupportsVertexLabelIndex for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{}

impl<Vertex, Edge> SupportsEdgeLabelIndex for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{}

impl<Vertex, Edge> SupportsVertexHashIndex for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{}

impl<Vertex, Edge> SupportsEdgeHashIndex for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{}

impl<Vertex, Edge> SupportsVertexRangeIndex for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{}

impl<Vertex, Edge> SupportsEdgeRangeIndex for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{}

impl<Vertex, Edge> SupportsVertexFullTextIndex for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{}
```

## Implementing Label Indexes

Label indexes allow for quickly finding all vertices or edges of a specific label (enum variant).

### Data Structure

A simple label index can be implemented using a vector of sets:

```rust
/// Stores vertex IDs by label
struct LabelIndex {
    // One entry per label, containing all vertex IDs with that label
    indexes: Vec<HashSet<VertexId>>,
}

impl LabelIndex {
    fn new(label_count: usize) -> Self {
        Self {
            indexes: (0..label_count).map(|_| HashSet::new()).collect(),
        }
    }

    fn insert(&mut self, label: usize, id: VertexId) {
        self.indexes[label].insert(id);
    }

    fn remove(&mut self, label: usize, id: &VertexId) {
        self.indexes[label].remove(id);
    }

    fn get(&self, label: usize) -> impl Iterator<Item=VertexId> + '_ {
        self.indexes[label].iter().copied()
    }
}
```

### Integration with Vertex/Edge Operations

Update label indexes during vertex/edge operations:

```rust
fn add_vertex(&mut self, vertex: Self::Vertex) -> Self::VertexId {
    let label_idx = vertex.label().ordinal();
    let vertex_id = // create a new vertex ID

        // Add to label index
        self.label_index.insert(label_idx, vertex_id);

    // Rest of implementation
    // ...
}
```

## Implementing Hash Indexes

Hash indexes allow for quick lookups by property value using exact matching.

### Data Structure

A hash index maps property values to sets of element IDs:

```rust
struct HashIndex<K, V> {
    map: HashMap<K, HashSet<V>>,
}

impl<K: Hash + Eq, V: Copy + Eq + Hash> HashIndex<K, V> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.map.entry(key).or_default().insert(value);
    }

    fn remove(&mut self, key: &K, value: &V) {
        if let Some(values) = self.map.get_mut(key) {
            values.remove(value);
            if values.is_empty() {
                self.map.remove(key);
            }
        }
    }

    fn get(&self, key: &K) -> impl Iterator<Item=V> + '_ {
        self.map
            .get(key)
            .into_iter()
            .flat_map(|values| values.iter().copied())
    }
}
```

### Integration with Mutation Handling

When a vertex or edge property changes, you need to update the hash indexes:

```rust
pub struct MyMutationListener<'reference, Element> {
    indexes: &'reference mut IndexCollection,
    id: VertexInternalId,
}

impl<'reference, Element> MutationListener<'reference, Element>
for MyMutationListener<'reference, Element>
where
    Element: Element,
{
    fn update(&mut self, index: <Element::Label as Label>::Index, before: Value, after: Value) {
        // Remove the old value from the index
        self.indexes[index.ordinal()].remove(&before, self.id);

        // Add the new value to the index
        self.indexes[index.ordinal()].insert(after, self.id);
    }
}
```

## Implementing Range Indexes

Range indexes allow for finding elements with property values in a specific range.

### Data Structure

A range index typically uses an ordered map like `BTreeMap`:

```rust
struct RangeIndex<K, V> {
    map: BTreeMap<K, HashSet<V>>,
}

impl<K: Ord, V: Copy + Eq + Hash> RangeIndex<K, V> {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.map.entry(key).or_default().insert(value);
    }

    fn remove(&mut self, key: &K, value: &V) {
        if let Some(values) = self.map.get_mut(key) {
            values.remove(value);
            if values.is_empty() {
                self.map.remove(key);
            }
        }
    }

    fn get(&self, key: &K) -> impl Iterator<Item=V> + '_ {
        self.map
            .get(key)
            .into_iter()
            .flat_map(|values| values.iter().copied())
    }

    fn range<'a, R>(&'a self, range: R) -> impl Iterator<Item=V> + 'a
    where
        R: RangeBounds<K> + 'a,
        K: 'a,
    {
        self.map
            .range(range)
            .flat_map(|(_, values)| values.iter().copied())
    }
}
```

### Handling Range Queries

Implement range query support in your vertex and edge iterators:

```rust
fn vertices<'search>(
    &self,
    search: &VertexSearch<'search, Self>,
) -> Self::VertexIter<'search, '_> {
    // ...other search handling

    if let VertexSearch::Range { index, range, .. } = search {
        let index_storage = &self.indexes[index.ordinal()];
        return VertexIter {
            iter: index_storage.range(range.clone(), index),
            // ...other fields
        };
    }

    // ...default handling
}
```

## Implementing Full-Text Indexes

Full-text indexes enable searching through text properties with capabilities like prefix matching, fuzzy matching, or
tokenized searches.

### Data Structure

A simple full-text index implementation might use an inverted index:

```rust
struct FullTextIndex<V> {
    // Maps tokens to element IDs
    tokens: HashMap<String, HashSet<V>>,
    // Maps element IDs to their full text
    contents: HashMap<V, String>,
}

impl<V: Copy + Eq + Hash> FullTextIndex<V> {
    fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            contents: HashMap::new(),
        }
    }

    fn insert(&mut self, id: V, text: &str) {
        // Remove old content if it exists
        self.remove(&id);

        // Store the full text
        self.contents.insert(id, text.to_string());

        // Tokenize the text and add to inverted index
        for token in tokenize(text) {
            self.tokens.entry(token).or_default().insert(id);
        }
    }

    fn remove(&mut self, id: &V) {
        if let Some(text) = self.contents.remove(id) {
            // Remove from token index
            for token in tokenize(&text) {
                if let Some(ids) = self.tokens.get_mut(&token) {
                    ids.remove(id);
                    if ids.is_empty() {
                        self.tokens.remove(&token);
                    }
                }
            }
        }
    }

    fn search(&self, query: &str) -> impl Iterator<Item=V> + '_ {
        // Tokenize the query
        let query_tokens: Vec<_> = tokenize(query).collect();

        // Find matches
        query_tokens
            .into_iter()
            .filter_map(move |token| self.tokens.get(&token))
            .flatten()
            .copied()
            .collect::<HashSet<_>>() // Deduplicate
            .into_iter()
    }
}

// Helper function to tokenize text
fn tokenize(text: &str) -> impl Iterator<Item=String> + '_ {
    text.to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
}
```

### Advanced Full-Text Features

For more advanced full-text search capabilities, consider:

1. **Stemming**: Reducing words to their root form (e.g., "running" → "run")
2. **N-grams**: Creating token sequences for partial matching
3. **Fuzzy Matching**: Supporting approximate matching with edit distance
4. **Relevance Scoring**: Ranking results by relevance to the query

## Efficient Index Updates

For efficient index updates, consider these strategies:

### Lazy Updates

Only update indexes when needed:

```rust
fn remove_vertex(&mut self, id: Self::VertexId) -> Option<Self::Vertex> {
    // Fetch the vertex
    let vertex = self.vertices.get(id.index())?;

    // Only update indexes if the vertex exists
    for index in vertex.label().indexes() {
        self.indexes[index.ordinal()].remove(&self.get_value(vertex, index), id.index());
    }

    // Remove the vertex
    // ...
}
```

### Batched Updates

For bulk operations, batch index updates:

```rust
fn add_vertices_batch(&mut self, vertices: Vec<Self::Vertex>) -> Vec<Self::VertexId> {
    let mut ids = Vec::with_capacity(vertices.len());
    let mut index_updates = Vec::new();

    // First pass: add vertices and collect index updates
    for vertex in vertices {
        let id = self.add_vertex_no_index(vertex);
        ids.push(id);

        // Collect index updates
        for index in vertex.label().indexes() {
            index_updates.push((index, self.get_value(&vertex, index), id));
        }
    }

    // Second pass: apply all index updates
    for (index, value, id) in index_updates {
        self.indexes[index.ordinal()].insert(value, id);
    }

    ids
}
```

## Common Pitfalls

When implementing indexes, watch out for these common issues:

1. **Index Inconsistency**: Ensure indexes are always updated when elements change
2. **Memory Overhead**: Indexes increase memory usage; consider selective indexing
3. **Type Safety**: Ensure index type safety, especially with range and full-text indexes
4. **Empty Sets**: Handle empty index entries to avoid wasted memory
5. **Concurrency**: If supporting concurrent access, protect indexes with appropriate synchronization

## Performance Considerations

For optimal index performance:

1. **Index Selection**: Only index properties that will be frequently queried
2. **Data Structure Choice**: Choose appropriate data structures based on workload
3. **Memory vs. Speed**: Balance memory usage with lookup speed
4. **Measure**: Benchmark index operations to identify bottlenecks

## Testing Indexes

Test your index implementation thoroughly:

```rust
#[test]
fn test_hash_index() {
    let mut graph = MyGraph::new();

    // Add vertices with indexed properties
    let v1 = graph.add_vertex(Vertex::Person {
        name: "Alice".to_string(),
        age: 30,
        // ...other fields
    });

    let v2 = graph.add_vertex(Vertex::Person {
        name: "Bob".to_string(),
        age: 25,
        // ...other fields
    });

    // Test index lookup
    let results = graph.walk()
        .vertices(Vertex::person_by_name("Alice"))
        .collect::<Vec<_>>();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), v1);

    // Test index update after mutation
    graph.vertex_mut(v1).unwrap()
        .project_mut::<PersonMut<_>>().unwrap()
        .set_name("Alicia");

    let results = graph.walk()
        .vertices(Vertex::person_by_name("Alicia"))
        .collect::<Vec<_>>();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), v1);
}
```

## Next Steps

Implementing efficient indexes is key to building a high-performance graph backend. By carefully designing index
structures and ensuring proper updates during mutations, you can provide fast lookup capabilities while maintaining
reasonable memory usage. Remember that different use cases may require different indexing strategies, so consider the
expected query patterns when deciding which indexes to implement and how to optimize them.