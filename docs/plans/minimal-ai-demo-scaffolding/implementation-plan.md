# Implementation Plan: Minimal AI Demo Scaffolding

Implementation roadmap with discrete, actionable tasks for building the minimal AI demo scaffolding project.

## Plan Overview

This plan transforms the [specifications](../../specs/minimal-ai-demo-scaffolding/) into a systematic implementation approach with 8 phases, clear dependencies, and natural commit points.

### Implementation Strategy
- **Incremental Development**: Build and validate each component before moving to the next
- **Early Integration**: Test component interactions frequently to catch issues early
- **Natural Checkpoints**: Each task represents a stable, committable state
- **Dependency Management**: Later phases depend on earlier phases being complete

### Success Metrics
- Each phase produces working, testable components
- All tasks have clear "definition of done" 
- Commit points represent stable rollback positions
- Final result matches specification requirements

---

## Phase 1: Project Foundation
*Estimated effort: 1-2 hours*

**Goal**: Establish basic project structure and workspace configuration

### Tasks

#### 1.1 Create Cargo Workspace Structure
**Status**: ✅ Completed  
**Dependencies**: None  
**Definition of Done**: 
- Root `Cargo.toml` with workspace members defined
- Placeholder `Cargo.toml` files in each crate directory
- Directory structure matches specification
- `cargo check --workspace` runs without errors

**Implementation Steps**:
- [x] Create root `Cargo.toml` with workspace configuration
- [x] Create subdirectories: `shared/`, `training/`, `interactive/`, `models/`
- [x] Create placeholder `Cargo.toml` in each crate with basic metadata
- [x] Create placeholder `src/lib.rs` (shared) and `src/main.rs` (binaries)

**Commit Message**: `[IMPL] Set up Cargo workspace structure and placeholder crates`

#### 1.2 Add Core Dependencies
**Status**: ✅ Completed  
**Dependencies**: 1.1  
**Definition of Done**:
- All required dependencies added to appropriate crates
- Workspace-level dependency management configured
- `cargo build --workspace` compiles successfully
- No version conflicts or dependency issues

**Implementation Steps**:
- [x] Add shared dependencies in workspace `Cargo.toml`
- [x] Configure crate-specific dependencies for training binary
- [x] Configure crate-specific dependencies for interactive binary
- [x] Add WASM-specific features for interactive crate
- [x] Test compilation for all targets

**Dependencies to Add**:
```toml
# Workspace level
candle-core = "0.6"
candle-nn = "0.6"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }

# Training specific
safetensors = "0.4"

# Interactive specific  
bevy = "0.16"
wasm-bindgen = "0.2"
web-sys = "0.3"
getrandom = { version = "0.3", features = ["wasm_js"] }
```

**Commit Message**: `[IMPL] Add core dependencies and configure WASM features`

#### 1.3 Verify Development Environment
**Status**: ✅ Completed  
**Dependencies**: 1.2  
**Definition of Done**:
- Native compilation works for all crates
- WASM target is available and compiles
- Basic tooling (rustfmt, clippy) configured
- Development environment documented

**Implementation Steps**:
- [x] Test native builds: `cargo build --workspace`
- [x] Install WASM target: `rustup target add wasm32-unknown-unknown`
- [x] Install wasm-server-runner: `cargo install wasm-server-runner`
- [x] Create `.cargo/config.toml` with WASM runner configuration
- [x] Test WASM compilation: `cargo build --target wasm32-unknown-unknown`
- [x] Test WASM run: `cargo run --target wasm32-unknown-unknown --bin interactive`
- [x] Create justfile for development task automation
- [x] Document environment setup in README

**Commit Message**: `[IMPL] Verify and document development environment setup`

---

## Phase 2: Shared Library Implementation
*Estimated effort: 3-4 hours*

**Goal**: Implement core data types, model architecture, and persistence functions

### Tasks

