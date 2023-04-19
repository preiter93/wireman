use ratatui::{
    style::{Color, Modifier},
    widgets::BorderType,
};

const COLOR_PRIMARY: Color = Color::White;
const COLOR_SECONDARY: Color = Color::Rgb(21, 21, 21); // Dark Gray
const COLOR_TERTIARY: Color = Color::Rgb(0, 228, 228); // Dark Cyan

pub const COL_TEXT_NORMAL: Color = COLOR_PRIMARY;
pub const COL_TEXT_ERROR: Color = Color::Red;

pub const COL_BACKGROUND: Color = COLOR_SECONDARY;

// Window border
pub const COL_WINDOW_BORDER_FG: Color = COLOR_PRIMARY;
pub const COL_WINDOW_BORDER_BG: Color = COLOR_SECONDARY;
pub const COL_WINDOW_BORDER_HIGHLIGHTED_FG: Color = COLOR_TERTIARY;
pub const COL_WINDOW_BORDER_HIGHLIGHTED_BG: Color = COLOR_SECONDARY;
pub const COL_WINDOW_TITLE: Color = COLOR_TERTIARY;
pub const MOD_WINDOW_TITLE: Modifier = Modifier::BOLD;
pub const TYP_BORDER: BorderType = BorderType::Plain;
pub const TYP_BORDER_HIGHLIGHTED: BorderType = BorderType::Double;

// List
pub const COL_LIST_HIGHLIGHTED_SERVICE_FG: Color = COLOR_SECONDARY;
pub const COL_LIST_HIGHLIGHTED_SERVICE_BG: Color = COLOR_PRIMARY;
pub const COL_LIST_HIGHLIGHTED_METHOD_FG: Color = COLOR_SECONDARY;
pub const COL_LIST_HIGHLIGHTED_METHOD_BG: Color = COLOR_TERTIARY;

// Editor
pub const MOD_CURSOR_NORMAL_MODE: Modifier = Modifier::REVERSED;
pub const MOD_CURSOR_INSERT_MODE: Modifier = Modifier::UNDERLINED;
pub const COL_CURSOR_NORMAL_MODE: Color = COLOR_PRIMARY;
pub const COL_CURSOR_INSERT_MODE: Color = COLOR_PRIMARY;

// Help window
pub const COL_HELP_KEY_FG: Color = COLOR_TERTIARY;
pub const COL_HELP_MSG_FG: Color = COLOR_PRIMARY;
