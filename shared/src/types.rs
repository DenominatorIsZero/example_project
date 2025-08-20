/// Model metadata for AI demo scaffolding
use anyhow::{anyhow, Result};
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
        let metadata = Self { input_size, output_size, hidden_size };
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
            return Err(anyhow!("Input size {} exceeds maximum {}", self.input_size, MAX_SIZE));
        }
        if self.output_size > MAX_SIZE {
            return Err(anyhow!("Output size {} exceeds maximum {}", self.output_size, MAX_SIZE));
        }
        if self.hidden_size > MAX_SIZE {
            return Err(anyhow!("Hidden size {} exceeds maximum {}", self.hidden_size, MAX_SIZE));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_metadata() {
        let metadata = ModelMetadata::new(2, 1, 4).unwrap();
        assert_eq!(metadata.input_size, 2);
        assert_eq!(metadata.output_size, 1);
        assert_eq!(metadata.hidden_size, 4);
    }

    #[test]
    fn test_model_metadata_validation() {
        // Valid cases
        assert!(ModelMetadata::new(2, 1, 4).is_ok());
        
        // Zero sizes should fail
        assert!(ModelMetadata::new(0, 1, 4).is_err());
        assert!(ModelMetadata::new(2, 0, 4).is_err());
        assert!(ModelMetadata::new(2, 1, 0).is_err());
        
        // Too large sizes should fail
        assert!(ModelMetadata::new(20000, 1, 4).is_err());
        assert!(ModelMetadata::new(2, 20000, 4).is_err());
        assert!(ModelMetadata::new(2, 1, 20000).is_err());
    }

    #[test]
    fn test_serialization() {
        let metadata = ModelMetadata::new(2, 1, 4).unwrap();
        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: ModelMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(metadata, deserialized);
    }
}
