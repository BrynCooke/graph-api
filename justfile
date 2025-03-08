build:
    cargo build

test:
    cargo nextest run -p graph-api-lib --no-tests=pass
    cargo nextest run -p graph-api-derive
    cargo nextest run -p graph-api-simplegraph
    cargo nextest run -p graph-api-petgraph

doctest:
    cargo test -p graph-api-lib --doc
    cargo test -p graph-api-derive --doc
    cargo test -p graph-api-simplegraph --doc
    cargo test -p graph-api-petgraph --doc

book:
    cargo test -p graph-api-book --doc

lint:
    cargo clippy