#### 2.1 Implement Data Types and Structures
**Status**: ✅ Completed  
**Dependencies**: 1.3  
**Definition of Done**:
- All data structures from specification implemented
- Types are properly serializable where needed
- Basic validation functions work
- Unit tests pass for data type functionality

**Implementation Steps**:
- [x] Create `shared/src/types.rs` with core data structures
- [x] Implement `TrainingExample`, `PredictionInput`, `PredictionOutput`
- [x] Add `ModelMetadata` structure
- [x] Add serde derives where appropriate
- [x] Write basic validation functions
- [x] Add unit tests for data structures

**Key Types to Implement**:
```rust
pub struct TrainingExample {
    pub input: [f32; 2],    // Range [-1, 1]
    pub target: f32,        // Range [0, 1]
}

pub struct PredictionInput {
    pub values: [f32; 2],
}

pub struct PredictionOutput {
    pub value: f32,
}

pub struct ModelMetadata {
    pub input_size: usize,
    pub output_size: usize, 
    pub hidden_size: usize,
}
```

**Commit Message**: `[IMPL] Implement core data types and validation functions`

#### 2.2 Implement Model Architecture
**Status**: ✅ Completed  
**Dependencies**: 2.1  
**Definition of Done**:
- `DemoMLP` struct implemented with Candle layers
- Model creation and forward pass methods work
- Model can be instantiated and run inference
- Basic model tests pass

**Implementation Steps**:
- [x] Create `shared/src/model.rs` with MLP definition
- [x] Implement `DemoMLP::new()` with VarBuilder pattern
- [x] Implement `DemoMLP::forward()` with proper activations
- [x] Add basic error handling and validation
- [x] Write tests for model creation and forward pass
- [x] Test with dummy input data

**Model Implementation**:
```rust
pub struct DemoMLP {
    pub fc1: candle_nn::Linear,  // 2 → 4
    pub fc2: candle_nn::Linear,  // 4 → 1
}

impl DemoMLP {
    pub fn new(vb: VarBuilder) -> anyhow::Result<Self> {
        // Create linear layers with proper initialization
    }

    pub fn forward(&self, input: &Tensor) -> anyhow::Result<Tensor> {
        // input → fc1 → ReLU → fc2 → Sigmoid
    }
}
```

**Commit Message**: `[IMPL] Implement DemoMLP model architecture with Candle`

#### 2.3 Implement Model Persistence
**Status**: Pending  
**Dependencies**: 2.2  
**Definition of Done**:
- Model save/load functions work with safetensors format
- Round-trip save/load preserves model weights
- Error handling covers common failure cases
- Model verification function extracts correct metadata

**Implementation Steps**:
- [ ] Create `shared/src/persistence.rs` with I/O functions
- [ ] Implement `save_model()` with safetensors serialization
- [ ] Implement `load_model()` with proper error handling
- [ ] Implement `verify_model_file()` for metadata extraction
- [ ] Add comprehensive error handling and validation
- [ ] Write tests for save/load round-trips

**Key Functions**:
```rust
pub fn save_model(model: &DemoMLP, path: &str) -> anyhow::Result<()>;
pub fn load_model(path: &str, device: &Device) -> anyhow::Result<DemoMLP>;
pub fn verify_model_file(path: &str) -> anyhow::Result<ModelMetadata>;
```

**Commit Message**: `[IMPL] Implement model persistence with safetensors format`

#### 2.4 Create Shared Library Public API
**Status**: Pending  
**Dependencies**: 2.1, 2.2, 2.3  
**Definition of Done**:
- `shared/src/lib.rs` exports all public APIs cleanly
- Documentation is complete for all public functions
- API is easy to use from other crates
- Integration tests demonstrate API usage

**Implementation Steps**:
- [ ] Design clean public API in `lib.rs`
- [ ] Re-export key types and functions
- [ ] Add comprehensive documentation with examples
- [ ] Write integration tests showing typical usage patterns
- [ ] Test API from both training and interactive perspectives

