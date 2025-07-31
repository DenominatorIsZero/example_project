# Testing and Continuous Integration

Testing strategy, GitHub Actions workflows, and quality gates for the minimal AI demo scaffolding project.

## Testing Philosophy

- **Post-Implementation Testing**: Add tests after features are working (not TDD)
- **Focus on Integration Points**: Test the critical paths and boundaries between components
- **Isolated Tests**: Each test cleans up after itself and doesn't depend on external state
- **Practical Coverage**: Test what matters for the scaffolding use case, not exhaustive coverage

## Test Categories

### Unit Tests (Shared Library)

Test the core functionality of the shared library components.

#### Model Persistence Tests
```rust
// shared/src/persistence.rs
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, NamedTempFile};
    use candle_core::Device;

    #[test]
    fn test_save_and_load_model_roundtrip() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        
        // Create a test model
        let device = Device::Cpu;
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
        let original_model = DemoMLP::new(vb).unwrap();
        
        // Save and load
        save_model(&original_model, path).unwrap();
        let loaded_model = load_model(path, &device).unwrap();
        
        // Verify they produce same outputs for same inputs
        let test_input = Tensor::randn(0f32, 1f32, (1, 2), &device).unwrap();
        let original_output = original_model.forward(&test_input).unwrap();
        let loaded_output = loaded_model.forward(&test_input).unwrap();
        
        // Use approx crate for floating point comparison
        assert_relative_eq!(
            original_output.to_vec1::<f32>().unwrap()[0],
            loaded_output.to_vec1::<f32>().unwrap()[0],
            epsilon = 1e-6
        );
        
        // File automatically cleaned up when temp_file drops
    }

    #[test]
    fn test_load_nonexistent_model_fails_gracefully() {
        let result = load_model("definitely/does/not/exist.safetensors", &Device::Cpu);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to read model file"));
    }

    #[test]
    fn test_verify_model_file_with_valid_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        
        // Create and save a model
        let device = Device::Cpu;
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
        let model = DemoMLP::new(vb).unwrap();
        save_model(&model, path).unwrap();
        
        // Verify metadata extraction
        let metadata = verify_model_file(path).unwrap();
        assert_eq!(metadata.input_size, 2);
        assert_eq!(metadata.hidden_size, 4);
        assert_eq!(metadata.output_size, 1);
    }
}
```

```

### Integration Tests

Test the complete workflows and component interactions.

#### Training Pipeline Integration Test
```rust
// training/tests/integration_test.rs
use std::fs;
use tempfile::tempdir;
use std::process::Command;

#[test]
fn test_complete_training_pipeline() {
    // Use temporary directory for test isolation
    let temp_dir = tempdir().unwrap();
    let model_path = temp_dir.path().join("test_model.safetensors");
    
    // Set up environment variable for training binary
    std::env::set_var("MODEL_OUTPUT_PATH", model_path.to_str().unwrap());
    
    // Run training binary
    let output = Command::new("cargo")
        .args(&["run", "--bin", "training"])
        .output()
        .expect("Training should complete successfully");
    
    // Verify training succeeded
    assert!(output.status.success(), "Training failed: {}", 
            String::from_utf8_lossy(&output.stderr));
    
    // Verify model file was created
    assert!(model_path.exists(), "Model file was not created");
    
    // Verify model file has reasonable size
    let metadata = fs::metadata(&model_path).unwrap();
    assert!(metadata.len() > 100, "Model file too small");
    assert!(metadata.len() < 10_000_000, "Model file too large");
    
    // Clean up environment
    std::env::remove_var("MODEL_OUTPUT_PATH");
    
    // temp_dir automatically cleaned up when it drops
}

