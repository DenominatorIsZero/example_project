// Shared library for AI demo scaffolding
// This will contain common data types, model definitions, and utilities

pub mod model;
pub mod persistence;
pub mod types;

// Re-export key types for convenience (placeholder - will be used once modules are implemented)
#[allow(unused_imports)]
pub use model::*;
#[allow(unused_imports)]
pub use persistence::*;
#[allow(unused_imports)]
pub use types::*;
