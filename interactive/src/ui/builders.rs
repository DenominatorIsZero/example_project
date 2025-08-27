// UI building and spawning functions

use super::{components::*, constants::*, styles::*};
use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputTextColor, TextInputTextFont,
    TextInputValue,
};

// UI Component Functions

/// Spawn the central content box with all UI sections
pub fn spawn_content_box(builder: &mut ChildSpawnerCommands) {
    builder
        .spawn(central_content_style())
        .with_children(|content| {
            spawn_title(content);
            spawn_status_display(content);
            spawn_input_section(content);
            spawn_output_section(content);
        });
}

/// Spawn the title text
pub fn spawn_title(builder: &mut ChildSpawnerCommands) {
    builder.spawn((
        Text::new("Minimal AI Demo"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
        TitleText,
    ));
}

/// Spawn the model status display
pub fn spawn_status_display(builder: &mut ChildSpawnerCommands) {
    builder.spawn((
        Text::new("Model Status: Loading..."),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
        StatusDisplay,
    ));
}

/// Spawn the input section (input row + predict button)
pub fn spawn_input_section(builder: &mut ChildSpawnerCommands) {
    builder
        .spawn((input_section_style(),))
        .with_children(|inputs| {
            spawn_input_row(inputs);
            spawn_predict_button(inputs);
        });
}

/// Spawn the row containing both input fields
pub fn spawn_input_row(builder: &mut ChildSpawnerCommands) {
    builder
        .spawn((input_row_style(),))
        .with_children(|input_row| {
            spawn_input_field(input_row, "Input 1:", 1);
            spawn_input_field(input_row, "Input 2:", 2);
        });
}

/// Spawn a single input field with label
pub fn spawn_input_field(builder: &mut ChildSpawnerCommands, label: &str, field_id: usize) {
    builder
        .spawn((input_field_container_style(),))
        .with_children(|container| {
            // Label
            container.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            // Range indicator
            container.spawn((
                Text::new("(-1.0 to 1.0)"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)), // Light gray for subtle hint
            ));

            // Interactive text input field
            let mut input_entity = container.spawn((
                TextInput,
                Button, // Make it clickable
                input_field_style(),
                TextInputValue("".to_string()), // Start with empty string
                TextInputTextFont(TextFont {
                    font_size: 14.0,
                    ..default()
                }),
                TextInputTextColor(TextColor(Color::BLACK)),
            ));

            // Add field-specific marker components
            match field_id {
                1 => {
                    input_entity.insert(Input1);
                    // Input1 starts active (focused)
                    input_entity.insert(TextInputInactive(false)); // Active
                }
                2 => {
                    input_entity.insert(Input2);
                    // Input2 starts inactive (unfocused)
                    input_entity.insert(TextInputInactive(true)); // Inactive
                }
                _ => {}
            }
        });
}

/// Spawn the predict button
pub fn spawn_predict_button(builder: &mut ChildSpawnerCommands) {
    builder
        .spawn((
            Button,
            button_style(),
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
}

/// Spawn the output section with all result displays
pub fn spawn_output_section(builder: &mut ChildSpawnerCommands) {
    builder
        .spawn((output_section_style(),))
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
                TrueValueDisplay,
            ));
            output.spawn((
                Text::new("Error: --"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                ErrorDisplay,
            ));
        });
}

/// Set up the main UI layout
pub fn setup_ui(mut commands: Commands) {
    // Main container - full screen with dark background
    commands
        .spawn((
            main_container_style(),
            MainContainer,
        ))
        .with_children(spawn_content_box);
}
