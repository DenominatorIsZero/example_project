# AI Demo Scaffolding

A minimal AI demonstration project template designed for integration with Rust web applications. This project follows a standardized Cargo workspace structure with separate binaries for model training and interactive WASM demonstration.

## Quick Start

### Prerequisites

- Rust 1.70+ (MSRV)
- [just](https://github.com/casey/just) command runner: `cargo install just`

### Development Commands

```bash
# One-time setup (installs WASM target and tools)
just setup

# Build all workspace components
just build

# Run the training binary
just train

# Run interactive WASM demo (starts server at http://localhost:1334)
just demo

# Code quality
just fmt          # Format code
just lint         # Run clippy
just test         # Run tests
just check        # Run all checks (fmt + lint + test)

# See all available commands
just --list
```

## Project Structure

```
├── Cargo.toml              # Workspace configuration
├── justfile               # Development task automation
├── shared/                # Common library (model definitions, types)
├── training/              # Native binary for model training
├── interactive/           # WASM binary for web demo
│   └── src/
│       └── models/        # Model files for embedded assets (copied from training)
├── models/                # Training output directory (.safetensors files)
└── .cargo/config.toml     # WASM build configuration
```

### Model File Workflow

The training binary creates models in the workspace `models/` directory, then copies them to `interactive/src/models/` for use as embedded assets in the WASM build. This two-step process ensures:

- **Training output**: Workspace `models/` directory for easy access and backup  
- **WASM embedded assets**: `interactive/src/models/` directory required by Bevy's `embedded_asset!` macro
- **Automatic workflow**: Running `just train` handles both saving and copying

## Dependencies

- **Bevy 0.16**: Game engine for interactive demo
- **Candle**: ML framework for model training and inference
- **WebAssembly**: For browser deployment via `wasm-bindgen`

## Model Persistence API

The `shared` crate provides a flexible API for model saving and loading that works across both native and WASM environments:

### Core Functions

```rust
// Parse TOML metadata from bytes
let metadata = parse_model_metadata(toml_bytes)?;

// Load model from raw bytes (WASM-compatible)
let model = load_model_from_data(toml_bytes, safetensors_bytes, &device)?;

// Load model from files (native convenience)  
let model = load_model_from_files("path/to/model", &device)?;

// Save trained model
save_model_from_varmap(&varmap, &metadata, "path/to/model")?;
```

### Usage Patterns

- **Training binary**: Uses `load_model_from_files()` for direct file access
- **Interactive demo**: Uses `load_model_from_data()` with Bevy embedded assets
- **Cross-platform**: Memory-based loading works in both native and WASM environments

## Development Environment

This project is configured for optimal development with Claude Code assistance. The workspace uses modern Cargo features and is optimized for both native and WASM compilation.

### Tooling Configuration

- **justfile**: Task automation for common development commands
- **.cargo/config.toml**: WASM-specific build settings and server runner

## Next Steps

This scaffolding provides the foundation for AI demo projects. Implement your specific model and UI components in the appropriate workspace crates:

1. Define data types and model architecture in `shared/`
2. Implement training logic in `training/`  
3. Create interactive demo UI in `interactive/`
4. Deploy WASM build to your web application

For detailed implementation guidance, see `docs/projects/`.