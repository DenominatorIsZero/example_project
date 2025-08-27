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

_Estimated effort: 1-2 hours_

**Goal**: Establish basic project structure and workspace configuration

### Tasks

#### 1.1 Create Cargo Workspace Structure

**Status**: [x] Completed  
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

**Status**: [x] Completed  
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

**Status**: [x] Completed  
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

_Estimated effort: 3-4 hours_

**Goal**: Implement core data types, model architecture, and persistence functions

### Tasks

#### 2.1 Implement Data Types and Structures

**Status**: [x] Completed (Modified Approach)  
**Dependencies**: 1.3  
**Definition of Done**:

- Core data structures implemented (ModelMetadata in model.rs)
- Types are properly serializable where needed
- Basic validation functions work
- Unit tests pass for data type functionality

**Implementation Steps**:

- [x] ~~Create `shared/src/types.rs` with core data structures~~ (Not needed)
- [x] ~~Implement `TrainingExample`, `PredictionInput`, `PredictionOutput`~~ (Simple types used instead)
- [x] Add `ModelMetadata` structure in model.rs with full validation
- [x] Add serde derives for TOML serialization
- [x] Write comprehensive validation functions with bounds checking
- [x] Add extensive unit tests for data structures

**Actual Implementation**:

```rust
// ModelMetadata embedded in model.rs (not separate types.rs)
pub struct ModelMetadata {
    pub input_size: usize,    // Validated: > 0, < 10,000
    pub output_size: usize,   // Validated: > 0, < 10,000
    pub hidden_size: usize,   // Validated: > 0, < 10,000
}

// No formal TrainingExample/PredictionInput/Output structs
// Simple types used: Vec<(Vec<f32>, f32)> for training data
// Direct Tensor usage for model input/output
```

**Commit Message**: `[IMPL] Implement core data types and validation functions`

#### 2.2 Implement Model Architecture

**Status**: [x] Completed (Enhanced)  
**Dependencies**: 2.1  
**Definition of Done**:

- `DemoMLP` struct implemented with flexible, metadata-driven architecture
- Model creation and forward pass methods work
- Model can be instantiated with any architecture and run inference
- Comprehensive model tests pass including validation

**Implementation Steps**:

- [x] Create `shared/src/model.rs` with flexible MLP definition
- [x] Implement `DemoMLP::new(metadata, vb)` with VarBuilder pattern
- [x] Add `DemoMLP::new_demo()` convenience constructor for 2→4→1 default
- [x] Implement `DemoMLP::forward()` with ReLU→Sigmoid activations
- [x] Add comprehensive error handling and input validation
- [x] Write extensive tests for multiple architectures and edge cases
- [x] Test with various input shapes and validate outputs

**Actual Implementation**:

```rust
pub struct DemoMLP {
    pub fc1: candle_nn::Linear,    // input_size → hidden_size
    pub fc2: candle_nn::Linear,    // hidden_size → output_size
    pub metadata: ModelMetadata,   // Architecture specification
}

impl DemoMLP {
    // Flexible constructor with any architecture
    pub fn new(metadata: ModelMetadata, vb: VarBuilder) -> anyhow::Result<Self>;

    // Convenience constructor for default 2→4→1 demo
    pub fn new_demo(vb: VarBuilder) -> anyhow::Result<Self>;

    // Validated forward pass: input → fc1 → ReLU → fc2 → Sigmoid
    pub fn forward(&self, input: &Tensor) -> anyhow::Result<Tensor>;
}
```

**Commit Message**: `[IMPL] Implement DemoMLP model architecture with Candle`

#### 2.3 Implement Model Persistence

**Status**: [x] Completed (Enhanced with WASM Support)  
**Dependencies**: 2.2  
**Definition of Done**:

- [x] Model save/load functions work with two-file approach (.toml + .safetensors)
- [x] Round-trip save/load preserves model weights and metadata
- [x] Error handling covers common failure cases including file validation
- [x] Metadata is human-readable and automatically managed
- [x] **NEW**: WASM-compatible loading using memory-based approach
- [x] **NEW**: Layered API supports both file-based and data-based loading

**Implementation Steps**:

