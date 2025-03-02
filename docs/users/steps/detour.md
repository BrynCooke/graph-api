# Detour Step

The `detour` step allows you to create a sub-traversal for each element in the current traversal. It's like a temporary branch in the traversal that returns to the main traversal when complete. This is powerful for exploring connected elements without losing your current position.

## Visual Diagram

Before detour step (traversal position on Person A):
```text
  [Person A]* --- knows ---> [Person B] --- created ---> [Project 1]
                                             
                                             created
                                             
                                             ↓
                                          
                                          [Project 2]
```

During detour execution (for each element, a sub-traversal is performed):
```text
  Main traversal:
  [Person A]* --- knows ---> [Person B]
  
  Sub-traversal from Person A:
  [Person A] --- knows ---> [Person B] --- created ---> [Project 1]*
                                            
                                            created
                                            
                                            ↓
                                          
                                         [Project 2]*
```

After detour step (traversal position returns to original elements):
```text
  [Person A]* --- knows ---> [Person B] --- created ---> [Project 1]
                                             
                                             created
                                             
                                             ↓
                                          
                                          [Project 2]
```

## Parameters

- `traversal_fn`: A function that takes a reference to the current element and returns a new traversal. The results of this traversal are collected in the context.

## Return Value

A walker with the same elements as before, but with the results of the sub-traversals stored in the context.

## Examples

```rust
// Find people and their projects
let people_with_projects = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .detour(|person| {
        // For each person, find their projects
        person.out_edges(EdgeSearch::scan().with_label(Edge::created_label()))
            .tail()  // Move to the target vertices (projects)
    })
    .collect::<Vec<_>>();
    
// Calculate metrics using nested detours
let detailed_analysis = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .push_context(|v, ctx| v.project::<Person<_>>().unwrap().name().to_string())
    .detour(|person| {
        // Find all projects created by this person
        person.out_edges(EdgeSearch::scan().with_label(Edge::created_label()))
            .tail()
            .push_context(|project, ctx| {
                // Add project name to context
                (project.project::<Project<_>>().unwrap().name().to_string(), 0)
            })
            .detour(|project| {
                // For each project, count contributors
                project.in_edges(EdgeSearch::scan().with_label(Edge::created_label()))
                    .head()
                    .map_context(|(name, count), _| (name.clone(), count + 1))
            })
    })
    .collect::<Vec<_>>();
```

## Notes

- The detour doesn't change the main traversal elements - it only adds context data
- Detours can be nested for complex traversals
- The detour function can return any walker, allowing for flexible sub-traversals
- Use `push_context` and `map_context` inside detours to store data from the sub-traversal
- Detours are executed eagerly for each element in the traversal
