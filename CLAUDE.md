# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with this AI Demo Template repository.

## Project Overview

This is an AI Demo Template designed to be a reusable foundation for creating machine learning demonstration projects. The template provides a complete workflow from model training to web deployment, following a standardized Cargo workspace structure with separate binaries for model training and interactive WASM demonstration.

**Repository**: https://github.com/DenominatorIsZero/example_project

## Repository Structure

```
ai-demo-template/
├── Cargo.toml                # Workspace root configuration with metadata
├── LICENSE                   # MIT license for template usage
├── CONTRIBUTING.md           # Template contribution guidelines
├── README.md                 # Template usage guide and customization
├── justfile                  # Development commands and build automation
├── .gitignore               # Comprehensive development environment coverage
├── docs/                    # AI-assisted development documentation
│   ├── architecture.md      # Technical architecture and design decisions
│   └── projects/            # Project-based documentation organization
│       ├── archive/         # Completed project specifications and plans
│       │   └── minimal-ai-demo-scaffolding/
│       ├── template/        # Reusable specification and plan templates
│       └── polishing/       # Template polishing implementation plan
├── training/
│   ├── Cargo.toml           # Native training binary with metadata
│   └── src/
│       └── main.rs          # Model training implementation
├── interactive/
│   ├── Cargo.toml           # WASM demo binary with metadata
│   ├── src/
│   │   ├── main.rs          # Bevy application entry point
│   │   ├── lib.rs           # Library exports for WASM
│   │   ├── app.rs           # Application setup and configuration
│   │   ├── model/           # Model loading and inference systems
│   │   │   ├── mod.rs
│   │   │   ├── assets.rs
│   │   │   ├── loader.rs
│   │   │   ├── resources.rs
│   │   │   └── systems.rs
│   │   ├── models/          # Embedded model files
│   │   │   ├── demo_model.toml
│   │   │   └── demo_model.safetensors
│   │   └── ui/              # Modular UI system
│   │       ├── mod.rs
│   │       ├── components.rs    # UI component definitions
│   │       ├── constants.rs     # Customizable design constants
│   │       ├── builders.rs      # UI construction functions
│   │       ├── styles.rs        # Styling and layout functions
│   │       └── systems.rs       # UI behavior and event handling
│   ├── tests/               # Interactive demo test suite
│   └── web/                 # Generated web deployment package
├── shared/
│   ├── Cargo.toml           # Shared library with metadata
│   ├── src/
│   │   ├── lib.rs           # Common exports
│   │   ├── model.rs         # Model architecture definitions
│   │   └── persistence.rs   # Model loading and saving utilities
│   └── tests/
│       └── integration_tests.rs
└── models/                  # Generated model files from training
    ├── demo_model.toml
    └── demo_model.safetensors
```

## AI-Assisted Development Workflow

This project is designed to be developed extensively with Claude Code assistance. The following workflow ensures systematic, well-documented feature development from initial concept to final implementation.

### Documentation Structure

The `docs/` folder contains project organization that supports the AI-assisted development process:

- **`docs/projects/`** - Project-based documentation organization
  - **`archive/`** - Completed projects with their specifications and implementation plans
    - Contains the original minimal-ai-demo-scaffolding project as reference
  - **`template/`** - Reusable specification and plan templates for new projects
    - `spec-template.md` - Generic project specification template
    - `plan-template.md` - Phase-based implementation plan template
  - **`polishing/`** - Template polishing project with comprehensive implementation plan
- **`docs/architecture.md`** - Technical architecture documentation covering:
  - Workspace design rationale and three-crate structure
  - WASM integration strategy and responsive design
  - Model persistence architecture with embedded assets
  - UI architecture using Bevy ECS patterns
  - Build system and development workflow

### Three-Phase Development Process

#### Phase 1: Specification
Work with Claude Code to develop comprehensive feature specifications:

1. **Problem Definition** - Clearly articulate what needs to be built and why
2. **Requirements Gathering** - Define functional and non-functional requirements
3. **Success Criteria** - Establish measurable outcomes and acceptance criteria
4. **Technical Constraints** - Document limitations, dependencies, and architectural considerations
5. **Iterative Refinement** - Use Claude Code to refine specs until everything is crystal clear

The specification phase is complete when:
- All stakeholders would understand what's being built
- Technical approach is well-defined
- Edge cases and error scenarios are covered
- Success criteria are measurable and testable

