# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with this AI demo project repository.

## Project Overview

This is an AI demonstration project designed to be integrated with the main website at https://github.com/DenominatorIsZero/rust-website. The project follows a standardized Cargo workspace structure with separate binaries for model training and interactive WASM demonstration.

## Repository Structure

```
demo-project-name/
├── Cargo.toml                 # Workspace root configuration
├── docs/                     # AI-assisted development documentation
│   ├── specs/                # Feature specifications and requirements
│   └── plans/                # Implementation plans with discrete todos
├── training/
│   ├── Cargo.toml            # Native training binary dependencies
│   ├── src/
│   │   └── main.rs           # Model training implementation
│   ├── data/                 # Training data and preprocessing scripts
│   └── README.md             # Training-specific documentation
├── interactive/
│   ├── Cargo.toml            # WASM demo binary dependencies
│   ├── src/
│   │   └── main.rs           # Bevy + Candle inference application
│   ├── assets/               # Bevy-specific assets (textures, sounds, etc.)
│   └── README.md             # Interactive demo documentation
├── shared/
│   ├── Cargo.toml            # Shared library crate
│   └── src/
│       ├── lib.rs            # Common exports
│       ├── model.rs          # Model architecture definitions
│       ├── preprocessing.rs  # Data preprocessing utilities
│       └── types.rs          # Shared data structures
├── models/                   # Generated .safetensors files
├── README.md                 # Project overview and usage
└── build.sh                 # Build script for both training and WASM
```

## AI-Assisted Development Workflow

This project is designed to be developed extensively with Claude Code assistance. The following workflow ensures systematic, well-documented feature development from initial concept to final implementation.

### Documentation Structure

The `docs/` folder contains two key subdirectories that support the AI-assisted development process:

- **`docs/specs/`** - Feature specifications and requirements documents
  - One file per feature (e.g., `user-authentication.md`, `model-optimization.md`)
  - Clear problem statements, success criteria, and acceptance criteria
  - Technical constraints and architectural considerations
  
- **`docs/plans/`** - Implementation plans with discrete todos
  - Corresponds to specifications in `docs/specs/`
  - Broken down into actionable, atomic tasks
  - Dependencies and sequencing clearly defined

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

### Commit Strategy

#### Commit as Natural Checkpoints

Commits should represent meaningful progress points, not arbitrary code changes:

- **Feature Milestones** - Complete implementation of planned tasks
- **Stable States** - Code compiles, tests pass, and functionality works
- **Logical Units** - Related changes grouped together (e.g., model definition + tests)
- **Rollback Points** - States you could confidently return to if needed

#### Commit Message Conventions

Follow this format for AI workflow commits:

```
[PHASE] Brief description of change

- Specific changes made
- Reference to docs/specs/ or docs/plans/ files
- Any deviations from original plan

Closes: #issue-number (if applicable)
Refs: docs/specs/feature-name.md, docs/plans/feature-name.md
```

Examples:
```
[SPEC] Define user authentication requirements

- Added comprehensive auth spec with OAuth2 integration
- Defined security requirements and session management
- Identified integration points with existing user system

Refs: docs/specs/user-authentication.md
```

```
[PLAN] Break down user authentication implementation

- Created 12 discrete tasks for auth implementation
- Identified dependencies on existing user model
- Planned database migration and API changes

Refs: docs/plans/user-authentication.md
```

```
[IMPL] Add OAuth2 login endpoint

- Implemented /auth/login route with Google OAuth2
- Added token validation and user session creation
- Updated user model with OAuth provider fields
- Added comprehensive error handling

Refs: docs/specs/user-authentication.md, docs/plans/user-authentication.md
```

#### Branching Strategy

- **Feature Branches** - One branch per specification for organized development (e.g., `feature/user-authentication`)
- **Direct Merges** - Merge to main after local validation and testing (solo project workflow)
- **Clean History** - Use meaningful commit messages with phase tags for clear development history
- **Rollback Safety** - Each commit represents a stable checkpoint you can confidently return to

