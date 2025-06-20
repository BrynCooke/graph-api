# Walker Best Practices

This page provides guidance on effectively using the Graph API walker system for graph traversals.

## General Best Practices

1. **Chain steps logically**
    - Build your traversal in a logical sequence that mirrors how you would describe the path
    - Group related operations together to improve readability

2. **Use appropriate search criteria**
    - Limit vertices and edges early in the traversal to reduce the traversal set
    - Use the most specific search criteria available (label, index, property)

3. **Leverage type projections**
    - Use `.project::<Type<_>>()` to access type-specific methods
    - Handle projection failures gracefully with `match` or `if let`

4. **Use context for data collection**
    - Store intermediate results in context rather than using external collections
    - Use context to carry state through the traversal

5. **Consider performance**
    - For very large graphs, filter early to reduce the traversal set
    - Use indexed lookups when available
    - Limit traversal depth for potentially unbounded searches

## Optimization Tips

### Managing Compile Times

The Graph API's walker system uses deeply nested generic types that can lead to slow compilation in complex applications. Here are strategies to optimize build performance:

#### Use the Boxed Step

For complex walker chains, use the [`boxed()`](steps/boxed.md) step to reduce monomorphization and improve compile times:

```rust,noplayground
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use std::collections::HashSet;
#
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);
// Use boxed() for complex traversals to improve compilation
let result = graph.walk()
    .vertices(VertexSearch::scan())
    .edges(EdgeSearch::scan())
    .boxed()  // ← Breaks type complexity
    .head()
    .edges(EdgeSearch::scan())
    .boxed()  // ← Further reduces complexity
    .head()
    .collect::<Vec<_>>();
```

**When to use `boxed()`:**
- After 4+ chained operations
- When compile times become slow
- At logical checkpoints in long traversals
- When storing walkers in collections

**Trade-offs:**
- ✅ Faster compilation and smaller binaries
- ✅ Enables storage in collections and complex builder patterns
- ✅ Preserves all walker functionality including context operations
- ❌ 5-15% runtime overhead from indirect calls

#### Compile Time Warning Signs

Watch for these indicators that you may need boxing:

- **Slow incremental compilation** - Adding a new walker step takes noticeably longer to compile
- **Large binary sizes** - Your application binary grows significantly with each new walker chain
- **IDE slowness** - Type hints and autocompletion become sluggish in files with complex walkers
- **Long CI build times** - Continuous integration builds start timing out or taking much longer

#### Strategic Boxing Guidelines

Apply boxing strategically rather than everywhere:

```rust,noplayground
// ❌ Over-boxing - unnecessary for simple chains
let result = graph.walk()
    .vertices(VertexSearch::scan())
    .boxed()  // ← Not needed for simple 2-step chain
    .take(10)
    .collect::<Vec<_>>();

// ✅ Strategic boxing - at logical complexity boundaries
let result = graph.walk()
    .vertices(VertexSearch::scan())
    .edges(EdgeSearch::scan())
    .filter(|e, _| e.label().contains("important"))
    .head()
    .boxed()  // ← Good: after complex traversal logic
    .edges(EdgeSearch::scan())
    .head()
    .edges(EdgeSearch::scan())
    .boxed()  // ← Good: preventing deep nesting
    .head()
    .collect::<Vec<_>>();
```

For more details on the boxed step, see the [Boxed Step documentation](steps/boxed.md).

### Early Filtering

Filter vertices and edges as early as possible in the traversal:

```rust,noplayground
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use std::collections::HashSet;
#
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);
// Efficient - filters early
graph.walk()
    .vertices(VertexSearch::scan().with_label(Person::label()))
    .filter(|v, _| v.project::<Person<_>>().unwrap().age() > 30)
    .edges(...)
    // ... rest of traversal

// Less efficient - processes all edges before filtering
graph.walk()
    .vertices(VertexSearch::scan().with_label(Person::label()))
    .edges(...)
    .head()
    .filter(|v, _| v.project::<Person<_>>().unwrap().age() > 30)
    // ... rest of traversal
```

