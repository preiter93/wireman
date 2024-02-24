use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent};
use edtui::{
    actions::{Execute, InsertChar, SwitchMode},
    clipboard::ClipboardTrait,
    EditorMode, EditorState, EditorTheme, EditorView, Index2, Input, StatusLine,
};
use once_cell::sync::Lazy;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};
use std::sync::Mutex;
use theme::Theme;

/// Basic editor. Supports different modes, json formatting
/// and specifies commonly used key bindings.
#[derive(Clone)]
pub struct TextEditor {
    /// State contains the editors text and view state
    pub state: EditorState,

    /// The input register
    input: Input,

    /// Error buffer
    error: Option<ErrorKind>,

    /// Whether the editor is focused.
    focus: bool,
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl TextEditor {
    /// Returns an empty editor
    pub fn new() -> Self {
        let mut state = EditorState::default();
        state.set_clipboard(Lazy::force(&CLIPBOARD));
        Self {
            state,
            input: Input::default(),
            error: None,
            focus: false,
        }
    }

    pub fn clear(&mut self) {
        self.state.lines.clear();
        self.state.cursor = Index2::new(0, 0);
        self.state.selection = None;
    }

    pub fn focus(&mut self) {
        self.focus = true;
    }

    pub fn unfocus(&mut self) {
        self.focus = false;
    }

    /// Returns an empty editor
    pub fn from_str(text: &str) -> Self {
        let mut editor = Self::new();
        editor.set_text_raw(text);
        editor
    }

    /// Gets the editors content as raw text
    pub fn get_text_raw(&self) -> String {
        self.state.lines.clone().into()
    }

    /// Gets the editors content and formats it to json
    pub fn get_text_json(&self) -> Result<String, ErrorKind> {
        pretty_format_json(&self.get_text_raw())
    }

    /// Set the editors content from raw text
    pub fn set_text_raw(&mut self, text: &str) {
        self.clear();
        for line in text.lines() {
            let chars: Vec<char> = line.chars().collect();
            self.state.lines.push(chars);
        }
    }

    /// Return the error
    pub fn get_error(&self) -> Option<ErrorKind> {
        self.error.clone()
    }

    /// Set the error
    pub fn set_error(&mut self, error: Option<ErrorKind>) {
        self.error = error;
    }

    /// Whether the editor is in normal mode
    pub fn normal_mode(&self) -> bool {
        self.state.mode == EditorMode::Normal
    }

    /// Whether the editor is in insert mode
    pub fn insert_mode(&self) -> bool {
        self.state.mode == EditorMode::Insert
    }

    /// Whether the editor is in search mode
    pub fn search_mode(&self) -> bool {
        self.state.mode == EditorMode::Search
    }

    /// Whether the editor is in visual mode
    pub fn visual_mode(&self) -> bool {
        self.state.mode == EditorMode::Visual
    }

    /// Insert a str at the current cursor position. Handles newlines.
    fn insert_char(&mut self, ch: char) {
        InsertChar(ch).execute(&mut self.state);
    }

    /// Pretty formats the editors text. The error is stored
    /// internall in the error buffer.
    pub fn format_json(&mut self) {
        match pretty_format_json(&self.get_text_raw()) {
            Ok(pretty) => {
                self.set_text_raw(&pretty);
                self.error = None;
            }
            Err(err) => self.error = Some(err),
        }
    }

    /// Returns if the editor is empty
    pub fn is_empty(&self) -> bool {
        self.state.lines.is_empty()
    }

    /// Check if a given position is the first col.
    pub fn is_first_col(&self) -> bool {
        if self.is_empty() {
            return true;
        }
        let index = self.state.cursor;
        self.state.lines.is_first_col(index)
    }

    /// Check if a given position is the last col.
    pub fn is_last_col(&self) -> bool {
        if self.is_empty() {
            return true;
        }
        let index = self.state.cursor;
        self.state.lines.is_last_col(index)
    }

    /// Key bindings in normal mode
    pub fn on_key(&mut self, key: KeyEvent, single: bool) {
        match key.code {
            KeyCode::Tab | KeyCode::BackTab if single && self.insert_mode() => {
                SwitchMode(EditorMode::Normal).execute(&mut self.state);
            }
            _ => {
                self.input.on_key(key, &mut self.state);
            }
        }
    }
}

static CLIPBOARD: Lazy<GlobalClipboard> = Lazy::new(GlobalClipboard::new);

struct GlobalClipboard(Mutex<Option<Clipboard>>);

impl GlobalClipboard {
    pub fn new() -> Self {
        Self(Mutex::new(Clipboard::new().ok()))
    }
}

impl ClipboardTrait for &GlobalClipboard {
    fn set_text(&mut self, text: String) {
        if let Ok(mut clipboard) = self.0.lock() {
            if let Some(clipboard) = &mut *clipboard {
                let _ = clipboard.set_text(text);
            }
        }
    }

