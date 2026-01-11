use arboard::Clipboard;
use crossterm::event::MouseEvent;
use crossterm::event::{KeyCode, KeyEvent};
use edtui::EditorEventHandler;
use edtui::{
    actions::{Execute, SwitchMode},
    clipboard::ClipboardTrait,
    EditorMode, EditorState, EditorTheme, EditorView, Index2, Lines, RowIndex,
};
use logger::Logger;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};
use std::sync::{Mutex, OnceLock};
use theme::Theme;

/// Basic editor. Supports different modes, json formatting
/// and specifies commonly used key bindings.
#[derive(Clone)]
pub struct TextEditor {
    /// State contains the editors text and view state
    pub state: EditorState,

    /// The input register
    handler: EditorEventHandler,

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
        state.set_clipboard(get_clipboard());
        Self {
            state,
            handler: EditorEventHandler::default(),
            error: None,
            focus: false,
            single_line: false,
        }
    }
    /// Returns an empty single line editor
    pub fn single() -> Self {
        let mut state = EditorState::default();
        state.set_clipboard(get_clipboard());
        Self {
            state,
            handler: EditorEventHandler::default(),
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
    /// internal in the error buffer.
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
    pub fn on_key<B: Backend>(&mut self, key: KeyEvent, terminal: &mut Terminal<B>) {
        match key.code {
            KeyCode::Tab | KeyCode::BackTab if self.single_line && self.insert_mode() => {
                SwitchMode(EditorMode::Normal).execute(&mut self.state);
            }
            _ => {
                self.handler.on_key_event(key, &mut self.state);
                if edtui::system_editor::is_pending(&self.state) {
                    let _ = edtui::system_editor::open(&mut self.state, terminal);
                    let _ = crossterm::execute!(
                        std::io::stdout(),
                        crossterm::event::EnableMouseCapture,
                        crossterm::event::EnableBracketedPaste
                    );
                }
            }
        }

        if self.single_line {
            self.truncate_first_line();
        }
    }
    /// Handle mouse events.
    pub fn on_mouse(&mut self, event: MouseEvent) {
        self.handler.on_mouse_event(event, &mut self.state);
    }

    /// Handle paste events.
    pub fn on_paste(&mut self, text: String) {
        self.handler.on_paste_event(text, &mut self.state);
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

    pub fn streaming_error(msg: String) -> Self {
        Self {
            kind: "Streaming Error".to_owned(),
            msg,
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
pub fn view_selected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView<'_, '_> {
    let theme = Theme::global();
    let line_numbers = match theme.editor.line_numbers {
        theme::LineNumbers::None => edtui::LineNumbers::None,
        theme::LineNumbers::Absolute => edtui::LineNumbers::Absolute,
        theme::LineNumbers::Relative => edtui::LineNumbers::Relative,
    };
    EditorView::new(state)
        .theme(
            EditorTheme::default()
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title_style(theme.title.focused)
                        .border_style(theme.border.focused)
                        .border_type(theme.border.border_type_focused)
                        .title(title.into())
                        .title_alignment(Alignment::Center),
                )
                .line_numbers_style(theme.border.unfocused)
                .base(theme.base.focused)
                .cursor_style(theme.base.focused.reversed())
                .selection_style(theme.highlight.focused.reversed())
                .hide_status_line(),
        )
        .line_numbers(line_numbers)
}

/// Returns the editors view when unselected
pub fn view_unselected<S: Into<String>>(state: &mut EditorState, title: S) -> EditorView<'_, '_> {
    let theme = Theme::global();
    let line_numbers = match theme.editor.line_numbers {
        theme::LineNumbers::None => edtui::LineNumbers::None,
        theme::LineNumbers::Absolute => edtui::LineNumbers::Absolute,
        theme::LineNumbers::Relative => edtui::LineNumbers::Relative,
    };
    EditorView::new(state)
        .theme(
            EditorTheme::default()
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title_style(theme.title.unfocused)
                        .border_style(theme.border.unfocused)
                        .border_type(theme.border.border_type_unfocused)
                        .title(title.into())
                        .title_alignment(Alignment::Center),
                )
                .line_numbers_style(theme.border.unfocused)
                .hide_status_line()
                .base(theme.base.unfocused)
                .cursor_style(theme.base.unfocused)
                .selection_style(theme.highlight.unfocused.reversed()),
        )
        .line_numbers(line_numbers)
}

/// Returns the editors view for a single line editor when selected.
pub fn view_single_selected<S: Into<String>>(
    state: &mut EditorState,
    title: S,
) -> EditorView<'_, '_> {
    let theme = Theme::global();
    EditorView::new(state).wrap(false).theme(
        EditorTheme::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title_style(theme.title.focused)
                    .border_style(theme.border.focused)
                    .border_type(theme.border.border_type_focused)
                    .title(title.into())
                    .title_alignment(Alignment::Center),
            )
            .hide_status_line()
            .base(theme.base.focused)
            .cursor_style(theme.base.focused.reversed())
            .selection_style(theme.highlight.focused.reversed()),
    )
}

/// Returns the editors view for a single line editor when unselected.
pub fn view_single_unselected<S: Into<String>>(
    state: &mut EditorState,
    title: S,
) -> EditorView<'_, '_> {
    let theme = Theme::global();
    EditorView::new(state).wrap(false).theme(
        EditorTheme::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title_style(theme.title.unfocused)
                    .border_style(theme.border.unfocused)
                    .border_type(theme.border.border_type_unfocused)
                    .style(theme.base.unfocused)
                    .title(title.into())
                    .title_alignment(Alignment::Center),
            )
            .hide_status_line()
            .hide_cursor()
            .base(theme.base.unfocused)
            .cursor_style(theme.base.unfocused)
            .selection_style(theme.highlight.unfocused.reversed()),
    )
}

static CLIPBOARD: OnceLock<Option<Mutex<Clipboard>>> = OnceLock::new();

#[must_use]
pub fn get_clipboard() -> ClipboardWrapper {
    let clipboard = CLIPBOARD
        .get_or_init(|| match Clipboard::new() {
            Ok(clipboard) => Some(Mutex::new(clipboard)),
            Err(err) => {
                Logger::debug(format!("failed to initialize clipboard: {err}"));
                None
            }
        })
        .as_ref();
    ClipboardWrapper(clipboard)
}

pub struct ClipboardWrapper(Option<&'static Mutex<Clipboard>>);

// Implementing ClipboardTrait for OnceLock Clipboard
impl ClipboardTrait for ClipboardWrapper {
    fn set_text(&mut self, text: String) {
        if let Some(clipboard_mutex) = &mut self.0 {
            if let Ok(mut clipboard) = clipboard_mutex.lock() {
                let _ = clipboard.set_text(text);
            }
        }
    }

    fn get_text(&mut self) -> String {
        if let Some(clipboard_mutex) = &mut self.0 {
            if let Ok(mut clipboard) = clipboard_mutex.lock() {
                return clipboard.get_text().unwrap_or_default();
            }
        }
        String::new()
    }
}

/// Yank text to clipboard
pub fn yank_to_clipboard(text: &str) {
    let mut clipboard = get_clipboard();
    clipboard.set_text(text.to_string());
}
