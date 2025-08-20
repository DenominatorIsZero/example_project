// Shared library for AI demo scaffolding
// This will contain model definitions, persistence utilities, and other shared components

pub mod model;
pub mod persistence;

// Re-export key types for convenience
#[allow(unused_imports)]
pub use model::*;
#[allow(unused_imports)]
pub use persistence::*;
