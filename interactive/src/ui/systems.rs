// UI interaction and update systems

use super::{components::*, constants::*};
use crate::model::{LoadedModel, LoadingError};
use bevy::ecs::system::ParamSet;
use bevy::prelude::*;
use bevy_simple_text_input::{TextInput, TextInputInactive, TextInputValue};
use candle_core::IndexOp;
use shared::{Device, Tensor};

/// Handle button interactions with hover effects
type ButtonInteractionQuery<'w, 's> =
    Query<'w, 's, (&'static Interaction, &'static mut BackgroundColor), With<PredictButton>>;

/// Query for input1 styling updates
type Input1StylingQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut BorderColor,
        &'static mut BackgroundColor,
        &'static TextInputInactive,
    ),
    (With<Input1>, Without<Input2>),
>;

/// Query for input2 styling updates  
type Input2StylingQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut BorderColor,
        &'static mut BackgroundColor,
        &'static TextInputInactive,
    ),
    (With<Input2>, Without<Input1>),
>;

/// Query for input1 text sanitization
type Input1SanitizeQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut TextInputValue,
    (With<Input1>, Without<Input2>, Changed<TextInputValue>),
>;

/// Query for input2 text sanitization
type Input2SanitizeQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut TextInputValue,
    (With<Input2>, Without<Input1>, Changed<TextInputValue>),
>;

/// Query for input focus management
type FocusManagementQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static Interaction,
        Option<&'static Input1>,
        Option<&'static Input2>,
    ),
    (With<TextInput>, Changed<Interaction>),
>;

pub fn update_button_interactions(
    mut button_query: ButtonInteractionQuery,
    validation_state: Res<ValidationState>,
    input_values: Res<InputValues>,
    mut prediction_events: EventWriter<PredictionRequest>,
) {
    for (interaction, mut bg_color) in button_query.iter_mut() {
        let is_enabled = validation_state.both_valid();

        match *interaction {
            Interaction::Hovered if is_enabled => {
                *bg_color = BackgroundColor(GREEN_HOVER);
            }
            Interaction::None => {
                if is_enabled {
                    *bg_color = BackgroundColor(GREEN_PRIMARY);
                } else {
                    *bg_color = BackgroundColor(GRAY_SECONDARY); // Disabled state
                }
            }
            Interaction::Pressed if is_enabled => {
                if let (Some(value1), Some(value2)) = (input_values.value1, input_values.value2) {
                    info!("Predict button pressed with values: {}, {}", value1, value2);
                    prediction_events.write(PredictionRequest { value1, value2 });
                }
                *bg_color = BackgroundColor(GREEN_PRIMARY);
            }
            _ => {
                // Disabled or invalid state
                *bg_color = BackgroundColor(GRAY_SECONDARY);
            }
        }
    }
}

/// Update UI when the app is ready
pub fn update_ui_for_ready(mut query: Query<&mut Text, With<StatusDisplay>>) {
    for mut text in query.iter_mut() {
        *text = Text::new("Model Status: Loaded");
    }
}

/// Update UI when there's an error
pub fn update_ui_for_error(
    mut query: Query<&mut Text, With<StatusDisplay>>,
    error: Res<LoadingError>,
) {
    for mut text in query.iter_mut() {
        *text = Text::new(format!("Model Status: Error - {}", error.message));
    }
}

/// Validate and parse numeric input from text input fields
pub fn validate_numeric_inputs(
    input1_query: Query<Option<&TextInputValue>, With<Input1>>,
    input2_query: Query<Option<&TextInputValue>, With<Input2>>,
    mut input_values: ResMut<InputValues>,
    mut validation_state: ResMut<ValidationState>,
) {
    // Validate Input 1
    if let Ok(maybe_text_input_value) = input1_query.single() {
        if let Some(text_input_value) = maybe_text_input_value {
            let input_text = text_input_value.0.trim();

            if input_text.is_empty() {
                validation_state.input1_empty = true;
                validation_state.input1_valid = false;
                input_values.value1 = None;
            } else {
                validation_state.input1_empty = false;
                match input_text.parse::<f32>() {
                    Ok(value) if (INPUT_MIN..=INPUT_MAX).contains(&value) => {
                        validation_state.input1_valid = true;
                        input_values.value1 = Some(value);
                    }
                    _ => {
                        validation_state.input1_valid = false;
                        input_values.value1 = None;
                    }
                }
            }
        } else {
            // TextInputValue component doesn't exist yet - treat as empty
            validation_state.input1_empty = true;
            validation_state.input1_valid = false;
            input_values.value1 = None;
        }
    }

    // Validate Input 2
    if let Ok(maybe_text_input_value) = input2_query.single() {
        if let Some(text_input_value) = maybe_text_input_value {
            let input_text = text_input_value.0.trim();

            if input_text.is_empty() {
                validation_state.input2_empty = true;
                validation_state.input2_valid = false;
                input_values.value2 = None;
            } else {
                validation_state.input2_empty = false;
                match input_text.parse::<f32>() {
                    Ok(value) if (INPUT_MIN..=INPUT_MAX).contains(&value) => {
                        validation_state.input2_valid = true;
                        input_values.value2 = Some(value);
                    }
                    _ => {
                        validation_state.input2_valid = false;
                        input_values.value2 = None;
                    }
                }
            }
        } else {
            // TextInputValue component doesn't exist yet - treat as empty
            validation_state.input2_empty = true;
            validation_state.input2_valid = false;
            input_values.value2 = None;
        }
    }
}