**Commit Message**: `[IMPL] Complete shared library with clean public API`

---

## Phase 3: Training Binary Implementation
*Estimated effort: 2-3 hours*

**Goal**: Create functional training binary that generates and saves demo models

### Tasks

#### 3.1 Implement Synthetic Data Generation
**Status**: Pending  
**Dependencies**: 2.4  
**Definition of Done**:
- Generates random training data in specified ranges
- Data quality is suitable for demo purposes
- Generation is reproducible with seed option
- Basic data validation passes

**Implementation Steps**:
- [ ] Create data generation function in `training/src/main.rs`
- [ ] Generate inputs in range [-1, 1], targets in range [0, 1]
- [ ] Add option for reproducible random seed
- [ ] Implement basic data quality checks
- [ ] Add logging for data generation statistics

**Commit Message**: `[IMPL] Implement synthetic training data generation`

#### 3.2 Implement Model Training Pipeline  
**Status**: Pending  
**Dependencies**: 3.1  
**Definition of Done**:
- Model is created and initialized properly
- Training loop runs (even with 0 epochs)
- Loss calculation works correctly
- Training process has clear logging

**Implementation Steps**:
- [ ] Create model instance with proper device setup
- [ ] Implement basic training loop structure
- [ ] Add loss calculation (MSE for demonstration)
- [ ] Configure for 0 epochs as per specification
- [ ] Add progress logging and status messages
- [ ] Handle training errors gracefully

**Commit Message**: `[IMPL] Implement model training pipeline with 0-epoch demo`

#### 3.3 Implement Model Saving and Validation
**Status**: Pending  
**Dependencies**: 3.2  
**Definition of Done**:
- Trained model is saved to `models/demo_model.safetensors`
- Saved model can be reloaded and validated
- File size and format are reasonable
- Success/failure is clearly reported

**Implementation Steps**:
- [ ] Save model using shared library persistence functions
- [ ] Verify saved model by reloading and testing inference
- [ ] Add file size and location reporting
- [ ] Handle save errors with informative messages
- [ ] Clean up any temporary files or resources

**Commit Message**: `[IMPL] Complete training binary with model saving and validation`

---

## Phase 4: Interactive Demo Core
*Estimated effort: 4-5 hours*

**Goal**: Create basic Bevy application with model loading and UI structure

### Tasks

#### 4.1 Set Up Basic Bevy Application
**Status**: Pending  
**Dependencies**: 3.3  
**Definition of Done**:
- Bevy app initializes and runs without errors
- Basic plugin configuration is correct
- Window and rendering system work
- App can be cleanly shut down

**Implementation Steps**:
- [ ] Create basic Bevy app structure in `interactive/src/main.rs`
- [ ] Configure essential plugins (UI, text, asset loading)
- [ ] Set up basic window and rendering
- [ ] Test native compilation and execution
- [ ] Add basic error handling and logging

**Commit Message**: `[IMPL] Set up basic Bevy application structure`

#### 4.2 Implement Model Loading System
**Status**: Pending  
**Dependencies**: 4.1  
**Definition of Done**:
- Model loads from file on application startup
- Loading success/failure is tracked in app state
- Error handling provides useful feedback
- Model is stored as Bevy resource for other systems

**Implementation Steps**:
- [ ] Create `LoadedModel` resource structure
- [ ] Implement model loading system that runs on startup
- [ ] Add error handling for missing or invalid model files
- [ ] Store loading status for UI display
- [ ] Test with both valid and invalid model files

**Commit Message**: `[IMPL] Implement model loading system with error handling`

#### 4.3 Create Basic UI Layout
**Status**: Pending  
**Dependencies**: 4.2  
**Definition of Done**:
- UI layout matches specification design
- All UI elements are visible and properly positioned
- Text displays are working correctly
- UI scales reasonably on different screen sizes

