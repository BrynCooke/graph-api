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
    .vertices(VertexSearch::index(Person::by_name_index(), "Alice"))
    // ... rest of traversal

// Full scan (less efficient)
graph.walk()
    .vertices(VertexSearch::scan())
    .filter(|v, _| {
        if let Ok(person) = v.project::<Person<_>>() {
            person.name() == "Alice"
        } else {
            false
        }
    })
    // ... rest of traversal
```

### Limit Traversal Size

Use `limit()` to prevent processing excessive elements:

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
    .limit(10)
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
// Find all friends of Alice
let friends = graph.walk()
    .vertices(VertexSearch::index(Person::by_name_index(), "Alice"))
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