/// Update visual styling based on validation and focus state
pub fn update_input_styling(
    mut input1_query: Input1StylingQuery,
    mut input2_query: Input2StylingQuery,
    validation_state: Res<ValidationState>,
) {
    // Update Input 1 styling
    if let Ok((mut border_color, mut bg_color, is_inactive)) = input1_query.single_mut() {
        let is_focused = !is_inactive.0; // Check the boolean value

        // Set background color based on focus state
        if is_focused {
            *bg_color = BackgroundColor(INPUT_FOCUSED_BG); // Focused: bright white
        } else {
            *bg_color = BackgroundColor(INPUT_UNFOCUSED_BG); // Unfocused: light gray
        }

        // Set border color based on validation state and focus
        if validation_state.input1_empty {
            if is_focused {
                *border_color = BorderColor(INPUT_FOCUSED_BORDER); // Focused blue
            } else {
                *border_color = BorderColor(GRAY_SECONDARY); // Unfocused gray
            }
        } else if validation_state.input1_valid {
            *border_color = BorderColor(GREEN_PRIMARY); // Valid input
        } else {
            *border_color = BorderColor(INPUT_INVALID_BORDER); // Invalid input (red)
        }
    }

    // Update Input 2 styling
    if let Ok((mut border_color, mut bg_color, is_inactive)) = input2_query.single_mut() {
        let is_focused = !is_inactive.0; // Check the boolean value

        // Set background color based on focus state
        if is_focused {
            *bg_color = BackgroundColor(INPUT_FOCUSED_BG); // Focused: bright white
        } else {
            *bg_color = BackgroundColor(INPUT_UNFOCUSED_BG); // Unfocused: light gray
        }

        // Set border color based on validation state and focus
        if validation_state.input2_empty {
            if is_focused {
                *border_color = BorderColor(INPUT_FOCUSED_BORDER); // Focused blue
            } else {
                *border_color = BorderColor(GRAY_SECONDARY); // Unfocused gray
            }
        } else if validation_state.input2_valid {
            *border_color = BorderColor(GREEN_PRIMARY); // Valid input
        } else {
            *border_color = BorderColor(INPUT_INVALID_BORDER); // Invalid input (red)
        }
    }
}

/// Sanitize text input in real-time to only allow valid numeric values
pub fn sanitize_numeric_inputs(
    mut input1_query: Input1SanitizeQuery,
    mut input2_query: Input2SanitizeQuery,
) {
    // Sanitize Input 1
    if let Ok(mut text_input_value) = input1_query.single_mut() {
        let sanitized = sanitize_numeric_string(&text_input_value.0);
        if sanitized != text_input_value.0 {
            text_input_value.0 = sanitized;
        }
    }

    // Sanitize Input 2
    if let Ok(mut text_input_value) = input2_query.single_mut() {
        let sanitized = sanitize_numeric_string(&text_input_value.0);
        if sanitized != text_input_value.0 {
            text_input_value.0 = sanitized;
        }
    }
}

/// Helper function to sanitize a string to only contain valid numeric characters
fn sanitize_numeric_string(input: &str) -> String {
    let mut result = String::new();
    let mut has_decimal = false;
    let mut has_minus = false;

    for (i, ch) in input.chars().enumerate() {
        match ch {
            // Allow digits always
            '0'..='9' => result.push(ch),

            // Allow minus only at the beginning and only once
            '-' if i == 0 && !has_minus => {
                has_minus = true;
                result.push(ch);
            }

            // Allow decimal point only once
            '.' if !has_decimal => {
                has_decimal = true;
                result.push(ch);
            }

            // Skip all other characters
            _ => {}
        }
    }

    // If we have a valid number, clamp it to our range
    if let Ok(value) = result.parse::<f32>() {
        let clamped = value.clamp(INPUT_MIN, INPUT_MAX);
        if (clamped - value).abs() > f32::EPSILON {
            // Value was clamped, format it nicely
            if clamped.fract() == 0.0 && clamped.abs() < 1000.0 {
                format!("{}", clamped as i32)
            } else {
                format!("{clamped:.2}")
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            }
        } else {
            result
        }
    } else if result.is_empty() || result == "-" || result == "." || result == "-." {
        // Allow these partial inputs while typing
        result
    } else {
        // Invalid format, clear it
        String::new()
    }
}

