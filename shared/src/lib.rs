// Shared library for AI demo scaffolding
// This will contain common data types, model definitions, and utilities

pub mod types;
pub mod model; 
pub mod persistence;

// Re-export key types for convenience
pub use types::*;
pub use model::*;
pub use persistence::*;