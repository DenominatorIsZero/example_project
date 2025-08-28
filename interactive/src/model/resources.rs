// Model resources

use bevy::prelude::*;
use shared::DemoMLP;

#[derive(Resource)]
pub struct LoadedModel {
    #[allow(dead_code)] // Model is stored but not currently used in demo
    pub model: DemoMLP,
}

#[derive(Resource)]
pub struct LoadingError {
    pub message: String,
}