## Development Commands

### Model Training

```bash
# Train the model (from repository root)
cargo run --bin training

# Train with specific parameters
cargo run --bin training -- --epochs 100 --batch-size 32

# Validate trained model
cargo run --bin training -- --validate-only
```

### Interactive Demo Development

```bash
# Install wasm-pack if not already installed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build WASM demo for development
cd interactive
wasm-pack build --target web --dev

# Build WASM demo for production
cd interactive
wasm-pack build --target web --release

# Serve locally for testing (requires basic HTTP server)
cd interactive/pkg && python -m http.server 8000
```

### Complete Build Process

```bash
# Build everything (training + WASM)
./build.sh

# Or manually:
cargo run --bin training                          # Generate models
cd interactive && wasm-pack build --target web    # Build WASM
```

## Technology Stack

### Training Binary (`training/`)

- **Framework**: Native Rust with Candle for model training
- **Data**: Custom dataset loading and preprocessing
- **Output**: .safetensors model files in `models/` directory
- **Dependencies**: candle-core, candle-nn, candle-optimizers

### Interactive Demo (`interactive/`)

- **Framework**: Bevy game engine compiled to WASM
- **AI Inference**: Candle for model loading and inference
- **Target**: WebAssembly with wasm-bindgen
- **Dependencies**: bevy, candle-core, wasm-bindgen, web-sys

### Shared Library (`shared/`)

- **Purpose**: Common model definitions and utilities
- **Usage**: Imported by both training and interactive binaries
- **Contents**: Model architecture, data types, preprocessing functions

## Integration with Main Website

### Asset Deployment

After building, copy generated files to the main website:

```bash
# Copy WASM files
cp interactive/pkg/* ../rust-website/static/wasm/project-name/

# Copy model files
cp models/* ../rust-website/static/models/project-name/
```

### URL Structure

- Demo accessible at: `https://erik-engelhardt.com/demos/project-name`
- WASM assets served from: `/static/wasm/project-name/`
- Model files served from: `/static/models/project-name/`

### Iframe Integration

The demo can be embedded in blog posts using:

```html
<iframe src="/demos/project-name" width="800" height="600"></iframe>
```

## Development Guidelines

### Code Organization

- Keep model architecture definitions in `shared/src/model.rs`
- Implement training logic in `training/src/main.rs`
- Implement Bevy app logic in `interactive/src/main.rs`
- Share preprocessing utilities through `shared/src/preprocessing.rs`

### Model Constraints

- Target model size: <50MB for reasonable download times
- Use .safetensors format for security and compatibility
- Implement progressive loading for larger models
- Provide meaningful loading states in the Bevy UI

### WASM Optimization

- Use `--release` builds for production deployment
- Enable `wee_alloc` for smaller binary sizes
- Minimize unnecessary Bevy features in Cargo.toml
- Consider `wasm-opt` for additional size optimization

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

- Ensure wasm-pack is up to date
- Check target compatibility (web vs bundler)
- Verify all dependencies support WASM compilation
- Review console errors for specific failure points

### Bevy Integration Issues

- Ensure compatible Bevy version across dependencies
- Check WASM feature flags in Cargo.toml
- Verify asset loading paths for WASM target
- Test UI scaling on different screen sizes

## Contributing Guidelines

### Before Making Changes

1. Understand the existing model architecture
2. Test training pipeline with small datasets
3. Verify WASM compilation succeeds
4. Check integration with main website

### Code Style

- Follow standard Rust formatting (rustfmt)
- Use meaningful variable and function names
- Document public APIs and complex algorithms
- Include examples in documentation comments

### Commit Guidelines

- Keep training and interactive changes separate when possible
- Include model performance metrics in commit messages
- Update README.md for significant changes
- Test both training and WASM builds before committing

This project structure enables complete reproducibility while maintaining clean separation between training and deployment concerns.
