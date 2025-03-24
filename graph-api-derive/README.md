# Graph API Derive

Provides enhanced support for querying and mutating a graph.

Given the following model:

```rust
#[derive(Debug, Clone, VertexExt)]
pub enum Vertex {
    Person {
        #[index]
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
    Project(Project),
    Rust,
}

#[derive(Debug, Clone, EdgeExt)]
pub enum Edge {
    Knows { since: i32 },
    Created,
    Language(Language),
}
```

The following is generated:

* `Person`: A projection of a Person vertex that can be used for filtering.
* `PersonMut`: A projection of a Person vertex that can be used to safely mutate a vertex while ensuring that indexes
  are updated.
* `Knows`: A projection of a Knows edge that can be used for filtering.
* `KnowsMut`: A projection of a Knows edge that can be used to safely mutate an edge while ensuring that indexes are
  updated.
* `VertexIndex`: A helper that can be used when searching vertices. This will include generated methods for all indexes
  and labels.
    * `person()`: Person vertices
    * `person_by_name(value)`: People by name
    * `person_by_age(value or range)`: People by age or age range
    * `person_by_unique_id(value)`: People by unique id
    * `person_by_username(value or range)`: people by username or username range
    * `person_by_biography(value)`: People with a biography that contains the value
    * `project()`: Project vertices
    * `rust()`: Rust vertices
* `EdgeIndex`: A helper to that can be used when searching for edges adjacent to a vertex.
    * `knows()`: Knows edges
    * `created()`: Created edges
    * `language()`: Language edges



