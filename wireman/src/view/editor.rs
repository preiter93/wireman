use super::theme::THEME;
use edtui::{EditorState, EditorTheme, EditorView, StatusLine};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders},
};

/// Returns the editors view when selected.
pub fn view_selected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(title.into())
        .title_alignment(Alignment::Center)
        .style(THEME.content)
        .bold()
        .white();
    let theme = EditorTheme::default().block(block).status_line(
        StatusLine::default()
            .style_text(THEME.status_line.0)
            .style_line(THEME.status_line.1),
    );
    EditorView::new(state).theme(theme)
}

/// Returns the editors view when unselected
pub fn view_unselected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title(title.into())
        .title_alignment(Alignment::Center)
        .style(THEME.content)
        .bold()
        .white();
    let theme = EditorTheme::default()
        .block(block)
        .hide_status_line()
        .cursor_style(THEME.content);
    EditorView::new(state).theme(theme)
}

/// Returns the editors view for a single line editor when selected.
pub fn view_single_selected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(title.into())
        .title_alignment(Alignment::Left)
        .style(THEME.content)
        .bold()
        .white();
    let theme = EditorTheme::default().block(block).hide_status_line();
    EditorView::new(state).theme(theme)
}

/// Returns the editors view for a single line editor when unselected.
pub fn view_single_unselected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title(title.into())
        .title_alignment(Alignment::Left)
        .style(THEME.content)
        .bold()
        .white();
    let theme = EditorTheme::default()
        .block(block)
        .hide_status_line()
        .hide_cursor();
    EditorView::new(state).theme(theme)
}
