# [Step Name] Step

Brief description of what the step does and its purpose in a graph traversal.

## Visual Diagram

Before [step] step:
```
  [A]* --- edge1 ---> [B]* --- edge2 ---> [C]*  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]*                                        
```

After [step] step:
```
  [A]* --- edge1 ---> [B] --- edge2 ---> [C]  
   ^                                         
   |                                         
  edge3                                       
   |                                         
  [D]                                        
```

## Parameters

- `param1`: Description of the first parameter
- `param2`: Description of the second parameter

## Return Value

Description of what the step returns.

## Examples

```rust
// Simple example
let result = graph
    .walk()
    .vertices(VertexSearch::scan())
    .[step_name](param1, param2)
    .collect::<Vec<_>>();

// More complex example showing common use case
let result = graph
    .walk()
    .vertices(VertexSearch::scan().with_label(Vertex::person_label()))
    .[step_name](|element| {
        // Example logic
    })
    .collect::<Vec<_>>();
```

## Notes

- Important note about the step
- Performance considerations
- Common pitfalls to avoid
- Related steps or patterns