#### Phase 2: Planning
Transform specifications into actionable implementation plans:

1. **Task Breakdown** - Decompose features into discrete, atomic todos
2. **Dependency Mapping** - Identify task dependencies and optimal sequencing
3. **Commit Strategy** - Plan natural checkpoints for incremental commits
4. **Risk Assessment** - Identify potential blockers and mitigation strategies
5. **Resource Estimation** - Estimate effort and identify required expertise

Plans should contain:
- Numbered, sequential tasks that can be completed independently
- Clear definition of "done" for each task
- Commit points that represent stable, testable states
- Rollback strategies for high-risk changes

#### Phase 3: Execution
Implement plans step-by-step with Claude Code assistance:

1. **Task-by-Task Implementation** - Work through planned todos systematically
2. **Frequent Commits** - Commit at natural checkpoints for incremental progress
3. **Continuous Validation** - Test and validate changes at each checkpoint
4. **Documentation Updates** - Keep documentation in sync with implementation
5. **Plan Adaptation** - Adjust plans based on discoveries during implementation

### AI Integration Guidelines

#### Effective Collaboration with Claude Code

- **Context Management** - Reference specifications and plans to maintain context across sessions
- **Incremental Development** - Work on one discrete task at a time
- **Code Review Partnership** - Use Claude Code to review implementations before committing
- **Problem-Solving** - Leverage Claude Code's analysis for debugging and optimization

#### Best Practices for AI-Assisted Development

- **Clear Communication** - Provide specific, actionable requests
- **Iterative Refinement** - Use multiple rounds of feedback to improve specifications and implementations
- **Documentation First** - Always document decisions and rationale for future reference
- **Validation Focus** - Ask Claude Code to help validate implementations against specifications

#### CLAUDE.md Maintenance

**IMPORTANT**: Keep this file current to improve future AI interactions.

**When to update CLAUDE.md:**
- After major architectural changes or refactoring
- When new development patterns or workflows are established
- After discovering common issues and their solutions
- When user has to correct AI assumptions or provide missing context
- After adding new tools, dependencies, or build processes

**Proactive Update Protocol:**
- Claude Code should offer to update CLAUDE.md after significant changes
- Focus on generic guidance that benefits future interactions
- Document new patterns, workflows, or common corrections
- Update file paths, command references, or architectural descriptions
- Add lessons learned from debugging or problem-solving sessions

Example trigger situations:
- "You should use X instead of Y" → Add to development guidelines
- "The files are actually located in Z" → Update repository structure
- "This command doesn't work, use this instead" → Update development commands
- Major refactoring or new features → Update relevant sections

### Commit Strategy

#### Commit as Natural Checkpoints

Commits should represent meaningful progress points, not arbitrary code changes:

- **Feature Milestones** - Complete implementation of planned tasks
- **Stable States** - Code compiles, tests pass, and functionality works
- **Logical Units** - Related changes grouped together (e.g., model definition + tests)
- **Rollback Points** - States you could confidently return to if needed

#### Commit Review Requirement

**IMPORTANT**: Always ask the user to review changes before committing. Present a summary of what will be committed and wait for approval before executing any git commit commands.

#### Commit Message Conventions

Follow this format for AI workflow commits:

```
[PHASE] Brief description of change

- Specific changes made
- Reference to docs/projects/ files
- Any deviations from original plan

Closes: #issue-number (if applicable)
Refs: docs/projects/feature-name/specs/feature-name.md, docs/projects/feature-name/plan.md
```

**IMPORTANT**: Do NOT include "Generated with Claude Code" footers or "Co-Authored-By: Claude" lines in commit messages. Use only the clean format shown above.

Examples:
```
[SPEC] Define user authentication requirements

- Added comprehensive auth spec with OAuth2 integration
- Defined security requirements and session management
- Identified integration points with existing user system

Refs: docs/projects/user-authentication/specs/user-authentication.md
```

```
[PLAN] Break down user authentication implementation

- Created 12 discrete tasks for auth implementation
- Identified dependencies on existing user model
- Planned database migration and API changes

Refs: docs/projects/user-authentication/plan.md
```

```
[IMPL] Add OAuth2 login endpoint

- Implemented /auth/login route with Google OAuth2
- Added token validation and user session creation
- Updated user model with OAuth provider fields
- Added comprehensive error handling

Refs: docs/projects/user-authentication/specs/user-authentication.md, docs/projects/user-authentication/plan.md
```