- [x] Create `shared/src/persistence.rs` with two-file I/O functions
- [x] Implement `save_model_from_varmap()` with TOML metadata + safetensors weights
- [x] ~~Implement `load_model()` with automatic metadata reading and validation~~ **REPLACED**
- [x] **NEW**: Implement `parse_model_metadata()` for TOML byte parsing
- [x] **NEW**: Implement `load_model_from_data()` using `VarBuilder::from_slice_safetensors()` (WASM-compatible)
- [x] **NEW**: Implement `load_model_from_files()` as convenience wrapper
- [x] Add comprehensive error handling for missing files and invalid data
- [x] Write tests for save/load round-trips and error conditions
- [x] **NEW**: Add tests for memory-based loading approach

**Enhanced Implementation**:

```rust
// Layered API supporting both file-based and memory-based loading
pub fn parse_model_metadata(toml_bytes: &[u8]) -> anyhow::Result<ModelMetadata>;

// WASM-compatible: loads from raw bytes without filesystem operations
pub fn load_model_from_data(
    toml_bytes: &[u8], 
    safetensors_bytes: &[u8], 
    device: &Device
) -> anyhow::Result<DemoMLP>;

// Native convenience: reads files then calls load_model_from_data()  
pub fn load_model_from_files(base_path: &str, device: &Device) -> anyhow::Result<DemoMLP>;

// Unchanged: two-file saving approach
pub fn save_model_from_varmap(
    varmap: &VarMap,
    metadata: &ModelMetadata,
    base_path: &str
) -> anyhow::Result<()>;
```

**Key Benefits**:
- [x] **Universal compatibility**: Works in both native and WASM environments
- [x] **Clean separation**: File I/O separated from model creation logic
- [x] **Flexible usage**: Training uses file-based, Bevy uses memory-based
- [x] **No filesystem dependency**: Core loading logic doesn't require temp files
- [x] **Backward compatibility**: Existing save format unchanged

**Commit Message**: `[IMPL] Enhance model persistence with WASM-compatible memory loading`

#### 2.4 Create Shared Library Public API

**Status**: [x] Completed  
**Dependencies**: 2.1, 2.2, 2.3  
**Definition of Done**:

- `shared/src/lib.rs` exports all public APIs cleanly
- Documentation is complete for all public functions with examples
- API is easy to use from other crates with minimal imports
- Comprehensive integration tests demonstrate typical usage patterns

**Implementation Steps**:

- [x] Design clean public API in `lib.rs` with re-exports
- [x] Re-export key types and functions from model and persistence modules
- [x] Re-export commonly used Candle types for convenience
- [x] Add comprehensive documentation with working examples
- [x] Write extensive integration tests showing training and inference workflows
- [x] Test API ergonomics and ease of use

**Actual Implementation**:

```rust
// Clean, minimal imports needed by users:
// use shared::{DemoMLP, ModelMetadata, Device, VarBuilder, VarMap, save_model_from_varmap};

pub use model::{DemoMLP, ModelMetadata};
pub use persistence::{load_model_from_files, load_model_from_data, parse_model_metadata, save_model_from_varmap};
pub use candle_core::{DType, Device, Tensor};
pub use candle_nn::{VarBuilder, VarMap};
pub use anyhow::Result;
```

**Commit Message**: `[IMPL] Complete shared library with clean public API`

---

## Phase 3: Training Binary Implementation

_Estimated effort: 2-3 hours_

**Goal**: Create functional training binary that generates and saves demo models

### Tasks

#### 3.1 Implement Synthetic Data Generation

**Status**: Completed
**Dependencies**: 2.4  
**Definition of Done**:

- Generates random training data in specified ranges (inputs [-1, 1], targets [0, 1])
- Data works with existing `DemoMLP` architecture and `forward()` method
- Generation is reproducible with seed option
- Basic data validation and logging implemented

**Implementation Steps**:

- [x] Add `rand` dependency to training/Cargo.toml for random number generation
- [x] Create data generation function in `training/src/main.rs`
- [x] Generate inputs as `Vec<[f32; 2]>` in range [-1, 1] using uniform distribution
- [x] Generate synthetic targets as `Vec<f32>` in range [0, 1] (using simple function)
- [x] Convert to Candle `Tensor` format for model compatibility
- [x] Add reproducible random seed option (default + configurable)
- [x] Implement basic data quality checks (range validation, NaN detection)
- [x] Add logging for generation statistics (count, input/target ranges)

**Data Format Approach**:

