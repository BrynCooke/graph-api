build:
    cargo build

test:
    cargo nextest run -p graph-api-lib
    cargo nextest run -p graph-api-derive
    cargo nextest run -p graph-api-simplegraph
    cargo nextest run -p graph-api-petgraph

lint:
    cargo clippy

