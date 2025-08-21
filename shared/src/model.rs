use anyhow::{Result, anyhow};
use candle_core::{Module, Tensor};
use candle_nn::ops::sigmoid;
use candle_nn::{Linear, VarBuilder};
use serde::{Deserialize, Serialize};

/// Metadata describing the model architecture
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub input_size: usize,
    pub output_size: usize,
    pub hidden_size: usize,
}

impl ModelMetadata {
    pub fn new(input_size: usize, output_size: usize, hidden_size: usize) -> Result<Self> {
        let metadata = Self {
            input_size,
            output_size,
            hidden_size,
        };
        metadata.validate()?;
        Ok(metadata)
    }

    pub fn validate(&self) -> Result<()> {
        if self.input_size == 0 {
            return Err(anyhow!("Input size cannot be zero"));
        }
        if self.output_size == 0 {
            return Err(anyhow!("Output size cannot be zero"));
        }
        if self.hidden_size == 0 {
            return Err(anyhow!("Hidden size cannot be zero"));
        }

        // Reasonable upper bounds to catch obvious errors
        const MAX_SIZE: usize = 10_000;
        if self.input_size > MAX_SIZE {
            return Err(anyhow!(
                "Input size {} exceeds maximum {}",
                self.input_size,
                MAX_SIZE
            ));
        }
        if self.output_size > MAX_SIZE {
            return Err(anyhow!(
                "Output size {} exceeds maximum {}",
                self.output_size,
                MAX_SIZE
            ));
        }
        if self.hidden_size > MAX_SIZE {
            return Err(anyhow!(
                "Hidden size {} exceeds maximum {}",
                self.hidden_size,
                MAX_SIZE
            ));
        }

        Ok(())
    }
}

/// Simple Multi-Layer Perceptron for AI demo scaffolding
/// Architecture is defined by the metadata parameter
#[derive(Debug)]
pub struct DemoMLP {
    pub fc1: Linear,
    pub fc2: Linear,
    pub metadata: ModelMetadata,
}

impl DemoMLP {
    /// Create a new DemoMLP with the specified metadata and VarBuilder
    pub fn new(metadata: ModelMetadata, vb: VarBuilder) -> Result<Self> {
        let fc1 = candle_nn::linear(metadata.input_size, metadata.hidden_size, vb.pp("fc1"))?;
        let fc2 = candle_nn::linear(metadata.hidden_size, metadata.output_size, vb.pp("fc2"))?;

        Ok(Self { fc1, fc2, metadata })
    }

    /// Create a DemoMLP with default demo architecture (2→4→1)
    pub fn new_demo(vb: VarBuilder) -> Result<Self> {
        let metadata = ModelMetadata::new(2, 1, 4)?;
        Self::new(metadata, vb)
    }

    /// Forward pass through the network
    /// Input shape: [batch_size, input_size]
    /// Output shape: [batch_size, output_size]
    pub fn forward(&self, input: &Tensor) -> Result<Tensor> {
        // Validate input shape
        let input_shape = input.shape();
        if input_shape.dims().len() != 2 || input_shape.dims()[1] != self.metadata.input_size {
            return Err(anyhow!(
                "Expected input shape [batch_size, {}], got {:?}",
                self.metadata.input_size,
                input_shape
            ));
        }

        // Forward pass: input → fc1 → ReLU → fc2 → Sigmoid
        let x = self.fc1.forward(input)?;
        let x = x.relu()?;
        let x = self.fc2.forward(&x)?;
        let output = sigmoid(&x)?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;
    use candle_nn::VarMap;

    #[test]
    fn test_model_creation() -> Result<()> {
        let device = Device::Cpu;
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);

        // Test with custom metadata
        let metadata = ModelMetadata::new(2, 1, 4)?;
        let model = DemoMLP::new(metadata.clone(), vb)?;

        // Verify the model stores the metadata correctly
        assert_eq!(model.metadata.input_size, 2);
        assert_eq!(model.metadata.output_size, 1);
        assert_eq!(model.metadata.hidden_size, 4);

        Ok(())
    }

    #[test]
    fn test_demo_model_creation() -> Result<()> {
        let device = Device::Cpu;
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);

        let model = DemoMLP::new_demo(vb)?;

        // Verify demo model has the correct default architecture
        assert_eq!(model.metadata.input_size, 2);
        assert_eq!(model.metadata.output_size, 1);
        assert_eq!(model.metadata.hidden_size, 4);