### Use Indexes

Take advantage of indexes when available:

```rust,noplayground
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use std::collections::HashSet;
#
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);
// Using an index (more efficient)
graph.walk()
    .vertices(VertexSearch::index(Person::by_name_index(), "Bryn"))
    // ... rest of traversal

// Full scan (less efficient)
graph.walk()
    .vertices(VertexSearch::scan())
    .filter(|v, _| {
        if let Ok(person) = v.project::<Person<_>>() {
            person.name() == "Bryn"
        } else {
            false
        }
    })
    // ... rest of traversal
```

### Limit Traversal Size

Use `take()` to prevent processing excessive elements:

```rust,noplayground
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use std::collections::HashSet;
#
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);
// Limit to first 10 results
graph.walk()
    .vertices(VertexSearch::scan())
    .take(10)
    .collect::<Vec<_>>();
```

### Use Detours Effectively

Detours allow for complex traversals without losing your place:

```rust,noplayground
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use std::collections::HashSet;
#
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);
// Find people and their projects with ratings
graph.walk()
    .vertices(VertexSearch::scan().with_label(Person::label()))
    .push_context(|v, _| v.id()) // Store person ID
    .detour(|v| {
        v.edges(EdgeSearch::scan().with_label(Edge::created_label()))
        .tail()
        .push_context(|v, ctx| {
            // Return both the person ID and project
            (ctx.clone(), v.project::<Project<_>>().unwrap().name().to_string())
        })
    })
    .collect::<Vec<_>>();
```

## Common Patterns

### Finding Connected Vertices

```rust,noplayground
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use std::collections::HashSet;
#
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);
// Find all friends of Bryn
let friends = graph.walk()
    .vertices(VertexSearch::index(Person::by_name_index(), "Bryn"))
    .edges(EdgeSearch::scan().with_label(Edge::knows_label()))
    .tail()
    .collect::<Vec<_>>();
```

### Filtering by Properties

```rust,noplayground
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use std::collections::HashSet;
#
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);
// Find all people over 30
let seniors = graph.walk()
    .vertices(VertexSearch::scan().with_label(Person::label()))
    .filter(|v, _| v.project::<Person<_>>().unwrap().age() > 30)
    .collect::<Vec<_>>();
```

### Collecting Property Values

```rust,noplayground
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use std::collections::HashSet;
#
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);
// Collect names of all projects
let project_names = graph.walk()
    .vertices(VertexSearch::scan().with_label(Project::label()))
    .map(|v, _| v.project::<Project<_>>().unwrap().name().to_string())
    .collect::<Vec<_>>();
```

### Computing Aggregates

```rust,noplayground
# use graph_api_test::Vertex;
# use graph_api_test::Edge;
# use graph_api_test::VertexExt;
# use graph_api_test::EdgeExt;
# use graph_api_test::Person;
# use graph_api_test::Project;
# use graph_api_test::populate_graph;
# use graph_api_lib::EdgeSearch;
# use graph_api_lib::VertexSearch;
# use graph_api_simplegraph::SimpleGraph;
# use graph_api_lib::Graph;
# use graph_api_lib::VertexReference;
# use graph_api_lib::EdgeReference;
# use std::collections::HashSet;
#
# // Create a new graph
# let mut graph = SimpleGraph::new();
# // Populate the graph with test data
# let refs = populate_graph(&mut graph);
// Calculate average age of all people
let (sum, count) = graph.walk()
    .vertices(VertexSearch::scan().with_label(Person::label()))
    .fold((0, 0), |(sum, count), v, _| {
        let age = v.project::<Person<_>>().unwrap().age();
        (sum + age, count + 1)
    });
    
let average_age = if count > 0 { sum as f64 / count as f64 } else { 0.0 };
```