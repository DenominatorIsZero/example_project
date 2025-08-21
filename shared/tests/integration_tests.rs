//! Integration tests for the shared library public API
//!
//! These tests demonstrate typical usage patterns and ensure the API
//! is intuitive and complete for both training and inference workflows.

use shared::{DemoMLP, Device, ModelMetadata, Result, Tensor, VarBuilder, VarMap};
use shared::{load_model, save_model_from_varmap};
use tempfile::tempdir;

/// Test the complete training workflow: create model → train → save
#[test]
fn test_training_workflow() -> Result<()> {
    let device = Device::Cpu;

    // Step 1: Create model architecture
    let metadata = ModelMetadata::new(3, 1, 8)?;

    // Step 2: Initialize model for training
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
    let model = DemoMLP::new(metadata.clone(), vb)?;

    // Step 3: Verify model structure
    assert_eq!(model.metadata.input_size, 3);
    assert_eq!(model.metadata.output_size, 1);
    assert_eq!(model.metadata.hidden_size, 8);

    // Step 4: Test forward pass (simulating training)
    let batch_size = 4;
    let input = Tensor::randn(0f32, 1f32, (batch_size, 3), &device)?;
    let output = model.forward(&input)?;

    // Verify output shape and range
    assert_eq!(output.shape().dims(), &[batch_size, 1]);
    let output_vec = output.to_vec2::<f32>()?;
    for batch in output_vec {
        for value in batch {
            assert!((0.0..=1.0).contains(&value));
        }
    }

    // Step 5: Save trained model
    let temp_dir = tempdir()?;
    let model_path = temp_dir.path().join("test_model");
    let model_path_str = model_path.to_str().unwrap();

    save_model_from_varmap(&varmap, &metadata, model_path_str)?;

    // Verify files were created
    assert!(std::path::Path::new(&format!("{model_path_str}.toml")).exists());
    assert!(std::path::Path::new(&format!("{model_path_str}.safetensors")).exists());

    Ok(())
}

/// Test the complete inference workflow: load model → run inference
#[test]
fn test_inference_workflow() -> Result<()> {
    let device = Device::Cpu;

    // Step 1: Create and save a model (simulating trained model)
    let metadata = ModelMetadata::new(2, 2, 6)?;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
    let _training_model = DemoMLP::new(metadata.clone(), vb)?;

    let temp_dir = tempdir()?;
    let model_path = temp_dir.path().join("inference_model");
    let model_path_str = model_path.to_str().unwrap();
    save_model_from_varmap(&varmap, &metadata, model_path_str)?;

    // Step 2: Load model for inference (this is what users typically do)
    let inference_model = load_model(model_path_str, &device)?;

    // Step 3: Verify loaded model has correct metadata
    assert_eq!(inference_model.metadata.input_size, 2);
    assert_eq!(inference_model.metadata.output_size, 2);
    assert_eq!(inference_model.metadata.hidden_size, 6);

    // Step 4: Run inference on single sample
    let single_input = Tensor::from_vec(vec![0.7f32, -0.2f32], (1, 2), &device)?;
    let single_output = inference_model.forward(&single_input)?;
    assert_eq!(single_output.shape().dims(), &[1, 2]);

    // Step 5: Run inference on batch
    let batch_input = Tensor::from_vec(
        vec![0.1f32, 0.2f32, 0.3f32, 0.4f32, 0.5f32, 0.6f32],
        (3, 2),
        &device,
    )?;
    let batch_output = inference_model.forward(&batch_input)?;
    assert_eq!(batch_output.shape().dims(), &[3, 2]);

    // Verify all outputs are in valid sigmoid range
    let batch_vec = batch_output.to_vec2::<f32>()?;
    for batch in batch_vec {
        for value in batch {
            assert!((0.0..=1.0).contains(&value));
        }
    }

    Ok(())
}

