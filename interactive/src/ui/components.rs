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
pub struct PredictButton;

#[derive(Component)]
pub struct OutputDisplay;

// Input field markers for text input system
#[derive(Component)]
pub struct Input1;

#[derive(Component)]
pub struct Input2;

// Resources for input state management
#[derive(Resource, Default)]
pub struct InputValues {
    pub value1: Option<f32>,
    pub value2: Option<f32>,
}

#[derive(Resource)]
pub struct ValidationState {
    pub input1_valid: bool,
    pub input2_valid: bool,
    pub input1_empty: bool,
    pub input2_empty: bool,
}

impl Default for ValidationState {
    fn default() -> Self {
        Self {
            input1_valid: false,
            input2_valid: false,
            input1_empty: true,  // Start as empty
            input2_empty: true,  // Start as empty
        }
    }
}

impl ValidationState {
    pub fn both_valid(&self) -> bool {
        self.input1_valid && self.input2_valid && !self.input1_empty && !self.input2_empty
    }
}