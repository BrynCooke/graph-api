# Fold Step

The `fold` step accumulates a result by processing each element in a traversal, operating like the standard Rust `fold` operation but specifically for graph traversals. It's a powerful terminal operation that builds a single value from all elements.

```pikchr
# Graph structure with all vertices active in traversal
A: box rad 10px width 0.5 height 0.3 "A" "age=30" fill lightgreen
B: box same at 1 right of A "B" "age=25" fill lightgreen
C: box same at 1 right of B "C" "age=40" fill lightgreen

# Connect vertices with edges
line from A.e to B.w
MID: line from B.e to C.w

text at 0.4 below MID "Before fold(): All vertices in traversal, initial value = 0"

# Show fold operation and result
AccumBox: box width 2.5 height 1.0 at 1.5 below MID fill lightyellow \
  "acc = 0" \
  "acc = 0 + 30 = 30" \
  "acc = 30 + 25 = 55" \
  "acc = 55 + 40 = 95" 

arrow from A.s to 0.4 below A.s
arrow from 0.4 below A.s to AccumBox.nw
arrow from B.s to AccumBox.n
arrow from C.s to 0.4 below C.s
arrow from 0.4 below C.s to AccumBox.ne

# Final result
ResultBox: box width 1.0 height 0.5 at 0.75 below AccumBox "Result: 95" fill lightyellow

arrow from AccumBox.s to ResultBox.n

text at 0.4 below ResultBox "After fold(): Returns final accumulated value (traversal terminates)"
```

## Syntax

```rust,noplayground
walker.fold(initial_value, |accumulator, element, context| {
    // accumulation logic
})
```

## Parameters

- `initial_value`: The starting value for the accumulator
- `f`: A closure that takes:
    - The current accumulator value
    - A reference to the current element (vertex or edge)
    - The current element's context
    - Returns the updated accumulator value

## Return Value

Returns the final accumulated value after processing all elements in the traversal.

## Example

```rust,noplayground
{{#include fold/fold_example.rs:all}}
```

## Notes

- The fold step is a terminal operation - it consumes the traversal and returns a value
- Unlike map, fold doesn't yield a stream of values but a single accumulated result
- The closure is called once for each element with the accumulator and element
- Can be used for many common operations like counting, summing, finding min/max, etc.
- More flexible than specialized steps like count when you need to calculate custom aggregates
- The accumulator can be any type that matches your needs
- Context is available if you need it for your calculations