#### Branching Strategy

- **Feature Branches** - One branch per specification for organized development (e.g., `feature/user-authentication`)
- **Direct Merges** - Merge to main after local validation and testing (solo project workflow)
- **Clean History** - Use meaningful commit messages with phase tags for clear development history
- **Rollback Safety** - Each commit represents a stable checkpoint you can confidently return to

## Development Commands

The template uses a `justfile` for streamlined development workflow. All commands should be run from the repository root.

### Setup and Prerequisites

```bash
# One-time setup: Install WASM target and development tools
just setup

# Install just command runner if not already installed
cargo install just

# See all available commands
just --list
```

### Model Training

```bash
# Train the model (generates files in models/ directory)
just train

# Direct cargo command (if needed for debugging)
cargo run --bin training
```

### Interactive Demo Development

```bash
# Test natively during development (fastest iteration)
just interactive

# Test WASM build locally with development server
just wasm

# Test WASM release build locally
just wasm-release
```

### Build Commands

```bash
# Build all workspace components (debug)
just build

# Build all workspace components (release)
just build-release

# Build complete web package for deployment
just build-web

# Serve built web package locally for testing
just serve-web
```

### Code Quality

```bash
# Format all code
just fmt

# Run clippy linting
just lint

# Run test suite
just test

# Run all quality checks (format + lint + test)
just check
```

## Technology Stack

### Training Binary (`training/`)

- **Framework**: Native Rust with Candle 0.9 for model training
- **Data**: Synthetic dataset generation (easily replaceable)
- **Output**: .safetensors and .toml model files in `models/` directory
- **Dependencies**: candle-core, candle-nn, candle-optimizers, rand 0.9.2, shared crate

### Interactive Demo (`interactive/`)

- **Framework**: Bevy 0.16 game engine compiled to WASM
- **AI Inference**: Candle 0.9 for model loading and inference
- **UI Components**: bevy-simple-text-input for interactive elements
- **Target**: WebAssembly with wasm-bindgen integration
- **Model Loading**: Embedded assets using Bevy's asset system
- **Dependencies**: bevy, candle-core, candle-nn, bevy-simple-text-input, shared crate

### Shared Library (`shared/`)

- **Purpose**: Common model definitions and persistence utilities
- **Usage**: Imported by both training and interactive binaries
- **Contents**: Model architecture (`DemoMLP`), model persistence (memory and file loading)
- **Dependencies**: candle-core, candle-nn, safetensors, serde, toml

### Build System

- **Command Runner**: just (Justfile-based workflow)
- **WASM Compilation**: wasm-bindgen with web target
- **Package Management**: Cargo workspace with shared dependencies
- **Rust Edition**: 2024 edition with workspace metadata

## Template Usage and Integration

### Template Adaptation Workflow

When using this template for a new AI demo project:

1. **Copy the template**: Fork or clone this repository
2. **Customize the model**: Modify `shared/src/model.rs` for your neural network architecture
3. **Update training data**: Replace synthetic data in `training/src/main.rs` with your dataset
4. **Customize UI**: Modify `interactive/src/ui/constants.rs` for colors, fonts, and layout
5. **Update metadata**: Change project names, descriptions, and repository URLs in `Cargo.toml` files

### Web Integration

The template is designed for web deployment with responsive iframe embedding:

```html
<!-- Responsive iframe for blog posts -->
<iframe src="/demos/your-project-name" width="800" height="600"></iframe>
```

### Asset Deployment

After building with `just build-web`, deploy the generated files:

```bash
# Generated web package is in interactive/web/
# Contains: interactive.wasm, interactive.js, index.html

# Copy to your web server
cp interactive/web/* /path/to/your/website/demos/project-name/
```

### Template Features for Integration

- **Responsive Design**: Automatically fits parent container
- **Self-Contained**: All assets embedded in WASM binary
- **Theme Matching**: Customizable colors in `ui/constants.rs`
- **Professional Metadata**: Complete package information and licensing

## Development Guidelines

### Code Organization

- **Model Architecture**: Define models in `shared/src/model.rs`
- **Model Persistence**: Loading/saving utilities in `shared/src/persistence.rs`
- **Training Logic**: Implement in `training/src/main.rs`
- **Application Setup**: Main Bevy app configuration in `interactive/src/main.rs`
- **UI Constants**: Customizable design values in `interactive/src/ui/constants.rs`
- **UI Components**: Component definitions in `interactive/src/ui/components.rs`
- **UI Systems**: Behavior and event handling in `interactive/src/ui/systems.rs`
- **Model Systems**: Inference and loading in `interactive/src/model/systems.rs`

