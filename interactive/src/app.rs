// Shared application state and setup

use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    Ready,
    Error,
}

/// Initialize the application with basic setup
pub fn setup_application(mut commands: Commands) {
    info!("AI Demo application starting...");

    // Spawn a camera for the UI
    commands.spawn(Camera2d);

    info!("Basic Bevy application setup complete");
}