**Implementation Steps**:
- [ ] Design UI layout system with Bevy UI components
- [ ] Create status display for model loading state
- [ ] Create input field placeholders (text display for now)
- [ ] Create predict button placeholder
- [ ] Create output display area
- [ ] Test UI layout and basic interactivity

**UI Elements to Create**:
- Model status indicator
- Two input fields for numbers
- Predict button
- Output display area
- Title/header text

**Commit Message**: `[IMPL] Create basic UI layout with status displays`

---

## Phase 5: Interactive Demo Features
*Estimated effort: 3-4 hours*

**Goal**: Implement user input handling, inference processing, and output display

### Tasks

#### 5.1 Implement Input Handling System
**Status**: Pending  
**Dependencies**: 4.3  
**Definition of Done**:
- Users can input numbers in both input fields
- Input validation works (range checking, number parsing)
- Input state is properly managed in Bevy ECS
- Clear feedback for invalid inputs

**Implementation Steps**:
- [ ] Create input field components with editable text
- [ ] Implement text input handling (keyboard events)
- [ ] Add number parsing and validation
- [ ] Create visual feedback for valid/invalid inputs
- [ ] Test input handling edge cases (empty, non-numeric, out of range)

**Commit Message**: `[IMPL] Implement user input handling with validation`

#### 5.2 Implement Prediction System
**Status**: Pending  
**Dependencies**: 5.1  
**Definition of Done**:
- Predict button triggers inference when clicked
- Inference runs using loaded model
- Results are calculated correctly
- System handles inference errors gracefully

**Implementation Steps**:
- [ ] Create prediction event system for button clicks
- [ ] Implement inference processing system
- [ ] Add tensor creation from user inputs
- [ ] Run model inference and extract results
- [ ] Handle inference errors with user feedback
- [ ] Test prediction accuracy and error cases

**Event Flow**:
1. User clicks predict button
- [ ] System validates inputs
- [ ] Creates tensor from input values
- [ ] Runs model.forward()
- [ ] Extracts and formats output
- [ ] Updates UI display

**Commit Message**: `[IMPL] Implement prediction system with model inference`

#### 5.3 Implement Output Display and UI Updates
**Status**: Pending  
**Dependencies**: 5.2  
**Definition of Done**:
- Prediction results are displayed clearly
- UI updates responsively to user actions
- Status messages provide helpful feedback
- All UI interactions feel smooth and intuitive

**Implementation Steps**:
- [ ] Create output display update system
- [ ] Implement status message system
- [ ] Add loading states for long operations
- [ ] Polish UI responsiveness and feedback
- [ ] Test complete user interaction flow

**Commit Message**: `[IMPL] Complete interactive demo with output display and UI polish`

---

## Phase 6: WASM Compilation and Web Deployment
*Estimated effort: 2-3 hours*

**Goal**: Successfully compile to WASM and create web-deployable package

### Tasks

#### 6.1 Configure WASM Build System
**Status**: Pending  
**Dependencies**: 5.3  
**Definition of Done**:
- WASM compilation succeeds without errors
- Generated WASM files are reasonable size
- All required web assets are generated
- Build process is documented and repeatable

**Implementation Steps**:
- [ ] Configure Cargo.toml for WASM optimization
- [ ] Test wasm-pack build process
- [ ] Optimize for size and performance
- [ ] Document build requirements and process
- [ ] Test WASM output quality and size

**Build Commands**:
```bash
cd interactive
wasm-pack build --target web --release
```

**Commit Message**: `[IMPL] Configure WASM build system with optimization`

#### 6.2 Create Web Package and HTML Wrapper
**Status**: Pending  
**Dependencies**: 6.1  
**Definition of Done**:
- HTML file properly loads and initializes WASM
- Demo runs correctly in web browser
- Styling is appropriate for web deployment
- Loading states and error handling work in browser

