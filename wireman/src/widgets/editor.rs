use arboard::Clipboard;
use crossterm::event::MouseEvent;
use crossterm::event::{KeyCode, KeyEvent};
use edtui::{
    actions::{Execute, SwitchMode},
    clipboard::ClipboardTrait,
    EditorInput, EditorMode, EditorMouse, EditorState, EditorTheme, EditorView, Index2, Lines,
    RowIndex,
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
    input: EditorInput,

    /// Error buffer
    error: Option<ErrorKind>,

    /// Whether the editor is focused.
    focus: bool,

    /// Whether this is a single line editor.
    single_line: bool,
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
            input: EditorInput::default(),
            error: None,
            focus: false,
            single_line: false,
        }
    }
    /// Returns an empty single line editor
    pub fn single() -> Self {
        let mut state = EditorState::default();
        state.set_clipboard(Lazy::force(&CLIPBOARD));
        Self {
            state,
            input: EditorInput::default(),
            error: None,
            focus: false,
            single_line: true,
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

    /// Truncate after the first line
    pub fn truncate_first_line(&mut self) {
        if self.state.lines.len() <= 1 {
            return;
        }
        if let Some(first_line) = self.state.lines.get(RowIndex::new(0)) {
            let last_column = first_line.len().saturating_sub(1);
            self.state.cursor = Index2::new(0, last_column);
            self.state.lines = Lines::new(vec![first_line.clone()]);
        }
    }

    /// Handle key events.
    pub fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab | KeyCode::BackTab if self.single_line && self.insert_mode() => {
                SwitchMode(EditorMode::Normal).execute(&mut self.state);
            }
            _ => {
                self.input.on_event(key, &mut self.state);
            }
        }

        if self.single_line {
            self.truncate_first_line();
        }
    }
    /// Handle mouse events.
    pub fn on_mouse(&mut self, event: MouseEvent) {
        let mouse = EditorMouse::default();
        mouse.on_event(event, &mut self.state)
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
    EditorView::new(state).theme(
        EditorTheme::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title_style(theme.border.text.1)
                    .border_style(theme.border.border.1)
                    .border_type(theme.border.border_type.1)
                    .title(title.into())
                    .title_alignment(Alignment::Center),
            )
            .base(theme.editor.text)
            .cursor_style(theme.editor.cursor)
            .selection_style(theme.editor.selection)
            .hide_status_line(),
    )
}

/// Returns the editors view when unselected
pub fn view_unselected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView {
    let theme = Theme::global();
    EditorView::new(state).theme(
        EditorTheme::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title_style(theme.border.text.0)
                    .border_style(theme.border.border.0)
                    .border_type(theme.border.border_type.0)
                    .title(title.into())
                    .title_alignment(Alignment::Center),
            )
            .hide_status_line()
            .base(theme.editor.text)
            .cursor_style(theme.base.style)
            .selection_style(theme.editor.selection),
    )
}

/// Returns the editors view for a single line editor when selected.
pub fn view_single_selected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView {
    let theme = Theme::global();
    EditorView::new(state).theme(
        EditorTheme::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title_style(theme.border.text.1)
                    .border_style(theme.border.border.1)
                    .border_type(theme.border.border_type.1)
                    .title(title.into())
                    .title_alignment(Alignment::Left),
            )
            .hide_status_line()
            .base(theme.editor.text)
            .cursor_style(theme.editor.cursor)
            .selection_style(theme.editor.selection),
    )
}

/// Returns the editors view for a single line editor when unselected.
pub fn view_single_unselected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView {
    let theme = Theme::global();
    EditorView::new(state).theme(
        EditorTheme::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title_style(theme.border.text.0)
                    .border_style(theme.border.border.0)
                    .border_type(theme.border.border_type.0)
                    .style(theme.editor.text)
                    .title(title.into())
                    .title_alignment(Alignment::Left),
            )
            .hide_status_line()
            .hide_cursor()
            .base(theme.editor.text)
            .cursor_style(theme.base.style)
            .selection_style(theme.editor.selection),
    )
}
