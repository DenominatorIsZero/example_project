// Model module exports

pub mod assets;
pub mod loader;
pub mod resources;
pub mod systems;

// Re-export commonly used types
pub use assets::*;
pub use loader::EmbeddedAssetsPlugin;
pub use resources::*;
pub use systems::*;