### Template Customization Points

Key files to modify when adapting the template:

- **`interactive/src/ui/constants.rs`** 🎯 - Colors, fonts, spacing, layout dimensions
- **`shared/src/model.rs`** 🎯 - Neural network architecture and forward pass
- **`training/src/main.rs`** 🎯 - Training data, loss function, hyperparameters
- **`interactive/src/ui/builders.rs`** - UI layout and component structure
- **`interactive/src/ui/systems.rs`** - Input validation and prediction logic
- **`Cargo.toml` files** - Project metadata, descriptions, repository URLs

### Model Constraints

- Target model size: <50MB for reasonable download times
- Use .safetensors format for security and compatibility
- Implement progressive loading for larger models
- Provide meaningful loading states in the Bevy UI

### WASM Optimization

- Use `just build-web` for production deployment (includes release optimizations)
- Size optimization profile enabled: `opt-level = 'z'`, `lto = true`, `strip = "symbols"`
- Minimize unnecessary Bevy features in Cargo.toml
- Consider `wasm-opt` for additional size optimization
- Embedded assets eliminate network requests for model files

### Error Handling

- Graceful degradation for model loading failures
- Clear error messages for unsupported browsers
- Fallback UI states when inference fails
- Loading indicators for network requests

## Testing Strategy

### Training Validation

- Verify model convergence and performance metrics
- Test model serialization/deserialization
- Validate against known test datasets
- Check model file sizes and formats

### Interactive Demo Testing

- Test WASM compilation and loading
- Verify inference accuracy matches training results
- Test UI responsiveness and error states
- Cross-browser compatibility testing
- Mobile device performance validation

## Performance Considerations

### Training Performance

- Use appropriate batch sizes for available memory
- Consider GPU acceleration where available
- Profile training loops for bottlenecks
- Implement checkpointing for long training runs

### WASM Performance

- Profile inference performance in browser
- Optimize model size vs accuracy tradeoffs
- Implement efficient data structures for browser constraints
- Consider Web Workers for heavy computations

## Common Issues and Solutions

### Model Loading Problems

- Ensure .safetensors files are properly generated
- Check file paths and CORS headers
- Verify model architecture matches between training and inference
- Test with smaller models first

### WASM Build Issues

- Ensure wasm-server-runner and wasm-bindgen-cli are installed (`just setup`)
- Use `just wasm` for development testing instead of manual compilation
- Verify all dependencies support WASM compilation (check Cargo.toml features)
- Review browser console errors for specific failure points
- Long compile times (5+ minutes) are normal for Bevy + Candle WASM builds

### Bevy Integration Issues

- Ensure compatible Bevy version across dependencies
- Check WASM feature flags in Cargo.toml
- Verify asset loading paths for WASM target
- Test UI scaling on different screen sizes

## Template Files

### Documentation and Licensing

- **`LICENSE`**: MIT license enabling free template usage and modification
- **`CONTRIBUTING.md`**: Guidelines for contributing to the template (not using it)
- **`README.md`**: Comprehensive template usage guide with customization instructions
- **`docs/architecture.md`**: Technical architecture and design decisions
- **`docs/projects/template/`**: Reusable specification and plan templates for new projects

### Development Tools

- **`justfile`**: Streamlined development commands replacing complex shell scripts
- **`.gitignore`**: Comprehensive coverage for Rust, IDE, OS, and development artifacts

## Contributing Guidelines

This is a **template project** designed for reuse. See `CONTRIBUTING.md` for detailed contribution guidelines.

### Quick Contributing Guide

1. **Template improvements**: Focus on changes that benefit all template users
2. **Test thoroughly**: Run `just check` and verify WASM builds work
3. **Maintain simplicity**: Avoid over-engineering or unnecessary complexity
4. **Update documentation**: Keep README and architecture docs current

### Code Style

- **Formatting**: Use `just fmt` (standard rustfmt)
- **Quality**: Use `just lint` (clippy with warnings as errors)
- **Testing**: Use `just test` (comprehensive test coverage)
- **Architecture**: Maintain clean separation between training/interactive/shared crates

This template structure enables complete reproducibility while maintaining clean separation between training and deployment concerns, designed specifically as a foundation for creating new AI demonstration projects.
