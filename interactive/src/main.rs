// Interactive demo binary for AI demo scaffolding
// Bevy-based UI application for model inference demonstration

use bevy::{
    asset::embedded_asset,
    log::LogPlugin,
    prelude::*,
    window::{WindowPlugin, WindowResolution},
};
use interactive::{
    AppState, setup_application, setup_ui, EmbeddedAssetsPlugin,
    start_loading_assets, check_asset_loading, on_ready_system, on_error_system,
    update_button_interactions, update_ui_for_ready, update_ui_for_error, 
    update_status_display,
};

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins
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
                }));
    
    // Embed assets after DefaultPlugins are added
    embedded_asset!(app, "models/demo_model.toml");
    embedded_asset!(app, "models/demo_model.safetensors");
    
    app.add_plugins(EmbeddedAssetsPlugin)
        .insert_state(AppState::Loading)
        .add_systems(Startup, (setup_application, setup_ui))
        .add_systems(OnEnter(AppState::Loading), start_loading_assets)
        .add_systems(
            Update,
            check_asset_loading.run_if(in_state(AppState::Loading)),
        )
        .add_systems(
            OnEnter(AppState::Ready),
            (on_ready_system, update_ui_for_ready),
        )
        .add_systems(
            OnEnter(AppState::Error),
            (on_error_system, update_ui_for_error),
        )
        .add_systems(Update, (update_button_interactions, update_status_display))
        .run();
}