    fn get_text(&mut self) -> String {
        if let Ok(mut clipboard) = self.0.lock() {
            if let Some(clipboard) = &mut *clipboard {
                return clipboard.get_text().unwrap_or_default();
            }
        }
        String::new()
    }
}

/// Yank text to clipboard
pub fn yank_to_clipboard(text: &str) {
    if let Ok(mut clipboard) = CLIPBOARD.0.lock() {
        if let Some(clipboard) = &mut *clipboard {
            let _res = clipboard.set_text(text.to_string());
        }
    }
}

/// The error of the request. Can hold a kind value
/// to distinguish between format and grpc errors.
#[derive(Clone, Debug)]
pub struct ErrorKind {
    pub kind: String,
    pub msg: String,
}

unsafe impl Send for ErrorKind {}

impl ErrorKind {
    pub fn format_error(msg: String) -> Self {
        Self {
            kind: "Format Error".to_owned(),
            msg,
        }
    }

    pub fn default_error<T: Into<String>>(msg: T) -> Self {
        Self {
            kind: "Error".to_string(),
            msg: msg.into(),
        }
    }

    pub fn string(&self) -> String {
        format!("{}: {}", self.kind, self.msg)
    }
}

impl From<serde_json::Error> for ErrorKind {
    fn from(err: serde_json::Error) -> Self {
        Self::format_error(err.to_string())
    }
}

impl From<core::error::Error> for ErrorKind {
    fn from(err: core::error::Error) -> Self {
        Self::default_error(err.to_string())
    }
}

/// Pretty formats a string assuming it is in json format.
/// Returns an error if formatting fails.
pub fn pretty_format_json(input: &str) -> Result<String, ErrorKind> {
    let parsed = serde_json::from_str::<serde_json::Value>(input)?;
    let pretty = serde_json::to_string_pretty(&parsed)?;
    Ok(pretty)
}

/// Returns the editors view when selected.
pub fn view_selected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView {
    let theme = Theme::global();
    let block = Block::new()
        .borders(Borders::ALL)
        .title_style(theme.border.text_focused)
        .border_type(theme.border.border_type_focused)
        .border_style(theme.border.border_focused)
        .title(title.into())
        .title_alignment(Alignment::Center);
    let mut editor_theme = EditorTheme::default()
        .block(block)
        .status_line(
            StatusLine::default()
                .style_text(theme.editor.status_text)
                .style_line(theme.editor.status_line),
        )
        .base(theme.editor.text)
        .cursor_style(theme.editor.cursor)
        .selection_style(theme.editor.selection);
    if theme.editor.hide_status_line {
        editor_theme = editor_theme.hide_status_line();
    }
    EditorView::new(state).theme(editor_theme)
}

/// Returns the editors view when unselected
pub fn view_unselected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView {
    let theme = Theme::global();
    let block = Block::new()
        .borders(Borders::ALL)
        .title_style(theme.border.text)
        .border_style(theme.border.border)
        .title(title.into())
        .title_alignment(Alignment::Center);
    let theme = EditorTheme::default()
        .block(block)
        .hide_status_line()
        .base(theme.editor.text)
        .cursor_style(theme.base.style)
        .selection_style(theme.editor.selection);
    EditorView::new(state).theme(theme)
}

/// Returns the editors view for a single line editor when selected.
pub fn view_single_selected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView {
    let theme = Theme::global();
    let block = Block::new()
        .borders(Borders::ALL)
        .title_style(theme.border.text_focused)
        .border_type(theme.border.border_type_focused)
        .border_style(theme.border.border_focused)
        .title(title.into())
        .title_alignment(Alignment::Left);
    let theme = EditorTheme::default()
        .block(block)
        .hide_status_line()
        .base(theme.editor.text)
        .cursor_style(theme.editor.cursor)
        .selection_style(theme.editor.selection);
    EditorView::new(state).theme(theme)
}

/// Returns the editors view for a single line editor when unselected.
pub fn view_single_unselected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView {
    let theme = Theme::global();
    let block = Block::new()
        .borders(Borders::ALL)
        .title_style(theme.border.text)
        .border_type(theme.border.border_type)
        .border_style(theme.border.border)
        .style(theme.editor.text)
        .title(title.into())
        .title_alignment(Alignment::Left);
    let theme = EditorTheme::default()
        .block(block)
        .hide_status_line()
        .hide_cursor()
        .base(theme.editor.text)
        .cursor_style(theme.base.style)
        .selection_style(theme.editor.selection);
    EditorView::new(state).theme(theme)
}
