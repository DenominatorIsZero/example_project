// Interactive demo binary for AI demo scaffolding
// Bevy-based UI application for model inference demonstration

use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{WindowPlugin, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins(
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
        )
        .add_systems(Startup, setup_application)
        .run();
}

/// Initialize the application with basic setup
fn setup_application(mut commands: Commands) {
    info!("AI Demo application starting...");
    
    // Spawn a camera for the UI
    commands.spawn(Camera2d);
    
    info!("Basic Bevy application setup complete");
}
