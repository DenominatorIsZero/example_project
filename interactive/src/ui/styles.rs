// UI style functions

use super::constants::*;
use bevy::prelude::*;

// UI Style Functions
// Note: We use functions instead of constants because:
// - Node::default() cannot be evaluated at compile time
// - LazyLock + clone() approach is more verbose and no faster
// - Simple functions are cleaner and more idiomatic for this use case
pub fn main_container_style() -> (Node, BackgroundColor) {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR),
    )
}

pub fn central_content_style() -> (Node, BackgroundColor, BorderColor) {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(CONTENT_PADDING)),
            border: UiRect::all(Val::Px(CONTENT_BORDER_WIDTH)),
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR),
        BorderColor(GRAY_SECONDARY),
    )
}

pub fn input_section_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(INPUT_SECTION_ROW_GAP),
        ..default()
    }
}

pub fn input_row_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceEvenly,
        align_items: AlignItems::Center,
        column_gap: Val::Px(INPUT_ROW_COLUMN_GAP),
        ..default()
    }
}

pub fn input_field_container_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(INPUT_FIELD_CONTAINER_GAP),
        ..default()
    }
}

pub fn input_field_style() -> (Node, BackgroundColor, BorderColor) {
    (
        Node {
            width: Val::Px(INPUT_FIELD_WIDTH),
            height: Val::Px(INPUT_FIELD_HEIGHT),
            border: UiRect::all(Val::Px(INPUT_FIELD_BORDER_WIDTH)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(INPUT_FOCUSED_BG),
        BorderColor(GRAY_SECONDARY),
    )
}

pub fn button_style() -> (Node, BackgroundColor, BorderColor) {
    (
        Node {
            width: Val::Px(BUTTON_WIDTH),
            height: Val::Px(BUTTON_HEIGHT),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
            border: UiRect::all(Val::Px(BUTTON_BORDER_WIDTH)),
            ..default()
        },
        BackgroundColor(GREEN_PRIMARY),
        BorderColor(GREEN_PRIMARY),
    )
}

pub fn output_section_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(OUTPUT_SECTION_ROW_GAP),
        ..default()
    }
}
