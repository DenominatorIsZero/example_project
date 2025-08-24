// Interactive demo binary for AI demo scaffolding
// Bevy-based UI application for model inference demonstration

use anyhow::Result as AnyhowResult;
use bevy::{
    asset::{AssetLoader, AssetServer, embedded_asset},
    log::LogPlugin,
    prelude::*,
    window::{WindowPlugin, WindowResolution},
};
use shared::{DemoMLP, Device, load_model_from_data};

// Website-matching color palette
pub const BACKGROUND_COLOR: Color = Color::srgb(0.374, 0.374, 0.374); // gray-700: #5f5f5f
pub const TEXT_COLOR: Color = Color::WHITE;
pub const GREEN_PRIMARY: Color = Color::srgb(0.133, 0.698, 0.298); // green-500: #22c55e  
pub const GREEN_HOVER: Color = Color::srgb(0.251, 0.831, 0.412); // green-400: #4ade80
pub const GRAY_SECONDARY: Color = Color::srgb(0.282, 0.282, 0.282); // gray-600: #484848
pub const YELLOW_ACCENT: Color = Color::srgb(0.918, 0.784, 0.157); // yellow-500: #eab308

// UI Component markers
#[derive(Component)]
pub struct MainContainer;

#[derive(Component)]
pub struct TitleText;

#[derive(Component)]
pub struct StatusDisplay;

#[derive(Component)]
pub struct InputField {
    pub field_id: usize,
    pub placeholder: String,
}

#[derive(Component)]
pub struct PredictButton;

#[derive(Component)]
pub struct OutputDisplay;

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

/// Initialize the application with basic setup
fn setup_application(mut commands: Commands) {
    info!("AI Demo application starting...");

    // Spawn a camera for the UI
    commands.spawn(Camera2d);

    info!("Basic Bevy application setup complete");
}

/// Set up the main UI layout
fn setup_ui(mut commands: Commands) {
    // Main container - full screen with dark background
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BACKGROUND_COLOR),
            MainContainer,
        ))
        .with_children(|parent| {
            // Central content box
            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(500.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(GRAY_SECONDARY),
                    BorderColor(GREEN_PRIMARY),
                ))
                .with_children(|content| {
                    // Title
                    content.spawn((
                        Text::new("Minimal AI Demo"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        TitleText,
                    ));

                    // Model Status
                    content.spawn((
                        Text::new("Model Status: Loading..."),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        StatusDisplay,
                    ));

                    // Input section
                    content
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(10.0),
                            ..default()
                        },))
                        .with_children(|inputs| {
                            // Input row
                            inputs
                                .spawn((Node {
                                    width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::SpaceEvenly,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(20.0),
                                    ..default()
                                },))
                                .with_children(|input_row| {
                                    // Input 1
                                    input_row
                                        .spawn((Node {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            row_gap: Val::Px(5.0),
                                            ..default()
                                        },))
                                        .with_children(|input1| {
                                            input1.spawn((
                                                Text::new("Input 1:"),
                                                TextFont {
                                                    font_size: 14.0,
                                                    ..default()
                                                },
                                                TextColor(TEXT_COLOR),
                                            ));
                                            input1
                                                .spawn((
                                                    Node {
                                                        width: Val::Px(80.0),
                                                        height: Val::Px(30.0),
                                                        border: UiRect::all(Val::Px(1.0)),
                                                        justify_content: JustifyContent::Center,
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },
                                                    BackgroundColor(Color::WHITE),
                                                    BorderColor(GRAY_SECONDARY),
                                                    InputField {
                                                        field_id: 1,
                                                        placeholder: "0.0".to_string(),
                                                    },
                                                ))
                                                .with_children(|field| {
                                                    field.spawn((
                                                        Text::new("0.0"),
                                                        TextFont {
                                                            font_size: 14.0,
                                                            ..default()
                                                        },
                                                        TextColor(Color::BLACK),
                                                    ));
                                                });
                                        });

                                    // Input 2
                                    input_row
                                        .spawn((Node {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            row_gap: Val::Px(5.0),
                                            ..default()
                                        },))
                                        .with_children(|input2| {
                                            input2.spawn((
                                                Text::new("Input 2:"),
                                                TextFont {
                                                    font_size: 14.0,
                                                    ..default()
                                                },
                                                TextColor(TEXT_COLOR),
                                            ));
                                            input2
                                                .spawn((
                                                    Node {
                                                        width: Val::Px(80.0),
                                                        height: Val::Px(30.0),
                                                        border: UiRect::all(Val::Px(1.0)),
                                                        justify_content: JustifyContent::Center,
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },
                                                    BackgroundColor(Color::WHITE),
                                                    BorderColor(GRAY_SECONDARY),
                                                    InputField {
                                                        field_id: 2,
                                                        placeholder: "0.0".to_string(),
                                                    },
                                                ))
                                                .with_children(|field| {
                                                    field.spawn((
                                                        Text::new("0.0"),
                                                        TextFont {
                                                            font_size: 14.0,
                                                            ..default()
                                                        },
                                                        TextColor(Color::BLACK),
                                                    ));
                                                });
                                        });
                                });

                            // Predict Button
                            inputs
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(120.0),
                                        height: Val::Px(40.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::vertical(Val::Px(10.0)),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(GREEN_PRIMARY),
                                    BorderColor(GREEN_PRIMARY),
                                    PredictButton,
                                ))
                                .with_children(|button| {
                                    button.spawn((
                                        Text::new("Predict"),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(TEXT_COLOR),
                                    ));
                                });
                        });

                    // Output section
                    content
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(8.0),
                            ..default()
                        },))
                        .with_children(|output| {
                            output.spawn((
                                Text::new("Output: --"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                                OutputDisplay,
                            ));
                            output.spawn((
                                Text::new("True Value: --"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                            ));
                            output.spawn((
                                Text::new("Error: --"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                            ));
                        });
                });
        });
}

