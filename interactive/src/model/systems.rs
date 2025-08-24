// Model loading and state management systems

use bevy::prelude::*;
use crate::app::AppState;
use super::{assets::*, loader::*, resources::*};

/// Start loading the embedded model assets
pub fn start_loading_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading embedded model assets...");

    // Load the embedded assets using the original working path
    let toml_handle = asset_server.load("embedded://interactive/models/demo_model.toml");
    let safetensors_handle =
        asset_server.load("embedded://interactive/models/demo_model.safetensors");

    commands.insert_resource(ModelAssets {
        toml_handle,
        safetensors_handle,
    });
}

/// Check if assets are loaded and process them
pub fn check_asset_loading(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    model_assets: Res<ModelAssets>,
    toml_assets: Res<Assets<BinaryAsset>>,
    binary_assets: Res<Assets<BinaryAsset>>,
    asset_server: Res<AssetServer>,
) {
    // Check if both assets are loaded
    let toml_loaded = asset_server.is_loaded_with_dependencies(&model_assets.toml_handle);
    let safetensors_loaded =
        asset_server.is_loaded_with_dependencies(&model_assets.safetensors_handle);

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
                        let error_message = format!("Failed to load model from assets: {error}");
                        error!("{}", error_message);
                        commands.insert_resource(LoadingError {
                            message: error_message,
                        });
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
                        commands.insert_resource(LoadingError {
                            message: error_message,
                        });
                        next_state.set(AppState::Error);
                    }
                }

                if let Some(load_state) =
                    asset_server.get_load_state(&model_assets.safetensors_handle)
                {
                    if matches!(load_state, bevy::asset::LoadState::Failed(_)) {
                        let error_message = "Failed to load safetensors asset".to_string();
                        error!("{}", error_message);
                        commands.insert_resource(LoadingError {
                            message: error_message,
                        });
                        next_state.set(AppState::Error);
                    }
                }
            }
        }
    }
}

/// System that runs when the application is ready
pub fn on_ready_system() {
    info!("Application ready - model loaded successfully");
}

/// System that runs when there is a loading error
pub fn on_error_system(error: Res<LoadingError>) {
    error!("Application error state: {}", error.message);
}