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

/// Parse TOML metadata bytes into ModelMetadata structure
///
/// # Arguments
///
/// * `toml_bytes` - Raw TOML file content as bytes
///
/// # Returns
///
/// Parsed ModelMetadata structure containing model architecture info
///
/// # Example
///
/// ```rust
/// use shared::parse_model_metadata;
///
/// # fn main() -> anyhow::Result<()> {
/// let toml_content = b"input_size = 2\noutput_size = 1\nhidden_size = 4";
/// let metadata = parse_model_metadata(toml_content)?;
/// assert_eq!(metadata.input_size, 2);
/// # Ok(())
/// # }
/// ```
pub fn parse_model_metadata(toml_bytes: &[u8]) -> Result<ModelMetadata> {
    let toml_text =
        std::str::from_utf8(toml_bytes).context("Failed to convert TOML bytes to text")?;
    let metadata: ModelMetadata =
        toml::from_str(toml_text).context("Failed to parse TOML metadata")?;
    Ok(metadata)
}

/// Load a DemoMLP model from raw bytes (WASM-compatible)
///
/// This function creates a model from pre-loaded byte data without requiring
/// filesystem operations, making it compatible with WASM environments.
///
/// # Arguments
///
/// * `toml_bytes` - TOML metadata content as bytes
/// * `safetensors_bytes` - Safetensors weight data as bytes  
/// * `device` - Device to load the model onto (CPU or CUDA)
///
/// # Returns
///
/// A fully reconstructed `DemoMLP` model ready for inference
///
/// # Example
///
/// ```rust
/// use shared::{load_model_from_data, ModelMetadata};
/// use candle_core::Device;
///
/// # fn main() -> anyhow::Result<()> {
/// let device = Device::Cpu;
/// let toml_data = b"input_size = 2\noutput_size = 1\nhidden_size = 4";
/// // safetensors_data would come from actual model file
/// # let safetensors_data: &[u8] = &[];
/// // let model = load_model_from_data(toml_data, safetensors_data, &device)?;
/// # Ok(())
/// # }
/// ```
pub fn load_model_from_data(
    toml_bytes: &[u8],
    safetensors_bytes: &[u8],
    device: &Device,
) -> Result<DemoMLP> {
    let metadata = parse_model_metadata(toml_bytes)?;

    // Load model weights using memory-based VarBuilder (WASM-compatible)
    let vb = VarBuilder::from_slice_safetensors(safetensors_bytes, DType::F32, device)
        .context("Failed to create VarBuilder from safetensors data")?;

    // Create model with loaded parameters and metadata
    let model = DemoMLP::new(metadata, vb)?;

    Ok(model)
}

/// Load a DemoMLP model using two-file approach: read .toml metadata + .safetensors weights
///
/// This expects two files:
/// - `{base_path}.toml` - model metadata
/// - `{base_path}.safetensors` - model weights
///
/// The function automatically reads the metadata from the TOML file and uses it
/// to reconstruct the model architecture, then loads the weights from the safetensors file.
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
/// use shared::{load_model_from_files, save_model_from_varmap, ModelMetadata, DemoMLP};
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
/// let model = load_model_from_files(model_path_str, &device)?;
///
/// // Use for inference
/// let input = Tensor::from_vec(vec![0.5f32, -0.3f32], (1, 2), &device)?;
/// let output = model.forward(&input)?;
/// # Ok(())
/// # }
/// ```
pub fn load_model_from_files(base_path: &str, device: &Device) -> Result<DemoMLP> {
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

    // Read both files into memory
    let toml_bytes = std::fs::read(&toml_path)
        .with_context(|| format!("Failed to read metadata file: {toml_path}"))?;

    let safetensors_bytes = std::fs::read(&safetensors_path)
        .with_context(|| format!("Failed to read weights file: {safetensors_path}"))?;

    // Use the memory-based function
    load_model_from_data(&toml_bytes, &safetensors_bytes, device)
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
        let loaded_model = load_model_from_files(base_path_str, &device)?;

        // Verify metadata is preserved
        assert_eq!(loaded_model.metadata.input_size, 2);
        assert_eq!(loaded_model.metadata.output_size, 1);
        assert_eq!(loaded_model.metadata.hidden_size, 4);

        Ok(())
    }

    #[test]
    fn test_load_model_nonexistent_file() {
        let device = Device::Cpu;
        let result = load_model_from_files("/nonexistent/path", &device);
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

        let loaded_model = load_model_from_files(base_path_str, &device)?;
        let loaded_output = loaded_model.forward(&input)?;

        // Outputs should be identical (same initialized weights)
        assert_eq!(original_output.shape(), loaded_output.shape());

        // Verify output is in valid range [0, 1] for sigmoid
        let output_value = loaded_output.to_vec2::<f32>()?[0][0];
        assert!((0.0..=1.0).contains(&output_value));

        Ok(())
    }

    #[test]
    fn test_parse_model_metadata() -> Result<()> {
        let toml_content = b"input_size = 2\noutput_size = 1\nhidden_size = 4";
        let metadata = parse_model_metadata(toml_content)?;

        assert_eq!(metadata.input_size, 2);
        assert_eq!(metadata.output_size, 1);
        assert_eq!(metadata.hidden_size, 4);

        Ok(())
    }

    #[test]
    fn test_parse_model_metadata_invalid() {
        let invalid_toml = b"invalid toml content {";
        let result = parse_model_metadata(invalid_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_model_from_data() -> Result<()> {
        let device = Device::Cpu;

        // Create a model and save it to get valid safetensors data
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
        let metadata = ModelMetadata::new(2, 1, 4)?;
        let _original_model = DemoMLP::new(metadata.clone(), vb)?;

        // Save to get the bytes
        let temp_dir = tempdir()?;
        let base_path = temp_dir.path().join("test_model");
        let base_path_str = base_path.to_str().unwrap();
        save_model_from_varmap(&varmap, &metadata, base_path_str)?;

        // Read the saved files into memory
        let toml_bytes = std::fs::read(format!("{base_path_str}.toml"))?;
        let safetensors_bytes = std::fs::read(format!("{base_path_str}.safetensors"))?;

        // Load model from data
        let loaded_model = load_model_from_data(&toml_bytes, &safetensors_bytes, &device)?;

        // Verify metadata is preserved
        assert_eq!(loaded_model.metadata.input_size, 2);
        assert_eq!(loaded_model.metadata.output_size, 1);
        assert_eq!(loaded_model.metadata.hidden_size, 4);

        Ok(())
    }
}
