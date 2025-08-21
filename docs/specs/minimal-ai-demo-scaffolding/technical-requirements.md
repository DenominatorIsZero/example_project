# Technical Requirements

Detailed component specifications, dependencies, and APIs for the minimal AI demo scaffolding project.

## 1. Training Binary (`training/`)

### Core Functionality
- **Model Architecture**: Simple Multi-Layer Perceptron (MLP)
  - Input layer: 2 neurons
  - Hidden layer: 4 neurons with ReLU activation
  - Output layer: 1 neuron with sigmoid activation
- **Training Data**: Generate synthetic dataset (configurable size) with inputs in range [-1, 1] and targets in range [0, 1]
- **Training Process**: Initialize model but train for 0 epochs (demonstration purposes)
- **Model Persistence**: Save model using two-file approach to `models/demo_model.toml` and `models/demo_model.safetensors`
- **Validation**: Load saved model and verify inference capability

### Technical Specifications

#### Model Architecture
```rust
// shared/src/model.rs
pub struct DemoMLP {
    pub fc1: candle_nn::Linear,  // input_size → hidden_size
    pub fc2: candle_nn::Linear,  // hidden_size → output_size
    pub metadata: ModelMetadata,
}

pub struct ModelMetadata {
    pub input_size: usize,
    pub output_size: usize,
    pub hidden_size: usize,
}

impl DemoMLP {
    pub fn new(metadata: ModelMetadata, vb: VarBuilder) -> anyhow::Result<Self>;
    pub fn new_demo(vb: VarBuilder) -> anyhow::Result<Self>;  // Creates 2→4→1 default
    pub fn forward(&self, input: &Tensor) -> anyhow::Result<Tensor>;
}

impl ModelMetadata {
    pub fn new(input_size: usize, output_size: usize, hidden_size: usize) -> anyhow::Result<Self>;
    pub fn validate(&self) -> anyhow::Result<()>;
}
```

#### Data Format
```rust
// Note: No separate types.rs module - data structures are embedded in relevant modules
// Training data can be represented as simple tuples or vectors:

// For training (example approaches):
type TrainingData = Vec<(Vec<f32>, Vec<f32>)>;  // (inputs, targets)
// OR
struct TrainingBatch {
    inputs: Tensor,   // Shape: [batch_size, input_size]
    targets: Tensor,  // Shape: [batch_size, output_size]
}

// ModelMetadata is defined in model.rs and serialized to .toml files
pub struct ModelMetadata {
    pub input_size: usize,    // With validation (> 0, < 10,000)
    pub output_size: usize,   // With validation (> 0, < 10,000)  
    pub hidden_size: usize,   // With validation (> 0, < 10,000)
}
```

### Dependencies
```toml
# training/Cargo.toml
[dependencies]
candle-core = { workspace = true }
candle-nn = { workspace = true }
anyhow = { workspace = true }
shared = { path = "../shared" }

[dev-dependencies]
tempfile = "3.0"
```

### Success Criteria
✅ Binary compiles without errors  
✅ Generates random training data  
✅ Creates and initializes MLP model  
✅ Saves model to safetensors format  
✅ Loads model and performs test inference  
✅ Outputs clear success/failure messages  

## 2. Interactive Demo (`interactive/`)

### Core Functionality
- **Model Loading**: Load `models/demo_model.safetensors` on startup
- **User Interface**: Simple input/output interface
- **Inference Pipeline**: Process user input through loaded model
- **WASM Compilation**: Build for web deployment

### UI Layout
```
┌─────────────────────────────────┐
│     Minimal AI Demo             │
│                                 │
│ Model Status: ✅ Loaded         │
│                                 │
│ Input 1: [____] Input 2: [____] │
│                                 │
│         [Predict]               │
│                                 │
│ Output: 0.7234                  │
└─────────────────────────────────┘
```

### Technical Specifications

