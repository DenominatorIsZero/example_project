// UI interaction and update systems

use bevy::prelude::*;
use crate::app::AppState;
use crate::model::LoadingError;
use super::{components::*, constants::*};

/// Handle button interactions with hover effects
type ButtonInteractionQuery<'w, 's> = Query<
    'w, 's,
    (&'static Interaction, &'static mut BackgroundColor),
    (Changed<Interaction>, With<PredictButton>),
>;

pub fn update_button_interactions(mut button_query: ButtonInteractionQuery) {
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

/// Update UI when the app is ready
pub fn update_ui_for_ready(mut query: Query<&mut Text, With<StatusDisplay>>) {
    for mut text in query.iter_mut() {
        *text = Text::new("Model Status: Loaded");
    }
}

/// Update UI when there's an error
pub fn update_ui_for_error(mut query: Query<&mut Text, With<StatusDisplay>>, error: Res<LoadingError>) {
    for mut text in query.iter_mut() {
        *text = Text::new(format!("Model Status: Error - {}", error.message));
    }
}

/// Update status display based on current app state
pub fn update_status_display(
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