/// Test flexible model architectures
#[test]
fn test_flexible_architectures() -> Result<()> {
    let device = Device::Cpu;

    let architectures = vec![
        (1, 1, 2),  // Minimal: 1→2→1
        (2, 1, 4),  // Default demo: 2→4→1
        (5, 3, 10), // Larger: 5→10→3
        (8, 4, 16), // Even larger: 8→16→4
    ];

    for (input_size, output_size, hidden_size) in architectures {
        let metadata = ModelMetadata::new(input_size, output_size, hidden_size)?;
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
        let model = DemoMLP::new(metadata.clone(), vb)?;

        // Test forward pass
        let input = Tensor::randn(0f32, 1f32, (2, input_size), &device)?;
        let output = model.forward(&input)?;
        assert_eq!(output.shape().dims(), &[2, output_size]);

        // Test save/load round trip
        let temp_dir = tempdir()?;
        let model_path = temp_dir
            .path()
            .join(format!("model_{input_size}_{output_size}_{hidden_size}"));
        let model_path_str = model_path.to_str().unwrap();

        save_model_from_varmap(&varmap, &metadata, model_path_str)?;
        let loaded_model = load_model(model_path_str, &device)?;

        assert_eq!(loaded_model.metadata, metadata);

        // Verify loaded model produces same output shape
        let loaded_output = loaded_model.forward(&input)?;
        assert_eq!(loaded_output.shape().dims(), &[2, output_size]);
    }

    Ok(())
}

/// Test error handling and edge cases
#[test]
fn test_error_handling() -> Result<()> {
    let device = Device::Cpu;

    // Test invalid metadata
    assert!(ModelMetadata::new(0, 1, 4).is_err()); // zero input
    assert!(ModelMetadata::new(1, 0, 4).is_err()); // zero output  
    assert!(ModelMetadata::new(1, 1, 0).is_err()); // zero hidden
    assert!(ModelMetadata::new(20000, 1, 4).is_err()); // too large

    // Test loading nonexistent model
    assert!(load_model("/nonexistent/model", &device).is_err());

    // Test invalid input shapes
    let metadata = ModelMetadata::new(3, 1, 4)?;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
    let model = DemoMLP::new(metadata, vb)?;

    // Wrong input size
    let wrong_input = Tensor::randn(0f32, 1f32, (1, 2), &device)?; // expects 3
    assert!(model.forward(&wrong_input).is_err());

    // Wrong number of dimensions
    let wrong_dims = Tensor::randn(0f32, 1f32, 3, &device)?; // 1D instead of 2D
    assert!(model.forward(&wrong_dims).is_err());

    Ok(())
}

/// Test the convenience new_demo method
#[test]
fn test_demo_model() -> Result<()> {
    let device = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);

    // Test convenience constructor
    let demo_model = DemoMLP::new_demo(vb)?;

    // Verify it creates the default 2→4→1 architecture
    assert_eq!(demo_model.metadata.input_size, 2);
    assert_eq!(demo_model.metadata.output_size, 1);
    assert_eq!(demo_model.metadata.hidden_size, 4);

    // Test it works for inference
    let input = Tensor::from_vec(vec![0.5f32, -0.3f32], (1, 2), &device)?;
    let output = demo_model.forward(&input)?;
    assert_eq!(output.shape().dims(), &[1, 1]);

    // Test save/load works
    let temp_dir = tempdir()?;
    let model_path = temp_dir.path().join("demo_model");
    let model_path_str = model_path.to_str().unwrap();

    save_model_from_varmap(&varmap, &demo_model.metadata, model_path_str)?;
    let loaded_demo = load_model(model_path_str, &device)?;

    assert_eq!(loaded_demo.metadata, demo_model.metadata);

    Ok(())
}

/// Test API ergonomics - this should be easy to use
#[test]
fn test_api_ergonomics() -> Result<()> {
    // This test demonstrates how easy the API should be to use

    // Training scenario - minimal imports needed
    use shared::{DemoMLP, ModelMetadata, save_model_from_varmap};
    use shared::{Device, VarBuilder, VarMap};

    let device = Device::Cpu;

    // One-liner model creation
    let metadata = ModelMetadata::new(2, 1, 4)?;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
    let _model = DemoMLP::new(metadata.clone(), vb)?;

    // One-liner save
    let temp_dir = tempdir()?;
    let model_path = temp_dir.path().join("ergonomic_model");
    save_model_from_varmap(&varmap, &metadata, model_path.to_str().unwrap())?;

    // Inference scenario - minimal imports needed
    use shared::Tensor;
    use shared::load_model;

    // One-liner load
    let model = load_model(model_path.to_str().unwrap(), &device)?;

    // One-liner inference
    let input = Tensor::from_vec(vec![1.0f32, -1.0f32], (1, 2), &device)?;
    let _output = model.forward(&input)?;

    // Everything should work with minimal boilerplate
    Ok(())
}