```rust
// Simple approach using existing types (no TrainingExample struct)
fn generate_training_data(size: usize, seed: Option<u64>) -> anyhow::Result<(Tensor, Tensor)> {
    // Generate inputs: Vec<[f32; 2]> → Tensor shape [size, 2]
    // Generate targets: Vec<f32> → Tensor shape [size, 1]
    // Return (input_tensor, target_tensor) ready for model.forward()
}
```

**Commit Message**: `[IMPL] Implement synthetic training data generation`

#### 3.2 Implement Model Training Pipeline

**Status**: Completed  
**Dependencies**: 3.1  
**Definition of Done**:

- Model is created and initialized properly
- Training loop runs (even with 0 epochs)
- Loss calculation works correctly
- Training process has clear logging

**Implementation Steps**:

- [x] Create model instance with proper device setup
- [x] Implement basic training loop structure
- [x] Add loss calculation (MSE for demonstration)
- [x] Configure for 0 epochs as per specification
- [x] Add progress logging and status messages
- [x] Handle training errors gracefully

**Commit Message**: `[IMPL] Implement model training pipeline with 0-epoch demo`

#### 3.3 Implement Model Saving and Validation

**Status**: Completed  
**Dependencies**: 3.2  
**Definition of Done**:

- Trained model is saved to `models/demo_model.safetensors`
- Saved model can be reloaded and validated
- File size and format are reasonable
- Success/failure is clearly reported

**Implementation Steps**:

- [x] Save model using shared library persistence functions
- [x] Verify saved model by reloading and testing inference
- [x] Add file size and location reporting
- [x] Handle save errors with informative messages
- [x] Clean up any temporary files or resources

**Commit Message**: `[IMPL] Complete training binary with model saving and validation`

---

## Phase 4: Interactive Demo Core

_Estimated effort: 4-5 hours_

**Goal**: Create basic Bevy application with model loading and UI structure

### Tasks

#### 4.1 Set Up Basic Bevy Application

**Status**: Completed  
**Dependencies**: 3.3  
**Definition of Done**:

- Bevy app initializes and runs without errors
- Basic plugin configuration is correct
- Window and rendering system work
- App can be cleanly shut down

**Implementation Steps**:

- [x] Create basic Bevy app structure in `interactive/src/main.rs`
- [x] Configure essential plugins (UI, text, asset loading)
- [x] Set up basic window and rendering
- [x] Test native compilation and execution
- [x] Add basic error handling and logging

**Commit Message**: `[IMPL] Set up basic Bevy application structure`

#### 4.2 Implement Model Loading System

**Status**: [x] Completed (Enhanced with WASM Compatibility Fix)  
**Dependencies**: 4.1  
**Definition of Done**:

- [x] Model loads from embedded assets on application startup
- [x] Loading success/failure is tracked in app state (Loading → Ready/Error)
- [x] Error handling provides useful feedback
- [x] Model is stored as Bevy resource for other systems
- [x] **FIXED**: Works identically on both native and WASM targets

**Implementation Steps**:

- [x] Create `LoadedModel` resource structure
- [x] Implement embedded asset plugin for cross-platform model loading
- [x] Add model loading system with async asset loading
- [x] Add error handling for asset loading failures
- [x] Store loading status using Bevy state management
- [x] **FIXED**: Remove filesystem dependency for WASM compatibility
- [x] Test with both native and WASM targets

**Technical Implementation Evolution**:

1. **Initial approach** (worked on native, failed on WASM):
   ```rust
   // ❌ FAILED: Used temp files, doesn't work in WASM
   let temp_file = std::env::temp_dir().join("embedded_model.safetensors");
   std::fs::write(&temp_file, &safetensors_asset.data)?;
   let vb = VarBuilder::from_mmaped_safetensors(&[&temp_file], ...)?;
   ```

2. **Enhanced approach** (works universally):
   ```rust
   // ✅ SUCCESS: Uses shared::load_model_from_data() - no filesystem needed
   let device = Device::Cpu;
   load_model_from_data(&toml_asset.data, &safetensors_asset.data, &device)
   ```

**Key Benefits**:
- [x] **WASM compatibility**: No "no filesystem on this platform" errors
- [x] **Code reuse**: Leverages shared persistence API consistently  
- [x] **Simplified logic**: 25 lines of temp file code → 3 lines of function call
- [x] **Better maintainability**: Single source of truth for model loading logic