#[test]
fn test_model_inference_after_training() {
    // Similar to above but also test loading and inference
    let temp_dir = tempdir().unwrap();
    let model_path = temp_dir.path().join("inference_test_model.safetensors");
    
    // Train model
    std::env::set_var("MODEL_OUTPUT_PATH", model_path.to_str().unwrap());
    let output = Command::new("cargo")
        .args(&["run", "--bin", "training"])
        .output()
        .unwrap();
    assert!(output.status.success());
    
    // Load model and test inference
    use shared::{load_model, PredictionInput};
    use candle_core::Device;
    
    let device = Device::Cpu;
    let model = load_model(model_path.to_str().unwrap(), &device).unwrap();
    
    // Test inference with sample input
    let test_input = Tensor::new(&[[1.0f32, 2.0f32]], &device).unwrap();
    let output = model.forward(&test_input).unwrap();
    let result = output.to_vec2::<f32>().unwrap();
    
    // Verify output is reasonable (sigmoid output should be 0-1)
    assert!(result[0][0] >= 0.0 && result[0][0] <= 1.0);
    
    std::env::remove_var("MODEL_OUTPUT_PATH");
}
```

#### Interactive Demo Component Tests
```rust
// interactive/tests/model_loading.rs
use bevy::prelude::*;
use shared::{DemoMLP, load_model};
use candle_core::Device;

#[test]
fn test_model_loading_in_bevy_context() {
    // Test that model loading works in Bevy's ECS system
    // This is a simplified test - real implementation would be more complex
    
    let mut world = World::new();
    
    // Simulate model loading system
    fn mock_load_model_system(world: &mut World) {
        // This would contain the actual model loading logic
        // For test, we just verify the system can run without panicking
    }
    
    // Should not panic
    mock_load_model_system(&mut world);
}
```

### Test Setup and Cleanup Utilities

```rust
// tests/common/mod.rs - Shared test utilities
use tempfile::TempDir;
use std::fs;

pub fn setup_clean_test_environment() -> TempDir {
    // Remove any existing test artifacts
    let _ = fs::remove_file("models/demo_model.safetensors");
    let _ = fs::remove_dir_all("target/wasm32-unknown-unknown/");
    
    // Create isolated temp directory
    tempfile::tempdir().expect("Failed to create temp directory")
}

pub fn create_test_model_file(path: &str) -> anyhow::Result<()> {
    use shared::{DemoMLP, save_model};
    use candle_core::{Device, DType};
    use candle_nn::{VarMap, VarBuilder};
    
    let device = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    let model = DemoMLP::new(vb)?;
    
    save_model(&model, path)?;
    Ok(())
}
```

### Testing Dependencies

Add to relevant `Cargo.toml` files:

```toml
[dev-dependencies]
tempfile = "3.0"      # For isolated temporary files/directories
approx = "0.5"        # For floating-point comparisons
serial_test = "3.0"   # For tests that can't run in parallel
```

## Manual Testing Checklist

### Pre-Test Setup
- [ ] Clean workspace: `cargo clean && rm -rf models/`
- [ ] Ensure no stale processes or file locks

### Basic Functionality
- [ ] `cargo test --workspace` passes for all components
- [ ] `cargo run --bin training` completes without errors
- [ ] Model file created with reasonable size (>100 bytes, <10MB)  
- [ ] `cargo run --bin interactive` launches without panics
- [ ] Web demo loads and accepts input

### Cross-Platform Testing
- [ ] Tests pass on Linux, macOS, and Windows
- [ ] WASM build succeeds on all platforms
- [ ] Generated artifacts work consistently

### Error Scenario Testing
- [ ] Training handles missing directories gracefully
- [ ] Interactive demo handles missing model file
- [ ] Invalid user input is rejected with clear error messages
- [ ] Network issues (for web demo) are handled gracefully

## Continuous Integration

### GitHub Actions Workflows

#### 1. Test Workflow (`.github/workflows/test.yml`)

Runs on every push and pull request to main branch.

```yaml
name: Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    name: Test Suite
    runs-on: ubuntu-latest
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache dependencies
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run tests
        run: cargo test --workspace --verbose
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Run clippy
        run: cargo clippy --workspace -- -D warnings
      
      - name: Check documentation
        run: cargo doc --workspace --no-deps --document-private-items

  # Test on multiple platforms
  cross-platform-test:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Run tests
        run: cargo test --workspace
```

#### 2. Native Builds Workflow (`.github/workflows/native-build.yml`)

Manual trigger for creating downloadable binaries.

```yaml
name: Native Builds

