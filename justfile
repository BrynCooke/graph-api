build:
    cargo build

test:
    cargo nextest run -p graph-api-lib --no-tests=pass
    cargo nextest run -p graph-api-derive
    cargo nextest run -p graph-api-simplegraph
    cargo nextest run -p graph-api-petgraph
    cargo test --examples

book:
    cargo test -p graph-api-book --doc

lint:
    cargo fmt --check
    cargo clippy -- --deny warnings
