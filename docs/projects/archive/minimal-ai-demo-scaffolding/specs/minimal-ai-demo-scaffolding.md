# Minimal AI Demo Scaffolding Project

## Overview

A complete AI demonstration template for Rust that provides scaffolding for machine learning projects. This project demonstrates the full workflow from model training to web deployment using Candle (ML framework) and Bevy (game engine/UI) compiled to WebAssembly.

### Purpose
Create a minimal but complete foundation that can be copied and extended for domain-specific AI projects, eliminating the need to set up basic infrastructure from scratch.

### Key Features
- **Training Pipeline**: Simple neural network training with Candle
- **Web Deployment**: Interactive demo compiled to WASM
- **Model Persistence**: Safetensors format for secure model storage
- **Complete Workflow**: From `cargo run --bin training` to working web demo
- **Modern Tooling**: GitHub Actions CI, proper testing, clean project structure

## Problem Statement

Starting a new AI project in Rust requires significant boilerplate:
- Workspace setup with multiple binaries
- ML framework integration (Candle)
- WASM compilation configuration
- Model serialization/loading patterns
- Web deployment pipeline
- Testing and CI setup

This template eliminates that friction by providing a working foundation that demonstrates all these integration points.

## Supporting Documentation

This specification is split across focused documents:

- **[Technical Requirements](technical-requirements.md)** - Detailed component specifications, dependencies, APIs
- **[Testing and CI](testing-and-ci.md)** - Testing strategy, GitHub Actions workflows, quality gates
- **[Implementation Guide](implementation-guide.md)** - Development approach, common pitfalls, future extensions

## Architecture Overview

### Components

**Training Binary (`training/`)**
- Simple 2→4→1 neural network using Candle
- Generates synthetic data, trains for 0 epochs (scaffolding demo)
- Saves model to `models/demo_model.safetensors`

**Interactive Demo (`interactive/`)**
- Bevy-based UI compiled to WebAssembly
- Loads saved model and provides number input interface
- Demonstrates complete web deployment pipeline

**Shared Library (`shared/`)**
- Common model definitions and data types
- Model persistence utilities (save/load safetensors)
- Data preprocessing functions

### Data Flow
```
Training: Random Data → MLP Creation → Save Model → models/demo_model.safetensors
                                                            ↓
Interactive: Load Model → User Input → Inference → Display Output
```

### Project Structure
```
project-root/
├── Cargo.toml                    # Workspace configuration
├── docs/projects/                # Project specifications and plans
├── training/                     # Native ML training binary
├── interactive/                  # Bevy WASM demo
├── shared/                       # Common library (models, types, persistence)
└── models/                       # Generated .safetensors files
```

## Success Criteria

**Functional Requirements:**
- Complete workflow from training to web demo works
- All components handle errors gracefully
- Clear user feedback throughout the process
- Reproducible builds across environments

**Quality Requirements:**
- Clean, well-documented code following Rust best practices
- All targets compile without warnings
- Responsive UI with fast model loading
- Effective scaffolding for future projects

See [Technical Requirements](technical-requirements.md) for detailed specifications and validation checklists.