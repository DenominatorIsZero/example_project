// UI components and markers

use bevy::prelude::*;

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