        Ok(())
    }

    #[test]
    fn test_forward_pass() -> Result<()> {
        let device = Device::Cpu;
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);

        let model = DemoMLP::new_demo(vb)?;

        // Create test input: batch size of 2, input size of 2
        let input = Tensor::from_vec(vec![0.5f32, -0.5f32, 1.0f32, -1.0f32], (2, 2), &device)?;

        let output = model.forward(&input)?;

        // Verify output shape matches metadata
        let output_shape = output.shape();
        assert_eq!(output_shape.dims(), &[2, model.metadata.output_size]);

        // Verify output is in sigmoid range [0, 1]
        let output_vec = output.to_vec2::<f32>()?;
        for batch in output_vec {
            for value in batch {
                assert!(
                    (0.0..=1.0).contains(&value),
                    "Sigmoid output should be in [0,1], got {value}"
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_input_validation() -> Result<()> {
        let device = Device::Cpu;
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);

        let model = DemoMLP::new_demo(vb)?;

        // Test with wrong input size (3 instead of 2)
        let invalid_input = Tensor::from_vec(vec![0.5f32, -0.5f32, 0.0f32], (1, 3), &device)?;

        let result = model.forward(&invalid_input);
        assert!(result.is_err(), "Should fail with invalid input shape");

        // Test with wrong number of dimensions
        let invalid_input_1d = Tensor::from_vec(vec![0.5f32, -0.5f32], 2, &device)?;

        let result = model.forward(&invalid_input_1d);
        assert!(result.is_err(), "Should fail with 1D input");

        Ok(())
    }

    #[test]
    fn test_single_sample_inference() -> Result<()> {
        let device = Device::Cpu;
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);

        let model = DemoMLP::new_demo(vb)?;

        // Test with single sample
        let input = Tensor::from_vec(vec![0.0f32, 0.0f32], (1, 2), &device)?;

        let output = model.forward(&input)?;

        // Verify single output matches metadata
        let output_shape = output.shape();
        assert_eq!(output_shape.dims(), &[1, model.metadata.output_size]);

        // The output should be a valid probability
        let output_value = output.to_vec2::<f32>()?[0][0];
        assert!((0.0..=1.0).contains(&output_value));

        Ok(())
    }

    #[test]
    fn test_flexible_architecture() -> Result<()> {
        let device = Device::Cpu;
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);

        // Test with different architecture (3 inputs, 8 hidden, 2 outputs)
        let metadata = ModelMetadata::new(3, 2, 8)?;
        let model = DemoMLP::new(metadata, vb)?;

        // Test with matching input
        let input = Tensor::from_vec(
            vec![0.1f32, -0.2f32, 0.3f32, -0.4f32, 0.5f32, -0.6f32],
            (2, 3), // batch_size=2, input_size=3
            &device,
        )?;

        let output = model.forward(&input)?;

        // Verify output shape matches custom metadata
        let output_shape = output.shape();
        assert_eq!(output_shape.dims(), &[2, 2]); // batch_size=2, output_size=2

        // Verify all outputs are in sigmoid range [0, 1]
        let output_vec = output.to_vec2::<f32>()?;
        for batch in output_vec {
            for value in batch {
                assert!(
                    (0.0..=1.0).contains(&value),
                    "Sigmoid output should be in [0,1], got {value}"
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_metadata_validation() -> Result<()> {
        // Test metadata validation
        assert!(ModelMetadata::new(0, 1, 4).is_err()); // zero input_size
        assert!(ModelMetadata::new(2, 0, 4).is_err()); // zero output_size  
        assert!(ModelMetadata::new(2, 1, 0).is_err()); // zero hidden_size
        assert!(ModelMetadata::new(20000, 1, 4).is_err()); // too large input_size

        // Valid metadata should work
        let valid_metadata = ModelMetadata::new(3, 2, 8)?;
        assert_eq!(valid_metadata.input_size, 3);
        assert_eq!(valid_metadata.output_size, 2);
        assert_eq!(valid_metadata.hidden_size, 8);

        Ok(())
    }

    #[test]
    fn test_metadata_serialization() -> Result<()> {
        let metadata = ModelMetadata::new(2, 1, 4)?;
        let json = serde_json::to_string(&metadata)?;
        let deserialized: ModelMetadata = serde_json::from_str(&json)?;
        assert_eq!(metadata, deserialized);
        Ok(())
    }
}
