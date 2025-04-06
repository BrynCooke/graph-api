build:
    cargo build

test:
    cargo nextest run -p graph-api-lib --no-tests=pass
    cargo nextest run -p graph-api-derive
    cargo nextest run -p graph-api-simplegraph
    cargo nextest run -p graph-api-petgraph
    cargo test --examples

doc:
    cargo doc

book:
    cargo test -p graph-api-book --doc
    mdbook serve graph-api-book -o

lint: build test doc
    cargo fmt --check
    cargo clippy -- --deny warnings

bench-simplegraph:
    cargo bench -p graph-api-simplegraph -- --save-baseline simplegraph

bench-petgraph:
    cargo bench -p graph-api-petgraph -- --save-baseline petgraph
