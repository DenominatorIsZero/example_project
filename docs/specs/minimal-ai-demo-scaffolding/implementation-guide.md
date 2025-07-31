# Implementation Guide

Development approach, common pitfalls, and future extension guidelines for the minimal AI demo scaffolding project.

## Development Approach

### 1. Incremental Development Strategy

Build and validate each component independently before integration:

**Phase 1: Shared Library Foundation**
- Start with basic data types and model structure
- Implement model persistence (save/load safetensors)
- Add comprehensive error handling
- Write unit tests for core functionality

**Phase 2: Training Binary**
- Implement synthetic data generation
- Create model training pipeline (0 epochs for demo)
- Add model validation and testing
- Ensure clean CLI output and error reporting

**Phase 3: Interactive Demo**
- Build basic Bevy UI with placeholder model
- Implement model loading and inference
- Add input/output handling
- Test native compilation first, then WASM

**Phase 4: Integration and Polish**
- End-to-end testing of complete workflow
- Performance optimization
- Documentation and examples
- CI/CD setup and validation

### 2. Early Integration Testing

Test integration points frequently to catch issues early:

```rust
// Integration test example - run after each phase
#[test]
fn test_end_to_end_workflow() {
    // 1. Generate training data
    // 2. Train and save model
    // 3. Load model in interactive context
    // 4. Perform inference
    // 5. Validate outputs
}
```

### 3. Error-First Design

Handle error cases before implementing happy paths:

```rust
// Example: Model loading with comprehensive error handling
pub fn load_model(path: &str, device: &Device) -> anyhow::Result<DemoMLP> {
    // Check file exists
    if !Path::new(path).exists() {
        anyhow::bail!("Model file not found: {}", path);
    }
    
    // Check file size is reasonable
    let metadata = fs::metadata(path)
        .with_context(|| format!("Cannot read file metadata: {}", path))?;
    if metadata.len() == 0 {
        anyhow::bail!("Model file is empty: {}", path);
    }
    if metadata.len() > 100_000_000 { // 100MB limit
        anyhow::bail!("Model file too large: {} bytes", metadata.len());
    }
    
    // Attempt to load and parse
    let data = fs::read(path)
        .with_context(|| format!("Failed to read model file: {}", path))?;
        
    // ... rest of loading logic with detailed error context
}
```

### 4. Documentation-Driven Development

Write documentation as you code, not after:

```rust
/// Loads a trained model from a safetensors file.
/// 
/// # Arguments
/// * `path` - Path to the .safetensors model file
/// * `device` - Candle device to load the model onto (CPU/GPU)
/// 
/// # Returns
/// * `Ok(DemoMLP)` - Successfully loaded model
/// * `Err(anyhow::Error)` - Loading failed with descriptive error
/// 
/// # Examples
/// ```
/// use candle_core::Device;
/// let device = Device::Cpu;
/// let model = load_model("models/demo_model.safetensors", &device)?;
/// ```
/// 
/// # Errors
/// This function will return an error if:
/// - The file doesn't exist or isn't readable
/// - The file format is invalid or corrupted
/// - The model architecture doesn't match expected structure
pub fn load_model(path: &str, device: &Device) -> anyhow::Result<DemoMLP> {
    // Implementation...
}
```

## Common Pitfalls and Solutions

### 1. WASM Compilation Issues

**Problem**: Dependencies that work natively fail in WASM builds.

**Solution**: Test WASM compilation early and often:

```bash
# Test WASM build after adding each dependency
cd interactive
wasm-pack build --target web --dev

# Common WASM-incompatible features to avoid:
# - File system operations (use web APIs instead)
# - Threading (single-threaded in WASM)
# - Some system libraries
```

**Bevy WASM-specific configuration**:
```toml
[dependencies.bevy]
version = "0.14"
default-features = false
features = [
    "bevy_ui",
    "bevy_text", 
    "bevy_asset",
    "bevy_render",
    "webgl2"  # WASM-specific
]
```

### 2. Model Loading Failures

**Problem**: Model files fail to load due to path issues, corruption, or format changes.

**Solution**: Robust error handling and validation:

```rust
pub fn verify_model_file(path: &str) -> anyhow::Result<ModelMetadata> {
    let data = fs::read(path)
        .with_context(|| format!("Cannot read model file: {}", path))?;
    
    let safetensors = SafeTensors::deserialize(&data)
        .context("Invalid safetensors format")?;
    
    // Verify expected tensor names exist
    let required_tensors = ["fc1.weight", "fc1.bias", "fc2.weight", "fc2.bias"];
    for tensor_name in required_tensors {
        if safetensors.get(tensor_name).is_err() {
            anyhow::bail!("Missing required tensor: {}", tensor_name);
        }
    }
    
    // Extract and validate dimensions
    let fc1_weight = safetensors.get("fc1.weight")?;
    if fc1_weight.shape() != &[4, 2] {
        anyhow::bail!("Invalid fc1 weight shape: {:?}", fc1_weight.shape());
    }
    
    Ok(ModelMetadata {
        input_size: 2,
        hidden_size: 4,
        output_size: 1,
    })
}
```

### 3. UI Responsiveness Issues

**Problem**: Long-running model operations block the UI thread.

**Solution**: Use Bevy's async systems and proper resource management:

```rust
// Don't do this - blocks UI
fn bad_inference_system(
    model: Res<LoadedModel>,
    mut query: Query<&mut OutputDisplay>
) {
    // Long-running inference blocks everything
    let result = model.predict_slow(&input); // BLOCKS UI!
    // ... update UI
}

