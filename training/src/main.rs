// Training binary for AI demo scaffolding
// Generates synthetic training data and trains the demo MLP model

use anyhow::Result;
use candle_core::IndexOp;
use candle_nn::optim::Optimizer;
use candle_optimisers::esgd::{ParamsSGD, SGD};
use rand::prelude::*;
use shared::{DType, DemoMLP, Device, Tensor, VarBuilder, VarMap, save_model_from_varmap};
use std::time::Instant;

/// Generate synthetic training data for the demo MLP
///
/// Creates inputs in range [-1, 1] and targets in range [0, 1] using a simple function.
/// The target function is: target = (input1 + input2).tanh() * 0.5 + 0.5
/// This ensures targets are always in [0, 1] range for sigmoid output compatibility.
fn generate_training_data(size: usize, seed: Option<u64>) -> Result<(Tensor, Tensor)> {
    let device = Device::Cpu;

    // Initialize random number generator with seed for reproducibility
    let mut rng = match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::seed_from_u64(42), // Default seed for consistency
    };

    // Generate inputs: Vec<[f32; 2]> with uniform distribution in [-1, 1]
    let mut inputs = Vec::with_capacity(size);
    let mut targets = Vec::with_capacity(size);

    for _ in 0..size {
        let input1: f32 = rng.random_range(-1.0..=1.0);
        let input2: f32 = rng.random_range(-1.0..=1.0);

        // Simple synthetic target function: (input1 + input2).tanh() * 0.5 + 0.5
        // This maps any sum to [0, 1] range
        let target: f32 = (input1 + input2).tanh() * 0.5 + 0.5;

        inputs.push(input1);
        inputs.push(input2);
        targets.push(target);
    }

    // Validate data quality
    validate_data(&inputs, &targets)?;

    // Convert to Candle tensors
    let input_tensor = Tensor::from_vec(inputs, (size, 2), &device)?;
    let target_tensor = Tensor::from_vec(targets, (size, 1), &device)?;

    // Log generation statistics
    println!("Generated {size} training samples");
    println!("Input tensor shape: {:?}", input_tensor.shape());
    println!("Target tensor shape: {:?}", target_tensor.shape());

    Ok((input_tensor, target_tensor))
}

/// Validate generated data for quality and range constraints
fn validate_data(inputs: &[f32], targets: &[f32]) -> Result<()> {
    // Check for NaN or infinite values
    for (i, &value) in inputs.iter().enumerate() {
        if !value.is_finite() {
            anyhow::bail!("Invalid input value at index {}: {}", i, value);
        }
    }

    for (i, &value) in targets.iter().enumerate() {
        if !value.is_finite() {
            anyhow::bail!("Invalid target value at index {}: {}", i, value);
        }
    }

    // Check input ranges [-1, 1]
    let (min_input, max_input) = inputs
        .iter()
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), &x| {
            (min.min(x), max.max(x))
        });

    if min_input < -1.0 || max_input > 1.0 {
        anyhow::bail!(
            "Input values out of range [-1, 1]: min={}, max={}",
            min_input,
            max_input
        );
    }

    // Check target ranges [0, 1]
    let (min_target, max_target) = targets
        .iter()
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), &x| {
            (min.min(x), max.max(x))
        });

    if min_target < 0.0 || max_target > 1.0 {
        anyhow::bail!(
            "Target values out of range [0, 1]: min={}, max={}",
            min_target,
            max_target
        );
    }

    println!("Data validation passed:");
    println!("  Input range: [{min_input:.3}, {max_input:.3}]");
    println!("  Target range: [{min_target:.3}, {max_target:.3}]");

    Ok(())
}

/// Calculate Mean Squared Error loss
fn mse_loss(predictions: &Tensor, targets: &Tensor) -> Result<Tensor> {
    let diff = (predictions - targets)?;
    let squared = diff.sqr()?;
    let mean = squared.mean_all()?;
    Ok(mean)
}

