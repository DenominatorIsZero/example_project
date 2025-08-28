// UI constants and configuration for easy template customization

use bevy::prelude::*;

// =============================================================================
// COLOR PALETTE
// =============================================================================

// Website-matching color palette
#[allow(clippy::approx_constant)]
pub const BACKGROUND_COLOR: Color = Color::srgb(0.216, 0.255, 0.318); // rgb(55 65 81) - website main background
pub const TEXT_COLOR: Color = Color::WHITE;
pub const GREEN_PRIMARY: Color = Color::srgb(0.133, 0.698, 0.298); // green-500: #22c55e  
pub const GREEN_HOVER: Color = Color::srgb(0.251, 0.831, 0.412); // green-400: #4ade80
pub const GRAY_SECONDARY: Color = Color::srgb(0.294, 0.333, 0.388); // rgb(75 85 99) - website content areas
pub const YELLOW_ACCENT: Color = Color::srgb(0.918, 0.784, 0.157); // yellow-500: #eab308

// Input field state colors
pub const INPUT_UNFOCUSED_BG: Color = Color::srgb(0.95, 0.95, 0.95); // Light gray background when unfocused
pub const INPUT_FOCUSED_BG: Color = Color::WHITE; // Bright white background when focused
pub const INPUT_FOCUSED_BORDER: Color = Color::srgb(0.4, 0.6, 1.0); // Blue border when focused
pub const INPUT_INVALID_BORDER: Color = Color::srgb(0.8, 0.2, 0.2); // Red border for invalid input
pub const RANGE_HINT_COLOR: Color = Color::srgb(0.6, 0.6, 0.6); // Light gray for range indicators

// =============================================================================
// FONT SIZES
// =============================================================================

/// Main title font size (large, prominent)
pub const FONT_SIZE_TITLE: f32 = 28.0;

/// Standard text size for buttons, status, and output displays
pub const FONT_SIZE_STANDARD: f32 = 16.0;

/// Input field labels and input text size
pub const FONT_SIZE_LABEL: f32 = 14.0;

/// Small hint text size (range indicators)
pub const FONT_SIZE_HINT: f32 = 12.0;

// =============================================================================
// LAYOUT DIMENSIONS
// =============================================================================

// Content area layout
/// Padding around the main content area
pub const CONTENT_PADDING: f32 = 20.0;

/// Border width around the main content area
pub const CONTENT_BORDER_WIDTH: f32 = 2.0;

// Input section layout
/// Vertical gap between input row and predict button
pub const INPUT_SECTION_ROW_GAP: f32 = 10.0;

/// Horizontal gap between input fields in the input row
pub const INPUT_ROW_COLUMN_GAP: f32 = 20.0;

/// Vertical gap between label, hint, and input field
pub const INPUT_FIELD_CONTAINER_GAP: f32 = 5.0;

// Input field dimensions
/// Width of individual input fields
pub const INPUT_FIELD_WIDTH: f32 = 80.0;

/// Height of individual input fields
pub const INPUT_FIELD_HEIGHT: f32 = 30.0;

/// Border width around input fields
pub const INPUT_FIELD_BORDER_WIDTH: f32 = 1.0;

// Button dimensions
/// Width of the predict button
pub const BUTTON_WIDTH: f32 = 120.0;

/// Height of the predict button
pub const BUTTON_HEIGHT: f32 = 40.0;

/// Vertical margin around the predict button
pub const BUTTON_MARGIN: f32 = 10.0;

/// Border width around the predict button
pub const BUTTON_BORDER_WIDTH: f32 = 2.0;

// Output section layout
/// Vertical gap between output display lines
pub const OUTPUT_SECTION_ROW_GAP: f32 = 8.0;

// =============================================================================
// INPUT VALIDATION
// =============================================================================

/// Minimum valid input value
pub const INPUT_MIN: f32 = -1.0;

/// Maximum valid input value  
pub const INPUT_MAX: f32 = 1.0;

/// Input value range as a tuple for validation
pub const INPUT_RANGE: (f32, f32) = (INPUT_MIN, INPUT_MAX);