// Do this - non-blocking approach
fn good_inference_system(
    model: Res<LoadedModel>,
    mut query: Query<&mut OutputDisplay>,
    mut commands: Commands,
) {
    // For WASM, keep operations fast or use web workers
    let result = model.predict_fast(&input); // Quick operation
    
    // Or spawn async task for heavy work
    commands.spawn(InferenceTask {
        input: input.clone(),
        status: TaskStatus::Running,
    });
}
```

### 4. Dependency Conflicts

**Problem**: Different targets (native vs WASM) require different dependency configurations.

**Solution**: Use feature flags and careful dependency management:

```toml
[dependencies]
# Shared base dependencies
candle-core = "0.6"
anyhow = "1.0"

# Platform-specific features
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tokio = { version = "1.0", features = ["full"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
web-sys = "0.3"

[features]
default = []
native-only = ["tokio"]
wasm-only = ["wasm-bindgen", "web-sys"]
```

### 5. Build Performance Issues

**Problem**: Compilation times become slow, especially for WASM builds.

**Solution**: Optimize build configuration and dependencies:

```toml
# Optimize for development speed
[profile.dev]
opt-level = 1  # Some optimization for faster dev builds

# Optimize dependencies even in debug mode
[profile.dev.package."*"]
opt-level = 2

# Fast release builds
[profile.release]
lto = "thin"        # Link-time optimization
codegen-units = 1   # Better optimization
panic = "abort"     # Smaller binaries
```


## Best Practices for Scaffolding Projects

### 1. Keep It Simple, But Complete

**Do:**
- Implement the full workflow, even if trivially
- Use well-established patterns and libraries
- Provide clear examples and documentation
- Handle common error cases gracefully

**Don't:**
- Add impressive but complex features
- Use cutting-edge or experimental dependencies
- Optimize prematurely for performance
- Skip error handling "for simplicity"

### 2. Structure for Extension

**Do:**
- Use traits and generics where appropriate
- Separate concerns cleanly (model/data/UI)
- Document extension points clearly
- Provide examples of how to extend

**Don't:**
- Over-engineer with unnecessary abstractions
- Create complex inheritance hierarchies
- Make assumptions about future use cases
- Couple components tightly together

### 3. Demonstrate Best Practices

**Do:**
- Follow current Rust conventions and idioms
- Show proper error handling patterns
- Include comprehensive testing examples
- Set up modern CI/CD workflows

**Don't:**
- Use outdated patterns or libraries
- Skip testing "because it's just a demo"
- Ignore performance considerations entirely
- Use poor security practices

### 4. Document the "Why"

**Do:**
- Explain architectural decisions in comments
- Document common pitfalls and solutions
- Provide clear getting-started instructions
- Include troubleshooting guides

**Don't:**
- Assume users know the technology stack
- Leave important decisions unexplained
- Focus only on "what" without "why"
- Skip examples of common usage patterns

## Maintenance and Updates

### 1. Dependency Management

Stay current with the ecosystem:

```bash
# Regular dependency updates
cargo update

# Check for outdated dependencies
cargo outdated

# Security audits
cargo audit
```

### 2. Compatibility Testing

Ensure scaffolding works across environments:

```bash
# Test on multiple Rust versions
rustup toolchain install 1.70.0
cargo +1.70.0 test

# Test WASM builds regularly
cd interactive && wasm-pack build --target web

# Validate on different platforms
# (automated via CI matrix builds)
```

### 3. Documentation Updates

Keep documentation synchronized with code:

- Update examples when APIs change
- Refresh dependency versions in docs
- Add new common pitfalls as discovered
- Update performance benchmarks periodically

This implementation guide provides the foundation for building a robust, extensible AI project scaffolding that will serve future projects well.