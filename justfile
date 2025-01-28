_default:
  just --list -u

alias f := format
alias t := test
alias l := lint

# Setup development environment
setup:
  # Install Rust-related tools
  cargo install cargo-binstall
  cargo binstall -y taplo-cli

  # Setup Node.js environment
  corepack enable
  corepack prepare --activate
  yarn

# Format all files
format: format-toml format-rust

# Format TOML files
format-toml:
  taplo format

# Format Rust files
format-rust:
  cargo fmt --all

# Formst JavaScript/TypeScript files
format-js:
  yarn biome format --wfite

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

# Start docs development
docs:
  yarn workspace docs run dev
