// Interactive demo library
// Re-exports for the main binary

pub mod app;
pub mod model;
pub mod ui;

// Re-export commonly used types for main.rs
pub use app::{AppState, setup_application};
pub use model::{
    EmbeddedAssetsPlugin, check_asset_loading, on_error_system, on_ready_system,
    start_loading_assets,
};
pub use ui::setup_ui;
pub use ui::{
    InputValues, PredictionRequest, PredictionResults, ValidationState, manage_input_focus,
    process_prediction_requests, sanitize_numeric_inputs, update_button_interactions,
    update_input_styling, update_output_displays, update_ui_for_error, update_ui_for_ready,
    validate_numeric_inputs,
};
