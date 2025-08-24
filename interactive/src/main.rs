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

// UI Style Functions
// Note: We use functions instead of constants because:
// - Node::default() cannot be evaluated at compile time
// - LazyLock + clone() approach is more verbose and no faster
// - Simple functions are cleaner and more idiomatic for this use case
fn main_container_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

fn central_content_style() -> Node {
    Node {
        width: Val::Px(400.0),
        height: Val::Px(500.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::SpaceEvenly,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(20.0)),
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    }
}

fn input_section_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(10.0),
        ..default()
    }
}

fn input_row_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceEvenly,
        align_items: AlignItems::Center,
        column_gap: Val::Px(20.0),
        ..default()
    }
}

fn input_field_container_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(5.0),
        ..default()
    }
}

fn input_field_style() -> Node {
    Node {
        width: Val::Px(80.0),
        height: Val::Px(30.0),
        border: UiRect::all(Val::Px(1.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

fn button_style() -> Node {
    Node {
        width: Val::Px(120.0),
        height: Val::Px(40.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::vertical(Val::Px(10.0)),
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    }
}

fn output_section_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        ..default()
    }
}

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

// UI Component Functions

/// Spawn the central content box with all UI sections
fn spawn_content_box(builder: &mut ChildSpawnerCommands) {
    builder
        .spawn((
            central_content_style(),
            BackgroundColor(GRAY_SECONDARY),
            BorderColor(GREEN_PRIMARY),
        ))
        .with_children(|content| {
            spawn_title(content);
            spawn_status_display(content);
            spawn_input_section(content);
            spawn_output_section(content);
        });
}

/// Spawn the title text
fn spawn_title(builder: &mut ChildSpawnerCommands) {
    builder.spawn((
        Text::new("Minimal AI Demo"),
        TextFont { font_size: 28.0, ..default() },
        TextColor(TEXT_COLOR),
        TitleText,
    ));
}

/// Spawn the model status display
fn spawn_status_display(builder: &mut ChildSpawnerCommands) {
    builder.spawn((
        Text::new("Model Status: Loading..."),
        TextFont { font_size: 16.0, ..default() },
        TextColor(TEXT_COLOR),
        StatusDisplay,
    ));
}

/// Spawn the input section (input row + predict button)
fn spawn_input_section(builder: &mut ChildSpawnerCommands) {
    builder.spawn((input_section_style(),)).with_children(|inputs| {
        spawn_input_row(inputs);
        spawn_predict_button(inputs);
    });
}

/// Spawn the row containing both input fields
fn spawn_input_row(builder: &mut ChildSpawnerCommands) {
    builder.spawn((input_row_style(),)).with_children(|input_row| {
        spawn_input_field(input_row, "Input 1:", 1);
        spawn_input_field(input_row, "Input 2:", 2);
    });
}

/// Spawn a single input field with label
fn spawn_input_field(builder: &mut ChildSpawnerCommands, label: &str, field_id: usize) {
    builder.spawn((input_field_container_style(),))
        .with_children(|container| {
            // Label
            container.spawn((
                Text::new(label),
                TextFont { font_size: 14.0, ..default() },
                TextColor(TEXT_COLOR),
            ));
            
            // Input field
            container.spawn((
                input_field_style(),
                BackgroundColor(Color::WHITE),
                BorderColor(GRAY_SECONDARY),
                InputField {
                    field_id,
                    placeholder: "0.0".to_string(),
                },
            ))
            .with_children(|field| {
                field.spawn((
                    Text::new("0.0"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::BLACK),
                ));
            });
        });
}

/// Spawn the predict button
fn spawn_predict_button(builder: &mut ChildSpawnerCommands) {
    builder.spawn((
        Button,
        button_style(),
        BackgroundColor(GREEN_PRIMARY),
        BorderColor(GREEN_PRIMARY),
        PredictButton,
    ))
    .with_children(|button| {
        button.spawn((
            Text::new("Predict"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(TEXT_COLOR),
        ));
    });
}

/// Spawn the output section with all result displays
fn spawn_output_section(builder: &mut ChildSpawnerCommands) {
    builder.spawn((output_section_style(),)).with_children(|output| {
        output.spawn((
            Text::new("Output: --"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(TEXT_COLOR),
            OutputDisplay,
        ));
        output.spawn((
            Text::new("True Value: --"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(TEXT_COLOR),
        ));
        output.spawn((
            Text::new("Error: --"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(TEXT_COLOR),
        ));
    });
}

/// Set up the main UI layout
fn setup_ui(mut commands: Commands) {
    // Main container - full screen with dark background
    commands
        .spawn((
            main_container_style(),
            BackgroundColor(BACKGROUND_COLOR),
            MainContainer,
        ))
        .with_children(spawn_content_box);
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
        *text = Text::new(format!("Model Status: Error - {}", error.message));
    }
}

/// Handle button interactions with hover effects
type ButtonInteractionQuery<'w, 's> = Query<
    'w, 's,
    (&'static Interaction, &'static mut BackgroundColor),
    (Changed<Interaction>, With<PredictButton>),
>;

fn update_button_interactions(mut button_query: ButtonInteractionQuery) {
    for (interaction, mut bg_color) in button_query.iter_mut() {
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
