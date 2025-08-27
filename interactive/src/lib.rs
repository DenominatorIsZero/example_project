// Interactive demo library
// Re-exports for the main binary

pub mod app;
pub mod ui;
pub mod model;

// Re-export commonly used types for main.rs
pub use app::{AppState, setup_application};
pub use ui::setup_ui;
pub use model::{EmbeddedAssetsPlugin, start_loading_assets, check_asset_loading, 
                on_ready_system, on_error_system};
pub use ui::{update_button_interactions, update_ui_for_ready, update_ui_for_error, 
             validate_numeric_inputs, update_input_styling,
             sanitize_numeric_inputs, manage_input_focus, process_prediction_requests,
             update_output_displays, InputValues, ValidationState, PredictionRequest, 
             PredictionResults};