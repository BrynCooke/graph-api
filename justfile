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
    mdbook build graph-api-book

serve-book: book
    mdbook serve graph-api-book -o

lint: build test doc
    cargo fmt --check
    cargo clippy -- --deny warnings

bench-simplegraph:
    cargo bench -p graph-api-simplegraph -- --save-baseline simplegraph

bench-petgraph:
    cargo bench -p graph-api-petgraph -- --save-baseline petgraph

update-book-versions:
    #!/usr/bin/env bash
    set -euo pipefail
    
    # Extract versions from each Cargo.toml file
    LIB_VERSION=$(grep -m 1 "^version =" graph-api-lib/Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    DERIVE_VERSION=$(grep -m 1 "^version =" graph-api-derive/Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    SIMPLEGRAPH_VERSION=$(grep -m 1 "^version =" graph-api-simplegraph/Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    PETGRAPH_VERSION=$(grep -m 1 "^version =" graph-api-petgraph/Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    TEST_VERSION=$(grep -m 1 "^version =" graph-api-test/Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    BENCHES_VERSION=$(grep -m 1 "^version =" graph-api-benches/Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    
    # Log extracted versions
    echo "Found versions:"
    echo "  graph-api-lib: $LIB_VERSION"
    echo "  graph-api-derive: $DERIVE_VERSION"
    echo "  graph-api-simplegraph: $SIMPLEGRAPH_VERSION"
    echo "  graph-api-petgraph: $PETGRAPH_VERSION"
    echo "  graph-api-test: $TEST_VERSION"
    echo "  graph-api-benches: $BENCHES_VERSION"
    
    # Update all version variables in book.toml
    sed -i.bak "s/^version = \".*\".*$/version = \"$LIB_VERSION\"  # Legacy variable - will be removed once all md files are updated/" graph-api-book/book.toml
    sed -i.bak "s/^lib_version = \".*\"$/lib_version = \"$LIB_VERSION\"/" graph-api-book/book.toml
    sed -i.bak "s/^derive_version = \".*\"$/derive_version = \"$DERIVE_VERSION\"/" graph-api-book/book.toml
    sed -i.bak "s/^simplegraph_version = \".*\"$/simplegraph_version = \"$SIMPLEGRAPH_VERSION\"/" graph-api-book/book.toml
    sed -i.bak "s/^petgraph_version = \".*\"$/petgraph_version = \"$PETGRAPH_VERSION\"/" graph-api-book/book.toml
    sed -i.bak "s/^test_version = \".*\"$/test_version = \"$TEST_VERSION\"/" graph-api-book/book.toml
    sed -i.bak "s/^benches_version = \".*\"$/benches_version = \"$BENCHES_VERSION\"/" graph-api-book/book.toml
    
    # Remove backup files
    rm graph-api-book/book.toml.bak
    
    echo "Updated book.toml versions"