#### Application Architecture
```rust
// interactive/src/main.rs - Key components and events
#[derive(Component)]
pub struct InputField {
    pub value: String,
    pub field_id: usize,
}

#[derive(Component)]
pub struct OutputDisplay {
    pub value: Option<f32>,
}

#[derive(Event)]
pub struct PredictionRequest {
    pub input1: f32,
    pub input2: f32,
}

#[derive(Event)]
pub struct PredictionResult {
    pub output: f32,
}

// System functions (signatures only)
fn setup_ui(commands: Commands, asset_server: Res<AssetServer>);
fn handle_button_input(/* ... query UI components, send prediction events ... */);
fn process_predictions(/* ... handle prediction events, run inference ... */);
fn update_ui_displays(/* ... update text displays with results ... */);
```

#### Model Integration
```rust
// Resource for loaded model
#[derive(Resource)]
pub struct LoadedModel {
    pub model: shared::DemoMLP,
}

// Pseudo code for model loading workflow
fn load_model_system() {
    // 1. Attempt to load model file
    // 2. If successful, insert LoadedModel resource
    // 3. If failed, set error status for UI display
    // 4. Update model status component
}

// Pseudo code for inference workflow  
fn run_inference(model: &DemoMLP, input1: f32, input2: f32) -> anyhow::Result<f32> {
    // 1. Create tensor from inputs
    // 2. Run model.forward()
    // 3. Extract output value
    // 4. Return result
}
```

### Dependencies
```toml
# interactive/Cargo.toml
[dependencies]
bevy = "0.16"
candle-core = "0.6"
wasm-bindgen = "0.2"
web-sys = "0.3"
shared = { path = "../shared" }

getrandom = { version = "0.3", features = ["wasm_js"] }

[lib]
crate-type = ["cdylib"]
```

### Build Process
```bash
# Development build
cd interactive
wasm-pack build --target web --dev

# Production build  
cd interactive
wasm-pack build --target web --release

# Local testing
cd interactive/pkg && python -m http.server 8000
```

### Success Criteria
✅ Binary compiles for native target  
✅ WASM compilation succeeds without errors  
✅ Loads model file and displays success status  
✅ UI renders correctly with input fields and button  
✅ User can input numbers and trigger prediction  
✅ Model inference produces output  
✅ Demo runs in web browser  

## 3. Shared Library (`shared/`)

### Core Functionality
- **Model Definition**: Common MLP structure used by both binaries
- **Data Types**: Shared input/output formats
- **Model Persistence**: Model saving/loading functionality

### Actual Module Organization

#### model.rs - Flexible Model Architecture
```rust
pub struct DemoMLP {
    pub fc1: candle_nn::Linear,    // input_size → hidden_size
    pub fc2: candle_nn::Linear,    // hidden_size → output_size  
    pub metadata: ModelMetadata,   // Architecture specification
}

pub struct ModelMetadata {
    pub input_size: usize,   // Validated: > 0, < 10,000
    pub output_size: usize,  // Validated: > 0, < 10,000
    pub hidden_size: usize,  // Validated: > 0, < 10,000
}

impl DemoMLP {
    // Create model with custom architecture
    pub fn new(metadata: ModelMetadata, vb: VarBuilder) -> anyhow::Result<Self>;
    
    // Convenience constructor for default 2→4→1 demo architecture
    pub fn new_demo(vb: VarBuilder) -> anyhow::Result<Self>;
    
    // Forward pass: input → fc1 → ReLU → fc2 → Sigmoid
    pub fn forward(&self, input: &Tensor) -> anyhow::Result<Tensor>;
}

impl ModelMetadata {
    pub fn new(input_size: usize, output_size: usize, hidden_size: usize) -> anyhow::Result<Self>;
    pub fn validate(&self) -> anyhow::Result<()>;
}
```

#### No separate types.rs module
```rust
// Data structures are embedded in their relevant modules:
// - ModelMetadata is in model.rs and handles serialization via serde
// - Training data is handled as simple Rust types (Vec, Tensor)
// - No formal PredictionInput/Output structs - direct Tensor usage

// For training data generation, simple approaches work well:
type TrainingData = Vec<(Vec<f32>, f32)>;  // (inputs, target)

// For model I/O, Tensors are used directly:
// Input: Tensor with shape [batch_size, input_size]  
// Output: Tensor with shape [batch_size, output_size]

// ModelMetadata handles TOML serialization automatically:
// File format: {base_path}.toml contains readable architecture info
```

