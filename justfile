# AI Demo Scaffolding - Development Commands

# Default recipe shows available commands
default:
    @just --list

# Build all workspace components
build:
    cargo build --workspace

# Build with release optimizations
build-release:
    cargo build --workspace --release

# Run the training binary
train:
    cargo run --bin training

# Build interactive demo for WASM
build-wasm:
    cargo build --target wasm32-unknown-unknown --bin interactive

# Build WASM with release optimizations
build-wasm-release:
    cargo build --target wasm32-unknown-unknown --bin interactive --release

# Run interactive demo with WASM development server
demo:
    cargo run --target wasm32-unknown-unknown --bin interactive

# Format all code
fmt:
    cargo fmt --all

# Check formatting without making changes
fmt-check:
    cargo fmt --all --check

# Run clippy linting
lint:
    cargo clippy --workspace -- -D warnings

# Run tests
test:
    cargo test --workspace

# Run all checks (format, lint, test)
check: fmt-check lint test

# Clean build artifacts
clean:
    cargo clean

# Install required tools for development
install-deps:
    rustup target add wasm32-unknown-unknown
    cargo install wasm-server-runner

# Full development setup from scratch
setup: install-deps
    @echo "Development environment ready!"