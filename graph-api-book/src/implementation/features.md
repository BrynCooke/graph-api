# Features and Extensions

This chapter covers how to extend your graph implementation with additional features and capabilities beyond the basic
Graph API requirements.

## Core vs. Extended Features

The Graph API defines a set of core features required for all graph implementations, plus optional extended features:

### Required Core Features

All graph implementations must support:

1. **Basic Graph Management**: Adding/removing vertices and edges
2. **Element Access**: Retrieving vertices and edges by ID
3. **Graph Traversal**: Supporting the walker API
4. **Vertex and Edge Types**: Using the client's vertex and edge enum definitions

### Optional Extended Features

Optional features you may want to support:

1. **Indexing**: Various indexing strategies (label, hash, range, full-text)
2. **Graph Clearing**: Efficiently removing all elements
3. **Custom Traversal Steps**: Graph-specific optimization for certain traversals
4. **Persistence**: Saving and loading graphs from storage
5. **Concurrency**: Thread-safe access to graph elements

## Declaring Feature Support

The Graph API uses separate support traits to indicate which features a graph implementation supports:

```rust
// First implement the core Graph trait
impl<Vertex, Edge> Graph for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    // Core Graph functionality
    // ...
}

// Then implement the relevant support traits
impl<Vertex, Edge> SupportsVertexLabelIndex for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    // Any trait-specific methods if needed
}

impl<Vertex, Edge> SupportsEdgeLabelIndex for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    // Any trait-specific methods if needed
}

// Skip implementing SupportsVertexFullTextIndex if not supported
```

## Implementing Optional Features

### Index Support

To add index support to your graph implementation:

1. **Declare Support**: Update your `Graph` implementation to declare support for specific index types.
2. **Create Index Structures**: Implement the appropriate data structures for each index type.
3. **Update Index Maintenance**: Ensure indexes are updated when vertices/edges are added, modified, or removed.

Example for hash index support:

```rust
// Declare support
type SupportsVertexHashIndex = Supported;

// Create index structure
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

### Graph Clearing

To support clearing all elements from a graph, implement the `SupportsClear` trait:

```rust
// First implement the Graph trait
impl<Vertex, Edge> Graph for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    // ...core graph functionality
}

// Then implement the SupportsClear trait
impl<Vertex, Edge> SupportsClear for MyGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    fn clear(&mut self) {
        self.vertices.clear();
        self.edges.clear();
        self.indexes.iter_mut().for_each(|idx| idx.clear());
        // Clear any other data structures
    }
}
```

## Extending the Walker API

You can extend the Walker API for your graph implementation by creating custom steps:

```rust
// Define a custom step extension trait
pub trait CustomStepExt<'g, G: Graph> {
    // A custom step that applies a specific algorithm
    fn my_custom_step(self, param: CustomParam) -> Self;
}

// Implement for the appropriate walker type
impl<'g, G: Graph> CustomStepExt<'g, G> for VertexWalker<'g, G> {
    fn my_custom_step(self, param: CustomParam) -> Self {
        // Implementation that uses the walker's state
        // and applies your custom algorithm
        // ...
    }
}
```

See [custom_step.rs](https://github.com/BrynCooke/graph-api/blob/main/graph-api-lib/examples/custom_step.rs) for a full
example.

## Integration with External Systems

You might want to integrate your graph implementation with external systems:

### Serialization/Deserialization

Add support for saving and loading graphs:

```rust
impl<V, E> MyGraph<V, E>
where
    V: Element + Serialize + for<'de> Deserialize<'de>,
    E: Element + Serialize + for<'de> Deserialize<'de>,
{
    pub fn save<W: Write>(&self, writer: W) -> Result<(), Error> {
        // Serialize the graph
    }

    pub fn load<R: Read>(reader: R) -> Result<Self, Error> {
        // Deserialize the graph
    }
}
```

### Database Integration

Create adapters for database systems:

```rust
pub struct DatabaseBackedGraph<V, E> {
    connection: DbConnection,
    // Other fields for caching, etc.
    _phantom: PhantomData<(V, E)>,
}

impl<V, E> Graph for DatabaseBackedGraph<V, E>
where
    V: Element,
    E: Element,
{
    // Implement Graph trait with database operations
    // ...
}
```

## Feature Compatibility

When implementing optional features, use trait bounds in your API:

```rust
pub fn search_full_text<G>(graph: &G, text: &str) -> Vec<G::VertexId>
where
    G: Graph + SupportsVertexFullTextIndex,
{
    // Can safely use full text search here
    graph.walk()
        .vertices(Vertex::person_by_biography(text))
        .collect()
}
```

## Best Practices for Extensions

When adding features and extensions, follow these best practices:

1. **Maintain Core API Compatibility**: Ensure extensions don't break the core Graph API.
2. **Document Extensions Thoroughly**: Clearly document what extensions are available and how to use them.
3. **Test Extensions Separately**: Write dedicated tests for extended features.
4. **Consider Performance Impact**: Ensure extensions don't negatively impact core operations.

