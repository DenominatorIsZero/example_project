# AI Demo Template

A complete, reusable template for creating AI demonstration projects in Rust. This template provides everything needed to build machine learning demos with web deployment: model training, interactive UI, and WASM compilation pipeline.

## Template Features

- **Complete ML Workflow**: From training to web deployment
- **WASM-Ready**: Optimized for browser deployment with responsive design
- **Easy Customization**: Well-organized code with clear customization points
- **Website Integration**: Designed for iframe embedding with theme matching
- **Professional Structure**: Clean workspace organization and build system

## Quick Start

### Using This Template

1. **Copy the template** to your new project directory
2. **Customize for your use case** (see [Template Customization](#template-customization) below)
3. **Follow the development workflow** to build and deploy your demo

### Prerequisites

- Rust 1.70+ (MSRV)
- [just](https://github.com/casey/just) command runner: `cargo install just`

### Development Workflow

```bash
# One-time setup (installs WASM target and tools)
just setup

# Train your model
just train

# Test natively during development
just interactive

# Test WASM build locally
just wasm

# Build web package for deployment
just build-web

# Code quality checks
just check        # Format, lint, and test all code

# See all available commands
just --list
```

## Template Customization

This template is designed for easy adaptation to your specific use case. Here are the key customization points:

### UI and Visual Design

**Location**: `interactive/src/ui/constants.rs`

```rust
// Customize colors to match your website
pub const BACKGROUND_COLOR: Color = Color::srgb(0.216, 0.255, 0.318);
pub const TEXT_COLOR: Color = Color::WHITE;
pub const GREEN_PRIMARY: Color = Color::srgb(0.133, 0.698, 0.298);

// Adjust font sizes and spacing
pub const FONT_SIZE_TITLE: f32 = 28.0;
pub const FONT_SIZE_STANDARD: f32 = 16.0;
```

### Model Architecture

**Location**: `shared/src/model.rs`

- Modify `DemoMLP` structure for your neural network
- Update input/output sizes and hidden layers
- Customize forward pass and activation functions

### Training Data and Logic

**Location**: `training/src/main.rs`

- Replace synthetic data generation with your dataset
- Modify training loop, loss function, and optimization
- Adjust model saving format if needed

### UI Layout and Interaction

**Location**: `interactive/src/ui/` module

- Customize input fields and validation in `systems.rs`
- Modify UI layout and styling in `builders.rs` and `styles.rs`
- Update component structure in `components.rs`

## Web Integration

This template is optimized for embedding in web applications:

### Iframe Deployment

```html
<iframe src="/demos/index.html" width="800" height="600"></iframe>
```

**Key Features:**

- **Responsive Design**: Canvas automatically fits parent container
- **Theme Integration**: Default colors match the template creator's website (customizable)
- **Fast Loading**: Optimized WASM bundle with embedded assets

### Deployment Workflow

```bash
# Build optimized web package
just build-web

# Files generated in interactive/web/
# - index.html      (HTML wrapper)
# - interactive.js   (JavaScript bindings)
# - interactive_bg.wasm (Optimized WASM binary)
```

### Color Theme Matching

The template comes with colors that match the creator's website ([rust-website](https://github.com/DenominatorIsZero/rust-website)). **You'll likely want to customize these colors** in `interactive/src/ui/constants.rs` to match your own website's theme and branding.

## Template Project Structure

This template uses a three-crate workspace structure optimized for ML development:

```
├── Cargo.toml              # Workspace configuration and dependencies
├── justfile               # Development task automation (setup, build, test)
├── docs/                  # Architecture and project documentation
├── shared/                # Common library for model definitions and persistence
│   ├── src/model.rs       # 🎯 Neural network architecture (customize here)
│   └── src/persistence.rs # Model save/load functions (WASM-compatible)
├── training/              # Native binary for model training
│   └── src/main.rs        # 🎯 Training data and logic (customize here)
├── interactive/           # WASM binary for web demo
│   ├── src/ui/            # 🎯 User interface (customize here)
│   │   ├── constants.rs   #    Colors, fonts, layout constants
│   │   ├── builders.rs    #    UI component creation
│   │   └── systems.rs     #    Input handling and interaction
│   └── src/models/        # Model files for embedded assets (auto-generated)
├── models/                # Training output (.toml + .safetensors files)
└── .cargo/config.toml     # WASM build configuration
```

**Key Template Components:**

- **`shared/`**: Model architecture and data types shared between training and demo
- **`training/`**: ML training pipeline - replace with your data and model
- **`interactive/`**: Bevy-based UI compiled to WASM - customize appearance and interaction
- **`docs/`**: Complete architecture documentation for template users

### Model File Workflow

The training binary creates models in the workspace `models/` directory, then copies them to `interactive/src/models/` for use as embedded assets in the WASM build. This two-step process ensures:

- **Training output**: Workspace `models/` directory for easy access and backup
- **WASM embedded assets**: `interactive/src/models/` directory required by Bevy's `embedded_asset!` macro
- **Automatic workflow**: Running `just train` handles both saving and copying

## Technology Stack

- **Bevy 0.16**: Cross-platform UI framework with excellent WASM support
- **Candle**: Pure Rust ML framework for training and inference
- **WebAssembly**: Browser deployment via `wasm-bindgen`
- **SafeTensors**: Secure, efficient model storage format

## Troubleshooting

### WASM Compilation Issues

**Problem**: `wasm32-unknown-unknown` target not found

```bash
# Solution: Install WASM target
just setup
# or manually: rustup target add wasm32-unknown-unknown
```

**Problem**: Large WASM bundle size

- Use `just build-web` for optimized release builds (not debug builds)
- Bundle should be ~20MB for full Bevy + Candle application

### Model Loading Issues

**Problem**: "Model file not found" in WASM

- Ensure `just train` was run to generate and copy model files
- Check that files exist in `interactive/src/models/` (required for embedded assets)

**Problem**: Model inference errors

- Verify input shapes match model architecture
- Check that input values are in expected range (template uses [-1, 1])

### UI and Display Issues

**Problem**: Text not visible or UI elements misaligned

- Check color definitions in `interactive/src/ui/constants.rs`
- Ensure text colors have sufficient contrast with background
- Verify font sizes are appropriate for your content

**Problem**: Canvas not responsive in iframe

- Template uses `fit_canvas_to_parent: true` for responsive behavior
- Test iframe dimensions and ensure parent container has defined size

### Development Issues

**Problem**: Commands not found

- Install `just`: `cargo install just`
- Use `just --list` to see all available commands

**Problem**: Slow compilation

- Use `just build` for debug builds during development
- Reserve `just build-web` for final deployment builds

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

## Template Development Workflow

### Complete Development Cycle

```bash
# 1. Set up development environment
just setup

# 2. Customize the template for your use case
# Edit shared/src/model.rs for your neural network
# Edit training/src/main.rs for your data and training
# Edit interactive/src/ui/constants.rs for colors and styling

# 3. Train your model
just train

# 4. Test the demo during development
just interactive    # Native testing (faster iteration)
just wasm          # WASM testing (matches deployment)

# 5. Build for deployment
just build-web     # Creates optimized web package

# 6. Quality checks before deployment
just check         # Format, lint, and test all code
```

### Available Commands

Use `just --list` to see all commands. Key commands:

- **Setup**: `setup`, `clean`
- **Build**: `build`, `build-web`, `wasm`, `wasm-release`
- **Run**: `train`, `interactive`, `serve-web`
- **Quality**: `fmt`, `lint`, `test`, `check`

## Architecture Documentation

For complete technical details, see:

- **[docs/architecture.md](docs/architecture.md)**: Comprehensive technical architecture
- **[docs/projects/template/](docs/projects/template/)**: Generic templates for new projects
- **[docs/projects/archive/](docs/projects/archive/)**: Reference implementation example

This template provides a complete foundation for creating AI demonstrations with web deployment. Copy, customize, and build your own ML demos using this proven structure.
