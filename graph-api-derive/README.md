# Graph API Derive

**Welcome to graph-api-derive** — where the type-safe magic happens for your graph models!

This crate provides powerful derive macros that enhance your graph data structures with compile-time safety and rich
query capabilities. Say goodbye to manually writing boilerplate code for working with graph elements!

With **graph-api-derive**, you can transform simple enum declarations into feature-rich graph components with indexing,
type-safe projections, and specialized query helpers.

## What It Does

Give us a model like this:

```rust
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index(hash)]
        name: String,
        #[index(range)]
        age: u64,
        #[index]
        unique_id: Uuid,
        #[index(range)]
        username: String,
        #[index(full_text)]
        biography: String,
    },
    Project { name: String },
    Rust,
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows { since: i32 },
    Created,
    Language { name: String },
}
```

And we'll generate a wealth of useful code for you:

## Generated Goodness

* **Type-Safe Projections**: Access vertex and edge data with confidence using `Person<_>` and `Knows<_>` projections
* **Safe Mutations**: Update properties with automatic index management via `PersonMut` and `KnowsMut`
* **Specialized Query Helpers**: Find exactly what you're looking for with generated index methods:
    * `VertexIndex::person_by_name("Bryn")`: Find people by name
    * `VertexIndex::person_by_age(30..50)`: Find people in an age range
    * `VertexIndex::person_by_biography("graph")`: Find people whose biography mentions "graph"
* **Tailored Edge Traversals**: Navigate your graph with purpose:
    * `EdgeIndex::knows()`: Follow "knows" relationships
    * `EdgeIndex::created()`: Explore creation relationships

## Benefits

* **Compiler-Checked Safety**: Catch errors at compile time, not runtime
* **Improved Readability**: Express your intent clearly with specialized methods
* **Enhanced Productivity**: Focus on your domain logic, not graph mechanics
* **Better Performance**: Use the right index for the right query automatically

Make your graph code more expressive, safer, and more enjoyable to write with graph-api-derive!

Learn more in the [graph-api book](https://bryncooke.github.io/graph-api/).