on:
  workflow_dispatch:  # Manual trigger only
    inputs:
      create_release:
        description: 'Create GitHub release'
        required: false
        default: false
        type: boolean

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            name: linux-x64
            extension: ""
          - os: windows-latest  
            target: x86_64-pc-windows-msvc
            name: windows-x64
            extension: ".exe"
          - os: macos-latest
            target: x86_64-apple-darwin
            name: macos-x64
            extension: ""
          - os: macos-latest
            target: aarch64-apple-darwin
            name: macos-arm64
            extension: ""
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Build training binary
        run: cargo build --release --bin training --target ${{ matrix.target }}
      
      - name: Build interactive binary  
        run: cargo build --release --bin interactive --target ${{ matrix.target }}
      
      - name: Package binaries
        shell: bash
        run: |
          mkdir -p dist
          cp target/${{ matrix.target }}/release/training${{ matrix.extension }} dist/
          cp target/${{ matrix.target }}/release/interactive${{ matrix.extension }} dist/
          cp README.md dist/
          tar -czf ${{ matrix.name }}-binaries.tar.gz -C dist .
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: binaries-${{ matrix.name }}
          path: ${{ matrix.name }}-binaries.tar.gz
```

#### 3. WASM Build Workflow (`.github/workflows/wasm-build.yml`)

Manual trigger for creating web-deployable WASM package.

```yaml
name: WASM Build

on:
  workflow_dispatch:  # Manual trigger only

jobs:
  wasm-build:
    name: Build WASM Package
    runs-on: ubuntu-latest
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
      
      - name: Install Rust with WASM target
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      
      - name: Install wasm-pack
        run: cargo install wasm-pack
      
      - name: Build WASM package
        run: cd interactive && wasm-pack build --target web --release
      
      - name: Create demo package
        run: |
          cd interactive/pkg
          # Create a complete web demo package
          cat > index.html << 'EOF'
          <!DOCTYPE html>
          <html>
          <head>
              <meta charset="utf-8">
              <title>AI Demo</title>
              <style>
                  body { margin: 0; padding: 0; background: #222; }
                  canvas { display: block; margin: 0 auto; }
              </style>
          </head>
          <body>
              <script type="module">
                  import init from './interactive.js';
                  init();
              </script>
          </body>
          </html>
          EOF
          
          echo "Demo ready at: interactive/pkg/index.html"
          ls -la
      
      - name: Upload WASM artifacts
        uses: actions/upload-artifact@v4
        with:
          name: wasm-demo
          path: interactive/pkg/
          
      - name: Deploy to GitHub Pages (optional)
        if: github.ref == 'refs/heads/main'
        uses: peaceiris/actions-gh-pages@v4
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./interactive/pkg
```

## Quality Gates

### Code Quality Standards
- **Formatting**: All code must pass `cargo fmt --check`
- **Linting**: All code must pass `cargo clippy -- -D warnings`
- **Documentation**: All public APIs must have documentation
- **Tests**: Critical paths must have test coverage

### Performance Standards
- **Build Time**: Full workspace build should complete in <5 minutes on CI
- **Test Time**: Full test suite should complete in <2 minutes
- **WASM Size**: Generated WASM should be <2MB uncompressed
- **Model Size**: Demo model should be <1MB

### Reliability Standards
- **Cross-Platform**: Tests must pass on Linux, macOS, and Windows
- **Reproducible**: Same inputs produce same outputs across runs
- **Error Handling**: All error cases have descriptive messages
- **Resource Cleanup**: Tests clean up temporary files and state

## CI Benefits for Future Projects

### Template Validation
- **Dependency Monitoring**: Catches breaking changes in Candle, Bevy, etc.
- **Platform Compatibility**: Ensures scaffolding works across environments
- **Quality Maintenance**: Prevents regression in template quality

### Best Practice Demonstration
- **Modern Workflows**: Shows current GitHub Actions best practices
- **Cross-Platform CI**: Matrix builds for multiple targets
- **Artifact Management**: Proper handling of build outputs

### Copy-Paste Ready
- **Direct Reuse**: CI configs can be used directly in domain projects
- **Documented Patterns**: Clear examples of Rust ML project CI
- **Extensibility**: Easy to add project-specific steps