/// Manage input focus - ensure only one text input is active at a time
pub fn manage_input_focus(
    mut commands: Commands,
    input_queries: FocusManagementQuery,
    input1_query: Query<Entity, (With<Input1>, Without<Input2>)>,
    input2_query: Query<Entity, (With<Input2>, Without<Input1>)>,
) {
    for (clicked_entity, interaction, is_input1, is_input2) in input_queries.iter() {
        if *interaction == Interaction::Pressed {
            if is_input1.is_some() {
                debug!("Input1 clicked - focusing");

                // Make Input1 active (set TextInputInactive to false)
                commands
                    .entity(clicked_entity)
                    .insert(TextInputInactive(false));

                // Make Input2 inactive (set TextInputInactive to true)
                if let Ok(input2_entity) = input2_query.single() {
                    commands
                        .entity(input2_entity)
                        .insert(TextInputInactive(true));
                }
            } else if is_input2.is_some() {
                debug!("Input2 clicked - focusing");

                // Make Input2 active (set TextInputInactive to false)
                commands
                    .entity(clicked_entity)
                    .insert(TextInputInactive(false));

                // Make Input1 inactive (set TextInputInactive to true)
                if let Ok(input1_entity) = input1_query.single() {
                    commands
                        .entity(input1_entity)
                        .insert(TextInputInactive(true));
                }
            }
        }
    }
}

/// Process prediction requests and run model inference
pub fn process_prediction_requests(
    mut commands: Commands,
    mut prediction_events: EventReader<PredictionRequest>,
    model: Option<Res<LoadedModel>>,
) {
    // Only process events if we have a loaded model
    let Some(model) = model else {
        // Clear events if no model is loaded
        prediction_events.clear();
        return;
    };

    for event in prediction_events.read() {
        match run_inference(&model.model, event.value1, event.value2) {
            Ok((prediction, true_value, error)) => {
                info!(
                    "Inference result - Prediction: {:.3}, True: {:.3}, Error: {:.3}",
                    prediction, true_value, error
                );
                commands.insert_resource(PredictionResults {
                    prediction,
                    true_value,
                    error,
                });
            }
            Err(e) => {
                error!("Inference failed: {}", e);
                // Insert error results
                commands.insert_resource(PredictionResults {
                    prediction: 0.0,
                    true_value: 0.0,
                    error: 0.0,
                });
            }
        }
    }
}

/// Run model inference and compute true value and error
fn run_inference(
    model: &shared::DemoMLP,
    value1: f32,
    value2: f32,
) -> anyhow::Result<(f32, f32, f32)> {
    // Create input tensor [1, 2] shape for single prediction
    let device = Device::Cpu;
    let input_tensor = Tensor::from_vec(vec![value1, value2], (1, 2), &device)?;

    // Run model forward pass
    let output_tensor = model.forward(&input_tensor)?;

    // Extract prediction from output tensor
    let prediction = output_tensor.i(0)?.i(0)?.to_scalar::<f32>()?;

    // Compute true value using the same function as training data
    let true_value = (value1 + value2).tanh() * 0.5 + 0.5;

    // Compute absolute error
    let error = (prediction - true_value).abs();

    Ok((prediction, true_value, error))
}

/// Update output displays when prediction results change
#[allow(clippy::type_complexity)]
pub fn update_output_displays(
    mut queries: ParamSet<(
        Query<&mut Text, With<OutputDisplay>>,
        Query<&mut Text, With<TrueValueDisplay>>,
        Query<&mut Text, With<ErrorDisplay>>,
    )>,
    results: Option<Res<PredictionResults>>,
) {
    // Only update if we have results and they changed
    let Some(results) = results else {
        return;
    };
    if !results.is_changed() {
        return;
    }

    // Update output display
    if let Ok(mut text) = queries.p0().single_mut() {
        *text = Text::new(format!("Output: {:.3}", results.prediction));
    }

    // Update true value display
    if let Ok(mut text) = queries.p1().single_mut() {
        *text = Text::new(format!("True Value: {:.3}", results.true_value));
    }

    // Update error display
    if let Ok(mut text) = queries.p2().single_mut() {
        *text = Text::new(format!("Error: {:.3}", results.error));
    }
}
