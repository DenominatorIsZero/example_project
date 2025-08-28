// Model loading logic and plugin

use super::assets::{BinaryAsset, BinaryAssetLoader};
use anyhow::Result as AnyhowResult;
use bevy::prelude::*;
use shared::{DemoMLP, Device, load_model_from_data};

pub struct EmbeddedAssetsPlugin;

impl Plugin for EmbeddedAssetsPlugin {
    fn build(&self, app: &mut App) {
        info!("EmbeddedAssetsPlugin: Registering asset loader...");

        // Register the binary asset loader (assets are embedded in main.rs)
        app.init_asset::<BinaryAsset>()
            .init_asset_loader::<BinaryAssetLoader>();

        info!("EmbeddedAssetsPlugin: Plugin setup complete");
    }
}

/// Load the model from the loaded binary assets using shared persistence functions
pub fn load_model_from_assets(
    toml_asset: &BinaryAsset,
    safetensors_asset: &BinaryAsset,
) -> AnyhowResult<DemoMLP> {
    info!("Loading model from embedded assets...");
    info!("TOML data: {} bytes", toml_asset.data.len());
    info!("Safetensors data: {} bytes", safetensors_asset.data.len());

    // Create device
    let device = Device::Cpu;

    // Use the new WASM-compatible loading function from shared crate
    load_model_from_data(&toml_asset.data, &safetensors_asset.data, &device)
}