**Implementation Steps**:
- [ ] Create HTML wrapper file for the demo
- [ ] Add basic CSS styling for professional appearance
- [ ] Implement WASM loading and initialization
- [ ] Add error handling for WASM loading failures
- [ ] Test in multiple browsers (Chrome, Firefox, Safari)

**HTML Structure**:
```html
<!DOCTYPE html>
<html>
<head>
    <title>AI Demo Scaffold</title>
    <style>/* Basic styling */</style>
</head>
<body>
    <script type="module">
        import init from './interactive.js';
        init();
    </script>
</body>
</html>
```

**Commit Message**: `[IMPL] Create web package with HTML wrapper and styling`

#### 6.3 Test and Validate Web Deployment
**Status**: Pending  
**Dependencies**: 6.2  
**Definition of Done**:
- Demo works correctly when served over HTTP
- All functionality works in web environment
- Performance is acceptable for demo purposes
- No console errors or warnings

**Implementation Steps**:
- [ ] Set up local HTTP server for testing
- [ ] Test complete user interaction flow in browser
- [ ] Verify model loading and inference work correctly
- [ ] Check for console errors and performance issues
- [ ] Test on different devices and screen sizes

**Testing Commands**:
```bash
cd interactive/pkg
python -m http.server 8000
# Test at http://localhost:8000
```

**Commit Message**: `[IMPL] Complete WASM deployment with full browser testing`

---

## Phase 7: Testing and Validation
*Estimated effort: 3-4 hours*

**Goal**: Add comprehensive test coverage for critical functionality

### Tasks

#### 7.1 Add Unit Tests for Shared Library
**Status**: Pending  
**Dependencies**: 2.4  
**Definition of Done**:
- All shared library functions have unit tests
- Tests cover both success and failure cases
- Test coverage includes data validation and model operations
- All tests pass consistently

**Implementation Steps**:
- [ ] Add tests for data type validation functions
- [ ] Add tests for model creation and forward pass
- [ ] Add tests for model save/load round trips
- [ ] Add tests for error conditions and edge cases
- [ ] Ensure tests are isolated and repeatable

**Test Categories**:
- Data type creation and validation
- Model initialization and inference
- Model persistence (save/load/verify)
- Error handling and edge cases

**Commit Message**: `[IMPL] Add comprehensive unit tests for shared library`

#### 7.2 Add Integration Tests for Training Pipeline
**Status**: Pending  
**Dependencies**: 3.3, 7.1  
**Definition of Done**:
- Training binary can be tested end-to-end
- Model generation and saving is verified
- Integration tests run in isolated environment
- Tests clean up temporary files properly

**Implementation Steps**:
- [ ] Create integration test for complete training workflow
- [ ] Test model file creation and validation
- [ ] Add tests for command-line interface
- [ ] Test error handling (missing directories, permissions)
- [ ] Ensure proper cleanup of test artifacts

**Commit Message**: `[IMPL] Add integration tests for training pipeline`

#### 7.3 Add Component Tests for Interactive Demo
**Status**: Pending  
**Dependencies**: 5.3, 7.2  
**Definition of Done**:
- Key interactive demo components are tested
- Model loading and inference are verified
- UI components can be tested in isolation
- Tests work for both native and WASM builds

**Implementation Steps**:
- [ ] Add tests for model loading functionality
- [ ] Add tests for inference processing
- [ ] Test input validation and parsing
- [ ] Add basic UI component tests where possible
- [ ] Test error handling in interactive context

**Commit Message**: `[IMPL] Add component tests for interactive demo functionality`

#### 7.4 Manual Testing and Documentation
**Status**: Pending  
**Dependencies**: 7.1, 7.2, 7.3  
**Definition of Done**:
- Complete manual testing checklist executed
- All success criteria from specification verified
- Testing documentation is complete
- Known issues and limitations documented

