_default:
    just --list -u

alias f := format
alias t := test
alias l := lint
alias bench := benchmarks

# Setup development environment
setup:
    # Install Rust-related tools
    cargo install taplo-cli --locked

    # Setup Node.js environment
    corepack enable
    corepack prepare --activate
    yarn

# Build project
build:
    yarn build

# Format all files
format: format-toml format-rust format-js

# Format TOML files
format-toml:
    taplo format

# Format Rust files
format-rust:
    cargo fmt --all

# Formst JavaScript/TypeScript files
format-js:
    yarn biome format --write

# Test all files
test: test-rust test-js

# Test Rust files
test-rust:
    cargo test --no-fail-fast

# Test JavaScript/TypeScript files
test-js:
    yarn build
    yarn vitest run

# Lint all files
lint: lint-rust lint-js

# Lint Rust files
lint-rust:
    cargo clippy

# Lint JavaScript/TypeScript files
lint-js:
    yarn biome check

# Check type is correct for TypeScript files
typecheck:
    yarn tsc --noEmit

# Start docs development server
docs:
    yarn workspace docs run dev

# Run documentation generator
docs-gen *ARGS:
    yarn workspace docs run gen {{ARGS}}

# Run benchmarks
benchmarks:
    yarn workspace benchmarks run bench