/// Start loading the embedded model assets
fn start_loading_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading embedded model assets...");

    // Load the embedded assets
    let toml_handle = asset_server.load("embedded://interactive/models/demo_model.toml");
    let safetensors_handle =
        asset_server.load("embedded://interactive/models/demo_model.safetensors");

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
                        let error_message = format!("Failed to load model from assets: {}", error);
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
                        return;
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
                        return;
                    }
                }
            }
        }
    }
}

/// Load the model from the loaded binary assets using shared persistence functions
fn load_model_from_assets(
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

/// System that runs when the application is ready
fn on_ready_system() {
    info!("Application ready - model loaded successfully");
}

/// System that runs when there is a loading error
fn on_error_system(error: Res<LoadingError>) {
    error!("Application error state: {}", error.message);
}

/// Update UI when the app is ready
fn update_ui_for_ready(mut query: Query<&mut Text, With<StatusDisplay>>) {
    for mut text in query.iter_mut() {
        *text = Text::new("Model Status: Loaded");
    }
}

/// Update UI when there's an error
fn update_ui_for_error(mut query: Query<&mut Text, With<StatusDisplay>>, error: Res<LoadingError>) {
    for mut text in query.iter_mut() {
        *text = Text::new(&format!("Model Status: Error - {}", error.message));
    }
}

/// Handle button interactions with hover effects
fn update_button_interactions(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PredictButton>),
    >,
) {
    for (interaction, mut bg_color) in query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = BackgroundColor(GREEN_HOVER);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(GREEN_PRIMARY);
            }
            Interaction::Pressed => {
                info!("Predict button pressed!");
                *bg_color = BackgroundColor(GREEN_PRIMARY);
            }
        }
    }
}

/// Update status display based on current app state
fn update_status_display(
    mut query: Query<&mut Text, With<StatusDisplay>>,
    current_state: Res<State<AppState>>,
) {
    for mut text in query.iter_mut() {
        match current_state.get() {
            AppState::Loading => {
                *text = Text::new("Model Status: Loading...");
            }
            AppState::Ready => {
                // Keep the "Loaded" text from update_ui_for_ready
            }
            AppState::Error => {
                // Keep the error text from update_ui_for_error
            }
        }
    }
}
