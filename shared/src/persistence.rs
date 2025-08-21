use anyhow::{Context, Result, anyhow};
use candle_core::{DType, Device};
use candle_nn::{VarBuilder, VarMap};
use std::path::Path;

use crate::model::{DemoMLP, ModelMetadata};

/// Save a DemoMLP model using two-file approach: .toml metadata + .safetensors weights
///
/// This saves two files:
/// - `{base_path}.toml` - model metadata in human-readable TOML format
/// - `{base_path}.safetensors` - model weights in safetensors format
///
/// # Arguments
///
/// * `varmap` - VarMap containing the trained model parameters
/// * `metadata` - Model architecture specification
/// * `base_path` - File path prefix (without extension)
///
/// # Example
///
/// ```rust
/// use shared::{save_model_from_varmap, ModelMetadata, DemoMLP};
/// use candle_core::Device;
/// use candle_nn::{VarBuilder, VarMap};
///
/// # fn main() -> anyhow::Result<()> {
/// let device = Device::Cpu;
/// let varmap = VarMap::new();
/// let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
/// let input_size = 2;
/// let output_size = 1;
/// let hidden_size = 4;
/// let metadata = ModelMetadata::new(input_size, output_size, hidden_size)?;
/// let _model = DemoMLP::new(metadata.clone(), vb)?;
///
/// // After training...
/// # let temp_dir = tempfile::tempdir()?;
/// # let model_path = temp_dir.path().join("trained_model");
/// # let model_path_str = model_path.to_str().unwrap();
/// save_model_from_varmap(&varmap, &metadata, model_path_str)?;
/// // Creates: trained_model.toml and trained_model.safetensors
/// # Ok(())
/// # }
/// ```
pub fn save_model_from_varmap(
    varmap: &VarMap,
    metadata: &ModelMetadata,
    base_path: &str,
) -> Result<()> {
    let toml_path = format!("{base_path}.toml");
    let safetensors_path = format!("{base_path}.safetensors");

    // Save metadata as TOML file
    let toml_content = toml::to_string(metadata).context("Failed to serialize metadata to TOML")?;

    std::fs::write(&toml_path, toml_content)
        .with_context(|| format!("Failed to write metadata file: {toml_path}"))?;

    // Save model weights as safetensors
    varmap
        .save(&safetensors_path)
        .with_context(|| format!("Failed to save model weights: {safetensors_path}"))?;

    Ok(())
}

