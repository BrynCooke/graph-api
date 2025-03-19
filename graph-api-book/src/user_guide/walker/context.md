# Context System

The context system is a powerful feature of the Graph API walker interface that allows you to carry data along with your
traversal. This enables sophisticated data collection, transformation, and decision making during graph exploration.

## What is Context?

Context is data that is associated with each element in a traversal. As you move through the graph, you can:

1. Store data in the context
2. Access previously stored context data
3. Transform the context based on graph elements
4. Make decisions using the context

## Basic Context Usage

### Adding Context to a Traversal

```rust,noplayground
// Store a person's age in the context
graph.walk()
    .vertices(VertexSearch::scan().with_label(Person::label()))
    .push_context(|vertex, _| {
        vertex.project::<Person<_>>().unwrap().age()
    })
    // Now the context contains the person's age
```

### Using Context in Later Steps

```rust,noplayground
// Store age and use it in a filter
graph.walk()
    .vertices(VertexSearch::scan().with_label(Person::label()))
    .push_context(|vertex, _| {
        vertex.project::<Person<_>>().unwrap().age()
    })
    .edges(EdgeSearch::scan().with_label(Edge::knows_label()))
    .tail()
    .filter(|_, ctx| {
        // Only keep people who know someone older than themselves
        let friend_age = ctx.project::<Person<_>>().unwrap().age();
        *ctx > friend_age
    })
```

## Context Nesting

When you use operations like `detour`, context becomes nested. The parent context is still accessible through the
context's parent reference:

```rust,noplayground
graph.walk()
    .vertices(VertexSearch::scan().with_label(Person::label()))
    .push_context(|vertex, _| {
        vertex.project::<Person<_>>().unwrap().age()
    })
    .detour(|v| {
        v.edges(EdgeSearch::scan().with_label(Edge::knows_label()))
        .tail()
        .push_context(|friend, parent_ctx| {
            // Access the original person's age via parent_ctx
            let friend_age = friend.project::<Person<_>>().unwrap().age();
            let original_age = *parent_ctx;
            
            // Store the age difference
            friend_age as i64 - original_age as i64
        })
    })
```

## Default Context

The Graph API provides built-in default contexts for common use cases:

```rust,noplayground
// Use default vertex context (contains vertex ID and weight)
graph.walk()
    .vertices(VertexSearch::scan())
    .push_default_context()
    .map(|_, ctx| {
        // Access vertex data directly from context
        let vertex_id = ctx.vertex_id;
        let vertex_data = &ctx.vertex;
        // ...
    })
```

## Transforming Context

You can transform context data as you traverse:

```rust,noplayground
graph.walk()
    .vertices(VertexSearch::scan().with_label(Person::label()))
    .push_context(|v, _| v.project::<Person<_>>().unwrap().name().to_string())
    .map(|v, ctx| {
        // Original context contains the name
        format!("Person named: {}", ctx)
    })
```

## Context Lifecycles

1. **Creation**: Initial context is created with first context step
2. **Propagation**: Context is passed along as you traverse the graph
3. **Transformation**: Context can be modified at any step
4. **Nesting**: Sub-traversals create nested contexts
5. **Use**: Context is used in terminal operations

## Performance Considerations

- Context data is cloned when passing between steps
- Use reference types or small data structures for better performance
- For large data, consider using indices or IDs instead of full objects

## Example: Building Complex Data Structures

```rust,noplayground
// Build a map of people to their projects
let person_projects = graph.walk()
    .vertices(VertexSearch::scan().with_label(Person::label()))
    .push_context(|v, _| {
        // Store a tuple of person's name and empty project list
        (
            v.project::<Person<_>>().unwrap().name().to_string(),
            Vec::<String>::new()
        )
    })
    .detour(|v| {
        v.edges(EdgeSearch::scan().with_label(Edge::created_label()))
        .tail()
        .push_context(|project_v, parent_ctx| {
            // Get the person's name and project list from parent context
            let (name, mut projects) = parent_ctx.clone();
            
            // Add this project to the list
            projects.push(project_v.project::<Project<_>>().unwrap().name().to_string());
            
            // Return updated tuple
            (name, projects)
        })
    })
    .map(|_, ctx| {
        // Convert to HashMap entry
        let (name, projects) = ctx;
        (name, projects)
    })
    .collect::<HashMap<String, Vec<String>>>();
```