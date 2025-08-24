// UI style functions

use bevy::prelude::*;

// UI Style Functions
// Note: We use functions instead of constants because:
// - Node::default() cannot be evaluated at compile time
// - LazyLock + clone() approach is more verbose and no faster
// - Simple functions are cleaner and more idiomatic for this use case
pub fn main_container_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn central_content_style() -> Node {
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

pub fn input_section_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(10.0),
        ..default()
    }
}

pub fn input_row_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceEvenly,
        align_items: AlignItems::Center,
        column_gap: Val::Px(20.0),
        ..default()
    }
}

pub fn input_field_container_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(5.0),
        ..default()
    }
}

pub fn input_field_style() -> Node {
    Node {
        width: Val::Px(80.0),
        height: Val::Px(30.0),
        border: UiRect::all(Val::Px(1.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn button_style() -> Node {
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

pub fn output_section_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        ..default()
    }
}