/// Train the model using SGD optimizer
fn train_model(
    model: &DemoMLP,
    input_tensor: &Tensor,
    target_tensor: &Tensor,
    epochs: usize,
    learning_rate: f64,
    varmap: &VarMap,
) -> Result<()> {
    println!("\nStarting model training...");
    println!("Configuration:");
    println!("  Epochs: {epochs}");
    println!("  Learning rate: {learning_rate}");
    println!("  Training samples: {}", input_tensor.dim(0)?);

    // Create optimizer
    let params = ParamsSGD {
        lr: learning_rate,
        ..Default::default()
    };
    let mut optimizer = SGD::new(varmap.all_vars(), params)?;

    // Calculate initial loss
    let initial_predictions = model.forward(input_tensor)?;
    let initial_loss = mse_loss(&initial_predictions, target_tensor)?;
    let initial_loss_value = initial_loss.to_scalar::<f32>()?;
    println!("\nInitial loss: {initial_loss_value:.6}");

    let training_start = Instant::now();

    // Training loop
    for epoch in 1..=epochs {
        // Forward pass
        let predictions = model.forward(input_tensor)?;

        // Calculate loss
        let loss = mse_loss(&predictions, target_tensor)?;

        // Backward pass
        optimizer.backward_step(&loss)?;

        // Log progress
        if epoch % (epochs / 10).max(1) == 0 || epoch == epochs {
            let loss_value = loss.to_scalar::<f32>()?;
            let elapsed = training_start.elapsed();
            println!(
                "Epoch {:3}/{}: Loss = {:.6} (elapsed: {:.1}s)",
                epoch,
                epochs,
                loss_value,
                elapsed.as_secs_f32()
            );
        }
    }

    // Final evaluation
    let final_predictions = model.forward(input_tensor)?;
    let final_loss = mse_loss(&final_predictions, target_tensor)?;
    let final_loss_value = final_loss.to_scalar::<f32>()?;
    let total_time = training_start.elapsed();

    println!("\nTraining completed!");
    println!("  Initial loss: {initial_loss_value:.6}");
    println!("  Final loss: {final_loss_value:.6}");
    println!(
        "  Loss reduction: {:.2}%",
        ((initial_loss_value - final_loss_value) / initial_loss_value * 100.0)
    );
    println!("  Total training time: {:.1}s", total_time.as_secs_f32());

    Ok(())
}

fn main() -> Result<()> {
    println!("AI Demo Training - Full Training Pipeline");
    println!("========================================");

    // Training configuration
    let data_size = 1000;
    let epochs = 10000;
    let learning_rate = 0.01;

    // Generate training data
    println!("\n1. Generating training data...");
    let (input_tensor, target_tensor) = generate_training_data(data_size, None)?;

    // Create model
    println!("\n2. Initializing model...");
    let device = Device::Cpu;
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    let model = DemoMLP::new_demo(vb)?;

    println!("Model architecture: 2 → 4 → 1 (input → hidden → output)");
    println!("Model parameters: {}", varmap.all_vars().len());

    // Train the model
    println!("\n3. Training model...");
    train_model(
        &model,
        &input_tensor,
        &target_tensor,
        epochs,
        learning_rate,
        &varmap,
    )?;

    // Test inference on a few examples
    println!("\n4. Testing model inference...");
    let test_inputs = [[0.5f32, 0.3f32], [-0.8f32, 0.2f32], [0.0f32, 0.0f32]];

    for (i, &[x1, x2]) in test_inputs.iter().enumerate() {
        let test_tensor = Tensor::from_vec(vec![x1, x2], (1, 2), &device)?;
        let prediction = model.forward(&test_tensor)?;
        let predicted_value = prediction.i(0)?.i(0)?.to_scalar::<f32>()?;
        let expected_value = (x1 + x2).tanh() * 0.5 + 0.5;

        println!(
            "  Test {}: input=({:.1}, {:.1}) → predicted={:.4}, expected={:.4}, error={:.4}",
            i + 1,
            x1,
            x2,
            predicted_value,
            expected_value,
            (predicted_value - expected_value).abs()
        );
    }

    // Save the trained model
    println!("\n5. Saving trained model...");
    let model_path = "models/demo_model";

    // Ensure models directory exists
    std::fs::create_dir_all("models")?;

    // Save model using shared library function
    save_model_from_varmap(&varmap, &model.metadata, model_path)?;

    // Verify saved files
    let toml_path = format!("{model_path}.toml");
    let safetensors_path = format!("{model_path}.safetensors");

    if std::path::Path::new(&toml_path).exists() && std::path::Path::new(&safetensors_path).exists()
    {
        let toml_size = std::fs::metadata(&toml_path)?.len();
        let safetensors_size = std::fs::metadata(&safetensors_path)?.len();

        println!("Model saved successfully:");
        println!("  Metadata: {toml_path} ({toml_size} bytes)");
        println!("  Weights: {safetensors_path} ({safetensors_size} bytes)");

        // Copy model files to interactive/src/models directory for embedded assets
        println!("\n6. Copying model files for WASM embedded assets...");
        let interactive_models_dir = "interactive/src/models";
        std::fs::create_dir_all(interactive_models_dir)?;

        let interactive_toml = format!("{interactive_models_dir}/demo_model.toml");
        let interactive_safetensors = format!("{interactive_models_dir}/demo_model.safetensors");

        std::fs::copy(&toml_path, &interactive_toml)?;
        std::fs::copy(&safetensors_path, &interactive_safetensors)?;

        println!("Files copied for embedded assets:");
        println!("  Metadata: {interactive_toml}");
        println!("  Weights: {interactive_safetensors}");
    } else {
        anyhow::bail!("Model files were not created successfully");
    }

    println!("\n✅ Training pipeline completed successfully!");
    println!("   Model is ready for use in the interactive demo.");

    Ok(())
}
