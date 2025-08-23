//! # Shared Library for AI Demo Scaffolding
//!
//! This library provides a complete toolkit for building AI demonstration projects
//! using the Candle machine learning framework. It follows a standardized pattern
//! for model definition, training, and deployment.
//!
//! ## Quick Start
//!
//! ### Creating and Training a Model
//!
//! ```rust
//! use shared::{ModelMetadata, DemoMLP, save_model_from_varmap};
//! use candle_core::Device;
//! use candle_nn::{VarBuilder, VarMap};
//!
//! # fn main() -> anyhow::Result<()> {
//! let device = Device::Cpu;
//! let mut varmap = VarMap::new();
//! let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
//!
//! // Define model architecture
//! let input_size = 2;
//! let output_size = 1; 
//! let hidden_size = 4;
//! let metadata = ModelMetadata::new(input_size, output_size, hidden_size)?;
//! let model = DemoMLP::new(metadata.clone(), vb)?;
//!
//! // ... training loop would go here ...
//!
//! // Save the trained model
//! # let temp_dir = tempfile::tempdir()?;
//! # let model_path = temp_dir.path().join("my_model");
//! # let model_path_str = model_path.to_str().unwrap();
//! save_model_from_varmap(&varmap, &metadata, model_path_str)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Loading and Using a Model
//!
//! ```rust
//! use shared::{load_model_from_files, save_model_from_varmap, ModelMetadata, DemoMLP};
//! use candle_core::{Device, Tensor};
//! use candle_nn::{VarBuilder, VarMap};
//!
//! # fn main() -> anyhow::Result<()> {
//! let device = Device::Cpu;
//!
//! // First create and save a model (normally done during training)
//! let varmap = VarMap::new();
//! let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
//! let input_size = 2;
//! let output_size = 1;
//! let hidden_size = 4;
//! let metadata = ModelMetadata::new(input_size, output_size, hidden_size)?;
//! let _model = DemoMLP::new(metadata.clone(), vb)?;
//! # let temp_dir = tempfile::tempdir()?;
//! # let model_path = temp_dir.path().join("my_model");
//! # let model_path_str = model_path.to_str().unwrap();
//! save_model_from_varmap(&varmap, &metadata, model_path_str)?;
//!
//! // Load model (automatically reads metadata from .toml file)
//! let model = load_model_from_files(model_path_str, &device)?;
//!
//! // Run inference
//! let input = Tensor::from_vec(vec![0.5f32, -0.3f32], (1, 2), &device)?;
//! let output = model.forward(&input)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The library is organized into two main modules:
//!
//! - [`model`] - Model architecture definitions and neural network components
//! - [`persistence`] - Model saving and loading utilities
//!
//! All public APIs are re-exported at the crate root for convenience.

pub mod model;
pub mod persistence;

// Re-export core types and functions
pub use model::{DemoMLP, ModelMetadata};
pub use persistence::{load_model_from_files, load_model_from_data, parse_model_metadata, save_model_from_varmap};

// Re-export commonly needed Candle types for user convenience
pub use candle_core::{DType, Device, Tensor};
pub use candle_nn::{VarBuilder, VarMap};

// Re-export Result type for convenience
pub use anyhow::Result;
