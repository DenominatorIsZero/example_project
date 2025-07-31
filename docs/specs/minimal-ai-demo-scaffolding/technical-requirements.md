# Technical Requirements

Detailed component specifications, dependencies, and APIs for the minimal AI demo scaffolding project.

## 1. Training Binary (`training/`)

### Core Functionality
- **Model Architecture**: Simple Multi-Layer Perceptron (MLP)
  - Input layer: 2 neurons
  - Hidden layer: 4 neurons with ReLU activation
  - Output layer: 1 neuron with sigmoid activation
- **Training Data**: Generate synthetic dataset of 100 random samples in range [-1, 1]
- **Training Process**: Initialize model but train for 0 epochs (demonstration purposes)
- **Model Persistence**: Save model weights to `models/demo_model.safetensors`
- **Validation**: Load saved model and verify inference capability

### Technical Specifications

#### Model Architecture
```rust
// shared/src/model.rs
pub struct DemoMLP {
    fc1: candle_nn::Linear,  // 2 → 4
    fc2: candle_nn::Linear,  // 4 → 1
}

impl DemoMLP {
    pub fn new(vb: VarBuilder) -> anyhow::Result<Self>;
    pub fn forward(&self, input: &Tensor) -> anyhow::Result<Tensor>;
}
```

#### Data Format
```rust
// shared/src/types.rs
pub struct TrainingExample {
    pub input: [f32; 2],    // Values in range [-1, 1]
    pub target: f32,        // Values in range [0, 1] (sigmoid output)
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

### Dependencies
```toml
# training/Cargo.toml
[dependencies]
candle-core = "0.6"
candle-nn = "0.6"
anyhow = "1.0"
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
bevy = { version = "0.14", default-features = false, features = [
    "bevy_ui", "bevy_text", "bevy_asset", "bevy_render", "bevy_core_pipeline"
] }
candle-core = "0.6"
wasm-bindgen = "0.2"
web-sys = "0.3"
shared = { path = "../shared" }

[dependencies.bevy]
# WASM-specific features
default-features = false
features = ["webgl2"]

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

### Module Organization

#### model.rs - Model Architecture
```rust
pub struct DemoMLP {
    pub fc1: candle_nn::Linear,  // 2 → 4
    pub fc2: candle_nn::Linear,  // 4 → 1
}

impl DemoMLP {
    pub fn new(vb: VarBuilder) -> anyhow::Result<Self>;
    pub fn forward(&self, input: &Tensor) -> anyhow::Result<Tensor>;
}
```

#### types.rs - Data Structures
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionInput {
    pub values: [f32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionOutput {
    pub value: f32,
}

#[derive(Debug, Clone)]
pub struct ModelMetadata {
    pub input_size: usize,
    pub output_size: usize,
    pub hidden_size: usize,
}
```

#### persistence.rs - Model I/O
```rust
pub fn save_model(model: &DemoMLP, path: &str) -> anyhow::Result<()>;
pub fn load_model(path: &str, device: &Device) -> anyhow::Result<DemoMLP>;  
pub fn verify_model_file(path: &str) -> anyhow::Result<ModelMetadata>;

// Pseudo code for save_model workflow:
// 1. Extract tensor data from model.fc1 and model.fc2
// 2. Create HashMap with tensor names ("fc1.weight", "fc1.bias", etc.)
// 3. Serialize to safetensors format with proper metadata
// 4. Write to file with error context

// Pseudo code for load_model workflow:
// 1. Read file and validate it exists/is readable
// 2. Deserialize safetensors format
// 3. Extract tensors by expected names and validate shapes
// 4. Create VarMap and reconstruct DemoMLP with loaded weights

// Pseudo code for verify_model_file workflow:
// 1. Load safetensors header without loading full tensor data
// 2. Check required tensor names exist
// 3. Validate tensor shapes match expected architecture
// 4. Return ModelMetadata with extracted information
```

### Dependencies
```toml
# shared/Cargo.toml
[dependencies]
candle-core = "0.6"
candle-nn = "0.6"
safetensors = "0.4"
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"

[dev-dependencies]
tempfile = "3.0"
approx = "0.5"
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