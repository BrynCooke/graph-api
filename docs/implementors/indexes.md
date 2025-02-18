# Index support

Graph API supports statically defined indexes.
When a user uses `#[derive(VertexExt)]` or `#[derive(EdgeExt)]` on an enum then the `Element` trait is derived.

The `Element` trait statically defines labels and indexes that a Graph may use to provide
fast access to said elements.

## Key traits

Let's take a look at the `Element`, `Label` and `Index` traits:

```rust
pub trait Element: Debug {
    /// Information about the Label for this element
    type Label: Eq + Copy + Hash + Debug + Label + 'static;

    /// Returns the label of the element.
    fn label(&self) -> Self::Label;

    /// Given an index returns the value that is associated with this index.
    fn value(&self, _index: &Self::Index) -> Option<Value> {
        None
    }
}
```

An `Element` is implemented by both `Vertex` and `Edge`. The associated type `Label` contains information about the element that can be introspected at runtime.

A `Label` in a graph is a type of element, for instance a `Person` vertex, or a `Knows` edge.

It is often useful to index elements in a graph so that they can be accessed quickly. For instance, looking up a person by name, or all people that have a biography that contains a keyword. 
The `Label` trait allows graph implementors to determine if an element added to the graph needs indexing.

Each Label can be indexed by more than one index.

```rust
pub trait Label
where
    Self: Sized + Copy + Eq + Hash + Debug,
{
    /// Information about indexes for this label
    type Index: Eq + Copy + Hash + Debug + Index + 'static;

    /// All label variants
    fn variants() -> &'static [Self];

    /// The indexes associated with this label
    fn indexes(&self) -> &'static [Self::Index];

    /// A unique ordinal for this label
    fn ordinal(&self) -> usize;

    /// The name of the label
    fn name(&self) -> &'static str;
}
```

The `Index` trait contains details about how an element is to be indexed.
The current options are:
* default - a regular lookup by value.
* ordered - supports range searches.
* full_text - full text search. (The exact semantics of this need to be tightened up)
```rust
pub trait Index
where
    Self: Sized + Copy + Eq + Hash + Debug,
{
    /// The type of the element being indexed.
    /// Supported types are graph dependant, however all graphs support basic rust type and strings.
    fn ty(&self) -> TypeId;

    /// The index ordinal
    fn ordinal(&self) -> usize;

    /// If the index is ordered
    fn ordered(&self) -> bool;

    /// If the index is full text
    fn full_text(&self) -> bool;
}
```

In addition, the `Element` trait allows you to obtain the label of the element and also the value for any index.

When inserting an element into a graph you can check to see if it is indexed, and update your index datastructures
accordingly.

## Searching your graph

When walking the graph a user may specify either a `VertexSearch` or `EdgeSearch`:

```rust
fn walk() {
    graph.walk()
        .vertices(VertexSearch::new()
                      .labelled(VertexLabel::Person)
                      .indexed(VertexIndex::PersonName, "Bryn"),
        ).collect::<Vec<_>>();
}
```

or the equivalent via the generated shortcut method:

```rust
fn walk() {
    graph.walk()
        .vertices(VertexIndex::person_by_name("Bryn"))
        .collect::<Vec<_>>();
}
```

It is up to you to ensure that your VertexIter and EdgeIter implementations correctly deal with
VertexSearch and VertexEdge data and iterate over only elements that match the search criteria.

## Mutating elements

When an element is mutated it may affect indexes that reference it. To support this the `VertexReferenceMut` and
`EdgeReferenceMut`
contain an associated type `MutationListener`.

This will be called whenever the setter method is called on a mutable projection of an element.

```rust
/// Trait to allow graphs to react to mutation of elements.
/// When an indexed element is updated the mutation listener is called with the index and the before and after values.
pub trait MutationListener<'reference, Element>
where
    Element: crate::Element,
{
    /// Called when a setter is called on a projection of an indexed `Element`.
    fn update(&mut self, index: Element::Index, before: Value, after: Value);
}
```

`MutationListener`s are created at the point where `project_mut` is called. For example here is the `SimpleGraph`
implementation:

```rust
impl<'graph, Graph> graph_api_lib::VertexReferenceMut<'graph, Graph>
for VertexReferenceMut<'graph, Graph>
where
    Graph: graph_api_lib::Graph<VertexId=VertexId> + 'graph,
{
    type MutationListener<'reference> = VertexMutationListener<'reference, Graph::Vertex>;

    fn weight_mut(&mut self) -> &mut Graph::Vertex {
        self.weight
    }

    fn project_mut<
        'reference,
        T: graph_api_lib::ProjectMut<
            'reference,
            <Graph as graph_api_lib::Graph>::Vertex,
            Self::MutationListener<'reference>,
        >,
    >(
        &'reference mut self,
    ) -> Option<T> {
        graph_api_lib::ProjectMut::project_mut(
            self.weight,
            VertexMutationListener {
                phantom_data: Default::default(),
                indexes: self.indexes,
                id: self.id.vertex(),
            },
        )
    }
}
```

`project_mut` creates a mutation listener with all the information that is required to update relevant indexes when
needed.

