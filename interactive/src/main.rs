// Interactive demo binary for AI demo scaffolding
// Bevy-based UI application for model inference demonstration

use bevy::{
    asset::{embedded_asset, AssetLoader, AssetServer},
    log::LogPlugin,
    prelude::*,
    window::{WindowPlugin, WindowResolution},
};
use shared::{DemoMLP, Device, load_model_from_data};
use anyhow::Result as AnyhowResult;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Loading,
    Ready,
    Error,
}

#[derive(Resource)]
struct LoadedModel {
    #[allow(dead_code)] // Model is stored but not currently used in demo
    model: DemoMLP,
}

#[derive(Resource)]
struct LoadingError {
    message: String,
}

// Resource to track loading assets
#[derive(Resource)]
struct ModelAssets {
    toml_handle: Handle<BinaryAsset>,
    safetensors_handle: Handle<BinaryAsset>,
}

// Custom asset type for binary data
#[derive(Asset, TypePath)]
struct BinaryAsset {
    pub data: Vec<u8>,
}

// Asset loader for binary files
#[derive(Default)]
struct BinaryAssetLoader;

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

struct EmbeddedAssetsPlugin;

impl Plugin for EmbeddedAssetsPlugin {
    fn build(&self, app: &mut App) {
        // Embed the model files
        embedded_asset!(app, "models/demo_model.toml");
        embedded_asset!(app, "models/demo_model.safetensors");
        
        // Register the binary asset loader
        app.init_asset::<BinaryAsset>()
           .init_asset_loader::<BinaryAssetLoader>();
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "AI Demo - Minimal Scaffolding".into(),
                        resolution: WindowResolution::new(800.0, 600.0),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".into(),
                    ..default()
                }),
            EmbeddedAssetsPlugin,
        ))
        .insert_state(AppState::Loading)
        .add_systems(Startup, setup_application)
        .add_systems(OnEnter(AppState::Loading), start_loading_assets)
        .add_systems(Update, check_asset_loading.run_if(in_state(AppState::Loading)))
        .add_systems(OnEnter(AppState::Ready), on_ready_system)
        .add_systems(OnEnter(AppState::Error), on_error_system)
        .run();
}

/// Initialize the application with basic setup
fn setup_application(mut commands: Commands) {
    info!("AI Demo application starting...");
    
    // Spawn a camera for the UI
    commands.spawn(Camera2d);
    
    info!("Basic Bevy application setup complete");
}

/// Start loading the embedded model assets
fn start_loading_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading embedded model assets...");
    
    // Load the embedded assets
    let toml_handle = asset_server.load("embedded://interactive/models/demo_model.toml");
    let safetensors_handle = asset_server.load("embedded://interactive/models/demo_model.safetensors");
    
    commands.insert_resource(ModelAssets {
        toml_handle,
        safetensors_handle,
    });
}

/// Check if assets are loaded and process them
fn check_asset_loading(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    model_assets: Res<ModelAssets>,
    toml_assets: Res<Assets<BinaryAsset>>,
    binary_assets: Res<Assets<BinaryAsset>>,
    asset_server: Res<AssetServer>,
) {
    // Check if both assets are loaded
    let toml_loaded = asset_server.is_loaded_with_dependencies(&model_assets.toml_handle);
    let safetensors_loaded = asset_server.is_loaded_with_dependencies(&model_assets.safetensors_handle);
    
    if toml_loaded && safetensors_loaded {
        // Get the loaded assets
        let toml_asset = toml_assets.get(&model_assets.toml_handle);
        let safetensors_asset = binary_assets.get(&model_assets.safetensors_handle);
        
        match (toml_asset, safetensors_asset) {
            (Some(toml_data), Some(safetensors_data)) => {
                match load_model_from_assets(toml_data, safetensors_data) {
                    Ok(model) => {
                        info!("Model loaded successfully from embedded assets");
                        commands.insert_resource(LoadedModel { model });
                        next_state.set(AppState::Ready);
                    }
                    Err(error) => {
                        let error_message = format!("Failed to load model from assets: {}", error);
                        error!("{}", error_message);
                        commands.insert_resource(LoadingError { message: error_message });
                        next_state.set(AppState::Error);
                    }
                }
            }
            _ => {
                // Assets not available - check for loading errors
                if let Some(load_state) = asset_server.get_load_state(&model_assets.toml_handle) {
                    if matches!(load_state, bevy::asset::LoadState::Failed(_)) {
                        let error_message = "Failed to load TOML asset".to_string();
                        error!("{}", error_message);
                        commands.insert_resource(LoadingError { message: error_message });
                        next_state.set(AppState::Error);
                        return;
                    }
                }
                
                if let Some(load_state) = asset_server.get_load_state(&model_assets.safetensors_handle) {
                    if matches!(load_state, bevy::asset::LoadState::Failed(_)) {
                        let error_message = "Failed to load safetensors asset".to_string();
                        error!("{}", error_message);
                        commands.insert_resource(LoadingError { message: error_message });
                        next_state.set(AppState::Error);
                        return;
                    }
                }
            }
        }
    }
}

/// Load the model from the loaded binary assets using shared persistence functions
fn load_model_from_assets(toml_asset: &BinaryAsset, safetensors_asset: &BinaryAsset) -> AnyhowResult<DemoMLP> {
    info!("Loading model from embedded assets...");
    info!("TOML data: {} bytes", toml_asset.data.len());
    info!("Safetensors data: {} bytes", safetensors_asset.data.len());
    
    // Create device  
    let device = Device::Cpu;
    
    // Use the new WASM-compatible loading function from shared crate
    load_model_from_data(&toml_asset.data, &safetensors_asset.data, &device)
}

/// System that runs when the application is ready
fn on_ready_system() {
    info!("Application ready - model loaded successfully");
}

/// System that runs when there is a loading error
fn on_error_system(error: Res<LoadingError>) {
    error!("Application error state: {}", error.message);
}