/// Load a DemoMLP model using two-file approach: read .toml metadata + .safetensors weights
///
/// This expects two files:
/// - `{base_path}.toml` - model metadata
/// - `{base_path}.safetensors` - model weights
///
/// The function automatically reads the metadata from the TOML file and uses it
/// to reconstruct the model architecture, then loads the weights using memory-mapped
/// safetensors for efficient loading.
///
/// # Arguments
///
/// * `base_path` - File path prefix (without extension)  
/// * `device` - Device to load the model onto (CPU or CUDA)
///
/// # Returns
///
/// A fully reconstructed `DemoMLP` model ready for inference
///
/// # Example
///
/// ```rust
/// use shared::{load_model, save_model_from_varmap, ModelMetadata, DemoMLP};
/// use candle_core::{Device, Tensor};
/// use candle_nn::{VarBuilder, VarMap};
///
/// # fn main() -> anyhow::Result<()> {
/// let device = Device::Cpu;
///
/// // First create and save a model (normally done during training)
/// let varmap = VarMap::new();
/// let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
/// let input_size = 2;
/// let output_size = 1;
/// let hidden_size = 4;
/// let metadata = ModelMetadata::new(input_size, output_size, hidden_size)?;
/// let _model = DemoMLP::new(metadata.clone(), vb)?;
/// # let temp_dir = tempfile::tempdir()?;
/// # let model_path = temp_dir.path().join("trained_model");
/// # let model_path_str = model_path.to_str().unwrap();
/// save_model_from_varmap(&varmap, &metadata, model_path_str)?;
///
/// // Load model (reads .toml and .safetensors files)
/// let model = load_model(model_path_str, &device)?;
///
/// // Use for inference
/// let input = Tensor::from_vec(vec![0.5f32, -0.3f32], (1, 2), &device)?;
/// let output = model.forward(&input)?;
/// # Ok(())
/// # }
/// ```
pub fn load_model(base_path: &str, device: &Device) -> Result<DemoMLP> {
    let toml_path = format!("{base_path}.toml");
    let safetensors_path = format!("{base_path}.safetensors");

    // Verify both files exist
    if !Path::new(&toml_path).exists() {
        return Err(anyhow!("Model metadata file does not exist: {}", toml_path));
    }
    if !Path::new(&safetensors_path).exists() {
        return Err(anyhow!(
            "Model weights file does not exist: {}",
            safetensors_path
        ));
    }

    // Load metadata from TOML file
    let toml_content = std::fs::read_to_string(&toml_path)
        .with_context(|| format!("Failed to read metadata file: {toml_path}"))?;

    let metadata: ModelMetadata = toml::from_str(&toml_content)
        .with_context(|| format!("Failed to parse metadata from: {toml_path}"))?;

    // Load model weights using elegant VarBuilder approach
    let vb = unsafe {
        VarBuilder::from_mmaped_safetensors(&[safetensors_path], DType::F32, device)
            .context("Failed to create VarBuilder from safetensors file")?
    };

    // Create model once with loaded parameters and metadata
    let model = DemoMLP::new(metadata, vb)?;

    Ok(model)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::DemoMLP;
    use candle_core::Tensor;
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_model() -> Result<()> {
        let device = Device::Cpu;

        // Create original model with VarMap for training
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
        let metadata = ModelMetadata::new(2, 1, 4)?;
        let _original_model = DemoMLP::new(metadata.clone(), vb)?;

        // Save model using two-file approach
        let temp_dir = tempdir()?;
        let base_path = temp_dir.path().join("test_model");
        let base_path_str = base_path.to_str().unwrap();
        save_model_from_varmap(&varmap, &metadata, base_path_str)?;

        // Verify both files were created
        assert!(Path::new(&format!("{base_path_str}.toml")).exists());
        assert!(Path::new(&format!("{base_path_str}.safetensors")).exists());

        // Load model back
        let loaded_model = load_model(base_path_str, &device)?;

        // Verify metadata is preserved
        assert_eq!(loaded_model.metadata.input_size, 2);
        assert_eq!(loaded_model.metadata.output_size, 1);
        assert_eq!(loaded_model.metadata.hidden_size, 4);

        Ok(())
    }

    #[test]
    fn test_load_model_nonexistent_file() {
        let device = Device::Cpu;
        let result = load_model("/nonexistent/path", &device);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_round_trip_inference() -> Result<()> {
        let device = Device::Cpu;

        // Create original model
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
        let metadata = ModelMetadata::new(2, 1, 4)?;
        let original_model = DemoMLP::new(metadata.clone(), vb)?;

        // Test input
        let input = Tensor::from_vec(vec![0.5f32, -0.3f32], (1, 2), &device)?;
        let original_output = original_model.forward(&input)?;

        // Save and reload model
        let temp_dir = tempdir()?;
        let base_path = temp_dir.path().join("test_model");
        let base_path_str = base_path.to_str().unwrap();
        save_model_from_varmap(&varmap, &metadata, base_path_str)?;

        let loaded_model = load_model(base_path_str, &device)?;
        let loaded_output = loaded_model.forward(&input)?;

        // Outputs should be identical (same initialized weights)
        assert_eq!(original_output.shape(), loaded_output.shape());

        // Verify output is in valid range [0, 1] for sigmoid
        let output_value = loaded_output.to_vec2::<f32>()?[0][0];
        assert!((0.0..=1.0).contains(&output_value));

        Ok(())
    }
}