**Manual Testing Checklist**:
- [ ] Training binary completes successfully
- [ ] Model file created with reasonable size
- [ ] Interactive demo launches without errors
- [ ] WASM build and web deployment work
- [ ] User can input numbers and get predictions
- [ ] Error cases handled gracefully
- [ ] Cross-platform compatibility verified

**Commit Message**: `[IMPL] Complete manual testing and documentation`

---

## Phase 8: CI/CD Setup and Final Integration
*Estimated effort: 2-3 hours*

**Goal**: Set up automated testing and build workflows

### Tasks

#### 8.1 Set Up GitHub Actions Test Workflow
**Status**: Pending  
**Dependencies**: 7.4  
**Definition of Done**:
- Test workflow runs on every push and PR
- All workspace tests execute successfully
- Code quality checks (fmt, clippy) pass
- Workflow is reliable and provides clear feedback

**Implementation Steps**:
- [ ] Create `.github/workflows/test.yml`  
- [ ] Configure Rust toolchain and caching
- [ ] Add workspace testing and quality checks
- [ ] Test workflow on different platforms if needed
- [ ] Ensure workflow provides clear success/failure feedback

**Commit Message**: `[IMPL] Set up GitHub Actions test workflow`

#### 8.2 Set Up Build Workflows for Releases
**Status**: Pending  
**Dependencies**: 8.1  
**Definition of Done**:
- Native build workflow creates distributable binaries
- WASM build workflow creates deployable web package
- Workflows are manually triggered and work reliably
- Artifacts are properly packaged and downloadable

**Implementation Steps**:
- [ ] Create `.github/workflows/native-build.yml`
- [ ] Create `.github/workflows/wasm-build.yml`  
- [ ] Configure multi-platform builds for native binaries
- [ ] Set up artifact uploading and packaging
- [ ] Test workflows and verify artifact quality

**Commit Message**: `[IMPL] Set up build workflows for native and WASM releases`

#### 8.3 Final Integration and Documentation
**Status**: Pending  
**Dependencies**: 8.1, 8.2  
**Definition of Done**:
- All components work together seamlessly
- Documentation is complete and accurate
- Project serves as effective scaffolding template
- Success criteria from specification are met

**Implementation Steps**:
- [ ] Run complete end-to-end testing workflow
- [ ] Update documentation with final setup instructions
- [ ] Verify project can be easily copied and extended
- [ ] Add any missing documentation or examples
- [ ] Perform final quality review

**Final Validation Checklist**:
- [ ] `cargo run --bin training` works end-to-end
- [ ] `cargo run --bin interactive` works natively  
- [ ] WASM build and web deployment work
- [ ] All tests pass: `cargo test --workspace`
- [ ] All quality checks pass: `cargo fmt --check`, `cargo clippy`
- [ ] GitHub Actions workflows are functional
- [ ] Project is ready to serve as scaffolding template

**Commit Message**: `[IMPL] Complete final integration and documentation`

---

## Implementation Notes

### Dependency Management
- Phases must be completed in order due to dependencies
- Some tasks within phases can be parallelized
- Each task should be committed individually for clean history
- Failed tasks should be debugged before proceeding

### Quality Gates
- Each phase should be fully tested before moving to next
- All commits should leave the project in a buildable state
- Regular integration testing prevents accumulation of issues
- Documentation should be updated as implementation progresses

### Adaptation Guidelines
- Tasks may need adjustment based on implementation discoveries
- New tasks can be added if unforeseen requirements emerge
- Time estimates are rough guidelines, not rigid constraints
- Plan should be updated to reflect actual implementation experience

### Success Metrics
- Working end-to-end demo that matches specification
- Clean, well-documented code that serves as good scaffolding
- Comprehensive testing and CI/CD setup
- Project ready for others to copy and extend

This plan provides a systematic approach to building the minimal AI demo scaffolding while maintaining flexibility for adaptation during implementation.