// Training binary for AI demo scaffolding
// Generates synthetic training data for the demo MLP model

use anyhow::Result;
use rand::prelude::*;
use shared::{Device, Tensor};

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
        let input1: f32 = rng.gen_range(-1.0..=1.0);
        let input2: f32 = rng.gen_range(-1.0..=1.0);
        
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
    println!("Generated {} training samples", size);
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
    let (min_input, max_input) = inputs.iter().fold((f32::INFINITY, f32::NEG_INFINITY), 
        |(min, max), &x| (min.min(x), max.max(x)));
    
    if min_input < -1.0 || max_input > 1.0 {
        anyhow::bail!("Input values out of range [-1, 1]: min={}, max={}", min_input, max_input);
    }
    
    // Check target ranges [0, 1]
    let (min_target, max_target) = targets.iter().fold((f32::INFINITY, f32::NEG_INFINITY),
        |(min, max), &x| (min.min(x), max.max(x)));
        
    if min_target < 0.0 || max_target > 1.0 {
        anyhow::bail!("Target values out of range [0, 1]: min={}, max={}", min_target, max_target);
    }
    
    println!("Data validation passed:");
    println!("  Input range: [{:.3}, {:.3}]", min_input, max_input);
    println!("  Target range: [{:.3}, {:.3}]", min_target, max_target);
    
    Ok(())
}

fn main() -> Result<()> {
    println!("AI Demo Training - Synthetic Data Generation");
    println!("============================================");
    
    // Generate training data
    let data_size = 1000;
    let (_input_tensor, _target_tensor) = generate_training_data(data_size, None)?;
    
    println!("\nSynthetic data generation completed successfully!");
    println!("Ready for model training implementation in next phase.");
    
    Ok(())
}