#### persistence.rs - Two-File Model I/O
```rust
// Two-file approach: .toml metadata + .safetensors weights
pub fn save_model_from_varmap(
    varmap: &VarMap, 
    metadata: &ModelMetadata, 
    base_path: &str
) -> anyhow::Result<()>;

pub fn load_model(base_path: &str, device: &Device) -> anyhow::Result<DemoMLP>;

// Implementation details:
// save_model_from_varmap creates two files:
// - {base_path}.toml - Human-readable metadata (input_size, output_size, hidden_size)
// - {base_path}.safetensors - Efficient binary weight storage

// load_model workflow:
// 1. Read {base_path}.toml to get ModelMetadata
// 2. Validate metadata with ModelMetadata::validate()
// 3. Load weights from {base_path}.safetensors using memory-mapped safetensors
// 4. Create VarBuilder from loaded weights
// 5. Reconstruct DemoMLP with metadata and loaded weights

// Benefits of two-file approach:
// - Metadata is human-readable and easily inspectable
// - Weights are efficiently stored and memory-mapped for fast loading
// - Clear separation of concerns
// - Easy to verify model architecture without loading full weights
```

### Public API Exports (lib.rs)
```rust
// Clean public API that re-exports everything users need
pub use model::{DemoMLP, ModelMetadata};
pub use persistence::{load_model, save_model_from_varmap};

// Re-export commonly needed Candle types for convenience
pub use candle_core::{DType, Device, Tensor};
pub use candle_nn::{VarBuilder, VarMap};

// Re-export Result type for convenience
pub use anyhow::Result;

// Usage example:
// use shared::{DemoMLP, ModelMetadata, Device, VarBuilder, VarMap, save_model_from_varmap};
```

### Dependencies
```toml
# shared/Cargo.toml
[dependencies]
candle-core = { workspace = true }
candle-nn = { workspace = true }
safetensors = "0.6"
serde = { workspace = true }
anyhow = { workspace = true }
toml = "0.9.5"

[dev-dependencies]
tempfile = "3.0"
approx = "0.5"
serde_json = "1.0"
```

### Success Criteria
✅ Compiles as library crate  
✅ Model definition works in both training and interactive contexts  
✅ Data types are properly serializable  
✅ Persistence functions handle errors gracefully  
✅ All public APIs are well-documented  

## 4. Workspace Configuration

### Root Cargo.toml
```toml
[workspace]
members = ["training", "interactive", "shared"]
resolver = "2"

[workspace.dependencies]
candle-core = "0.6"
candle-nn = "0.6"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }

[workspace.metadata.wasm-pack.profile.release]
wee-alloc = true
```

## 5. Constraints and Considerations

### Technical Constraints
- **Model Size**: Keep under 1MB for fast web loading
- **WASM Compatibility**: All dependencies must support WASM compilation
- **Browser Support**: Target modern browsers with WebAssembly support
- **Build Time**: Keep compilation fast for development iteration

### Design Constraints
- **Simplicity**: Avoid complex ML concepts or advanced Bevy features
- **Documentation**: Every public function must have clear documentation
- **Error Handling**: Fail gracefully with informative error messages
- **Testability**: Structure code for easy unit testing

### Future Extensibility
- **Model Architecture**: Easy to swap MLP for other architectures
- **Data Format**: Generic enough for different input/output types
- **UI Framework**: Bevy components can be extended for richer interfaces
- **Deployment**: Structure supports integration with various web frameworks

## 6. Validation Checklist

### Build Validation
- [ ] `cargo check --workspace` passes without errors
- [ ] `cargo build --workspace` compiles all binaries
- [ ] `cargo build --release --workspace` creates optimized builds
- [ ] WASM build: `cd interactive && wasm-pack build --target web --release`

### Functionality Validation
- [ ] `cargo run --bin training` completes successfully
- [ ] Model file is created in `models/` directory with reasonable size (>100 bytes, <10MB)
- [ ] `cargo run --bin interactive` launches without errors
- [ ] WASM build produces working web demo
- [ ] UI accepts input and produces output
- [ ] All error conditions are handled gracefully

### Code Quality Validation
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace -- -D warnings` passes
- [ ] All public APIs have documentation
- [ ] Code follows Rust best practices and conventions