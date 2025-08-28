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

# Run interactive demo on native
interactive:
    cargo run --bin interactive

# Run interactive demo with WASM development server
demo:
    cargo run --target wasm32-unknown-unknown --bin interactive

# Run optimized WASM demo (size-optimized release build)
demo-release:
    cargo build --target wasm32-unknown-unknown --bin interactive --release
    wasm-server-runner target/wasm32-unknown-unknown/release/interactive.wasm

# Build web package with wasm-bindgen for deployment
build-web:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building release WASM for web deployment..."
    cargo build --target wasm32-unknown-unknown --bin interactive --release
    echo "Creating web directory..."
    mkdir -p interactive/web
    echo "Generating JavaScript wrapper with wasm-bindgen..."
    wasm-bindgen --no-typescript --target web \
        --out-dir ./interactive/web/ \
        --out-name "interactive" \
        ./target/wasm32-unknown-unknown/release/interactive.wasm
    echo "Web package ready in interactive/web/"
    echo "Files generated:"
    ls -lh interactive/web/

# Format all code
fmt:
    cargo fmt --all

# Run clippy linting
lint:
    cargo clippy --workspace -- -D warnings

# Run tests
test:
    cargo test --workspace

# Run all checks (format, lint, test)
check: fmt lint test


# Install required tools for development
install-deps:
    rustup target add wasm32-unknown-unknown
    cargo install wasm-server-runner
    cargo install wasm-bindgen-cli