**Commit Message**: `[IMPL] Fix WASM model loading using memory-based persistence API`

#### 4.3 Create Basic UI Layout

**Status**: [x] Completed  
**Dependencies**: 4.2  
**Definition of Done**:

- [x] UI layout matches specification design
- [x] All UI elements are visible and properly positioned
- [x] Text displays are working correctly (fixed visibility issues)
- [x] UI scales reasonably on different screen sizes

**Implementation Steps**:

- [x] Design UI layout system with Bevy UI components
- [x] Create status display for model loading state
- [x] Create input field placeholders (text display for now)
- [x] Create predict button placeholder
- [x] Create output display area
- [x] Test UI layout and basic interactivity
- [x] **FIXED**: Corrected text color visibility for "True Value" and "Error" fields
- [x] **FIXED**: Removed emoji from status display for clean text rendering

**UI Elements Created**:

- [x] Model status indicator with loading/ready/error states
- [x] Two input fields for numbers with labels
- [x] Predict button with hover effects
- [x] Output display area with prediction, true value, and error
- [x] Title/header text with consistent styling

**Key Fixes Applied**:
- Changed "True Value" and "Error" text from `GRAY_SECONDARY` to `TEXT_COLOR` for visibility
- Removed emojis from status display for clean, universal text rendering
- Applied consistent color scheme and styling throughout UI

**Commit Message**: `[IMPL] Create complete UI layout with all elements and display fixes`

---

## Phase 5: Interactive Demo Features

_Estimated effort: 3-4 hours_

**Goal**: Implement user input handling, inference processing, and output display

### Tasks

#### 5.1 Implement Input Handling System

**Status**: [x] Completed  
**Dependencies**: 4.3  
**Definition of Done**:

- [x] Users can input numbers in both input fields
- [x] Input validation works (range checking, number parsing)
- [x] Input state is properly managed in Bevy ECS
- [x] Clear feedback for invalid inputs
- [x] Range indicators show expected input values (-1.0 to 1.0)
- [x] Real-time input sanitization prevents invalid characters
- [x] Professional focus management with click-to-focus behavior

**Implementation Steps**:

- [x] Add `bevy_simple_text_input` dependency for interactive text fields
- [x] Create input state management resources (InputValues, ValidationState)
- [x] Replace static UI with interactive TextInput components
- [x] Implement comprehensive input validation system (-1.0 to 1.0 range)
- [x] Add real-time input sanitization with character filtering and clamping
- [x] Implement proper focus management using TextInputInactive component
- [x] Create visual styling system with focus and validation indicators
- [x] Add range indicators under input labels for user guidance
- [x] Simplify input components by removing redundant InputField component
- [x] Consolidate styling functions to eliminate duplication
- [x] Fix clippy warnings with type aliases for complex query types
- [x] Test complete input handling flow with focus management

**Technical Implementation**:

```rust
// Input state management
#[derive(Resource, Default)]
pub struct InputValues {
    pub value1: Option<f32>,
    pub value2: Option<f32>,
}

#[derive(Resource)]
pub struct ValidationState {
    pub input1_valid: bool,
    pub input2_valid: bool,
    pub input1_empty: bool,
    pub input2_empty: bool,
}

// Key systems implemented:
// - validate_numeric_inputs(): Parse and validate input range (-1.0 to 1.0)
// - sanitize_numeric_inputs(): Real-time character filtering and clamping
// - manage_input_focus(): Click-to-focus with TextInputInactive state management
// - update_input_styling(): Visual feedback based on focus and validation
```

**Key Fixes Applied**:
- **Focus Management Bug**: Fixed TextInputInactive component handling (state vs marker pattern)
- **Input Range**: Updated validation from -10..10 to -1..1 to match training data
- **Code Simplification**: Removed redundant InputField component and duplicate styling
- **Styling Consolidation**: Style functions now return complete (Node, BackgroundColor, BorderColor) tuples
- **Clippy Warnings**: Added type aliases for complex query types, fixed format string

**Commit Message**: `[IMPL] Implement comprehensive input handling with validation and focus management`

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

_Estimated effort: 2-3 hours_

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
    <style>
      /* Basic styling */
    </style>
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

_Estimated effort: 3-4 hours_

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

_Estimated effort: 2-3 hours_

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
