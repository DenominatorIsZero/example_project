# AI Demo Template - Development Commands

# Default recipe shows available commands
default:
    @just --list

# Setup and Maintenance Commands

# Install WASM target and required development tools
setup:
    rustup target add wasm32-unknown-unknown
    cargo install wasm-server-runner
    cargo install wasm-bindgen-cli

# Clean all build artifacts
clean:
    cargo clean

# Build Commands

# Build all workspace components (debug)
build:
    cargo build --workspace

# Build all workspace components (release)
build-release:
    cargo build --workspace --release

# Build WASM binary (debug) - for testing with wasm-server-runner
build-wasm:
    cargo build --target wasm32-unknown-unknown --bin interactive

# Build WASM binary (release) - for testing with wasm-server-runner
build-wasm-release:
    cargo build --target wasm32-unknown-unknown --bin interactive --release

# Build web package with wasm-bindgen (for website deployment)
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

# Run Commands

# Run the training binary
train:
    cargo run --bin training

# Run interactive demo natively (not WASM)
interactive:
    cargo run --bin interactive

# Run WASM demo with development server (debug build)
wasm: build-wasm
    wasm-server-runner target/wasm32-unknown-unknown/debug/interactive.wasm

# Run WASM demo with development server (release build)
wasm-release: build-wasm-release
    wasm-server-runner target/wasm32-unknown-unknown/release/interactive.wasm

# Serve built web package locally for testing
serve-web:
    @echo "Starting local server for web package..."
    @echo "Open http://localhost:8000 in your browser"
    @cd interactive/web && python -m http.server 8000

# Code Quality Commands

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
