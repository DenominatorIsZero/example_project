// Model asset definitions

use bevy::{asset::AssetLoader, prelude::*};

// Resource to track loading assets
#[derive(Resource)]
pub struct ModelAssets {
    pub toml_handle: Handle<BinaryAsset>,
    pub safetensors_handle: Handle<BinaryAsset>,
}

// Custom asset type for binary data
#[derive(Asset, TypePath)]
pub struct BinaryAsset {
    pub data: Vec<u8>,
}

// Asset loader for binary files
#[derive(Default)]
pub struct BinaryAssetLoader;

impl AssetLoader for BinaryAssetLoader {
    type Asset = BinaryAsset;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(BinaryAsset { data: bytes })
    }

    fn extensions(&self) -> &[&str] {
        &["safetensors", "toml"]
    }
}
