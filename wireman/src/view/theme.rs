mod tailwind;
use ratatui::{prelude::*, widgets::BorderType};

pub struct Theme {
    pub root: Style,
    pub content: Style,
    pub app_title: Style,
    pub tabs: Style,
    pub tabs_active: Style,
    pub tabs_selected: Style,
    pub borders: Style,
    pub description: Style,
    pub description_title: Style,
    pub key_binding: KeyBinding,
    pub list: List,
    pub status_line: (Style, Style),
    pub divider: Divider,
}

pub struct KeyBinding {
    pub key: Style,
    pub description: Style,
}

pub struct List {
    pub selected: Style,
}
pub struct Divider {
    pub title: Style,
    pub border_style: Style,
    pub border_type: BorderType,
}

pub const THEME: Theme = Theme {
    root: Style::new().bg(DARK_BLUE),
    content: Style::new().bg(DARK_BLUE).fg(LIGHT_GRAY),
    app_title: Style::new()
        .fg(WHITE)
        .bg(DARK_BLUE)
        .add_modifier(Modifier::BOLD),
    tabs: Style::new().fg(MID_GRAY).bg(DARK_BLUE),
    tabs_active: Style::new().fg(WHITE).bg(DARK_BLUE),
    tabs_selected: Style::new()
        .fg(WHITE)
        .bg(DARK_BLUE)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::REVERSED),
    borders: Style::new().fg(LIGHT_GRAY),
    description: Style::new().fg(LIGHT_GRAY).bg(DARK_BLUE),
    description_title: Style::new().fg(LIGHT_GRAY).add_modifier(Modifier::BOLD),
    key_binding: KeyBinding {
        key: Style::new().fg(BLACK).bg(DARK_GRAY),
        description: Style::new().fg(DARK_GRAY).bg(BLACK),
    },
    list: List {
        selected: Style::new().fg(DARK_BLUE).bg(WHITE),
    },
    status_line: (
        Style::new()
            .fg(WHITE)
            .bg(LIGHT_PURPLE)
            .add_modifier(Modifier::BOLD),
        Style::new().bg(DARK_PURPLE),
    ),
    divider: Divider {
        title: Style::new()
            .add_modifier(Modifier::BOLD)
            .fg(WHITE)
            .bg(DARK_PURPLE),
        border_style: Style::new().fg(DARK_BLUE).bg(DARK_BLUE),
        border_type: BorderType::Thick,
    },
};

// Blue
const DARK_BLUE: Color = tailwind::SLATE.c900;

// Purple
const LIGHT_PURPLE: Color = tailwind::PURPLE.c700;
const DARK_PURPLE: Color = tailwind::PURPLE.c900;

// Gray
const LIGHT_GRAY: Color = tailwind::SLATE.c400;
const MID_GRAY: Color = tailwind::SLATE.c600;
const DARK_GRAY: Color = tailwind::SLATE.c800;

// Black and white
const WHITE: Color = tailwind::SLATE.c100;
const BLACK: Color = tailwind::SLATE.c950;
