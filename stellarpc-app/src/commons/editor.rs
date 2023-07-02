#![allow(clippy::module_name_repetitions)]
use crate::theme;
use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent};
use lazy_static::lazy_static;
use ratatui::{
    style::Style,
    widgets::{Block, Widget},
};
use std::sync::Mutex;
use tui_textarea::{CursorMove, Input, TextArea};

lazy_static! {
    pub static ref CLIPBOARD: Mutex<Option<Clipboard>> = Mutex::new(Clipboard::new().ok());
}

/// Basic editor. Supports different modes, json formatting
/// and specifies commonly used key bindings.
#[derive(Clone)]
pub struct TextEditor<'a> {
    /// Textarea contains all the core functionality
    editor: TextArea<'a>,

    /// Error buffer
    error: Option<ErrorKind>,

    /// The editor mode
    mode: EditorMode,
}

impl<'a> Default for TextEditor<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> TextEditor<'a> {
    /// Returns an empty editor
    pub fn new() -> Self {
        Self {
            editor: TextArea::new(Vec::new()),
            error: None,
            mode: EditorMode::Normal,
        }
    }

    /// Whether we are in insert mode
    pub fn insert_mode(&self) -> bool {
        self.mode == EditorMode::Insert
    }

    /// Returns an empty editor
    pub fn from_str(text: &str) -> Self {
        let mut editor = Self::new();
        editor.set_text_raw(text);
        editor
    }

    /// Gets the editors content as raw text
    pub fn get_text_raw(&self) -> String {
        self.editor.clone().into_lines().join("\n")
    }

    /// Set the editors content from raw text
    pub fn set_text_raw(&mut self, text: &str) {
        self.editor = TextArea::new(text.lines().map(ToOwned::to_owned).collect());
    }

    /// Return the error
    pub fn get_error(&self) -> Option<ErrorKind> {
        self.error.clone()
    }

    /// Set the error
    pub fn set_error(&mut self, error: Option<ErrorKind>) {
        self.error = error;
    }

    /// Clear all text
    pub fn clear(&mut self) {
        self.editor = TextArea::new(vec![]);
    }

    /// Go into normal mode
    pub fn set_normal_mode(&mut self) {
        self.mode = EditorMode::Normal;
    }

    /// Go into insert mode
    pub fn set_insert_mode(&mut self) {
        self.mode = EditorMode::Insert;
    }

    /// Paste text from clipboard to editor
    pub fn paste_from_clipboard(&mut self) {
        if let Ok(mut clipboard) = CLIPBOARD.lock() {
            if let Some(clipboard) = &mut *clipboard {
                if let Ok(text) = clipboard.get_text() {
                    self.insert_str(&text);
                }
            }
        }
    }

    /// Yanks the full text
    pub fn yank(&self) {
        Self::yank_to_clipboard(&self.get_text_raw());
    }

    /// Yank text to clipboard
    pub fn yank_to_clipboard(text: &str) {
        if let Ok(mut clipboard) = CLIPBOARD.lock() {
            if let Some(clipboard) = &mut *clipboard {
                let _res = clipboard.set_text(text);
            }
        }
    }

    /// Insert a str at the current cursor position. Handles newlines.
    fn insert_str(&mut self, s: &str) {
        let mut iter = s.lines().peekable();
        while let Some(line) = iter.next() {
            self.editor.insert_str(line);
            if iter.peek().is_some() {
                self.editor.insert_newline();
            }
        }
    }

    /// Updates the style depending on the editor mode
    pub fn update_style(&mut self) {
        // Set the cursor style depending on the mode
        let cursor_style = if self.mode == EditorMode::Insert {
            Style::default()
                .fg(theme::COL_CURSOR_INSERT_MODE)
                .add_modifier(theme::MOD_CURSOR_INSERT_MODE)
        } else {
            Style::default()
                .fg(theme::COL_CURSOR_NORMAL_MODE)
                .add_modifier(theme::MOD_CURSOR_NORMAL_MODE)
        };

        self.set_cursor_style(cursor_style);
        self.set_cursor_line_style(Style::default());
    }

    pub(crate) fn set_style(&mut self, style: Style) {
        self.editor.set_style(style);
    }

    pub(crate) fn set_block(&mut self, block: Block<'a>) {
        self.editor.set_block(block);
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
        self.editor.is_empty()
    }

    /// Returns the editors mode
    pub fn mode(&self) -> EditorMode {
        self.mode.clone()
    }

    pub fn widget(&self) -> impl Widget + '_ {
        self.editor.widget()
    }

    /// Key bindings in normal mode
    pub fn on_key_normal_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('i') => self.mode = EditorMode::Insert,
            KeyCode::Char('a') => {
                self.mode = EditorMode::Insert;
                self.editor.move_cursor(CursorMove::Forward);
            }
            // Cursor movement
            KeyCode::Down | KeyCode::Char('j') => self.editor.move_cursor(CursorMove::Down),
            KeyCode::Up | KeyCode::Char('k') => self.editor.move_cursor(CursorMove::Up),
            KeyCode::Left | KeyCode::Char('h') => self.editor.move_cursor(CursorMove::Back),
            KeyCode::Right | KeyCode::Char('l') => self.editor.move_cursor(CursorMove::Forward),
            KeyCode::Char('w') => self.editor.move_cursor(CursorMove::WordForward),
            KeyCode::Char('b') => self.editor.move_cursor(CursorMove::WordBack),
            KeyCode::Char('J') => self.editor.move_cursor(CursorMove::End),
            KeyCode::Char('H') => self.editor.move_cursor(CursorMove::Head),
            // Delete
            KeyCode::Char('x') => {
                self.editor.delete_next_char();
            }
            KeyCode::Char('d') => {
                self.editor.delete_line_by_end();
            }
            KeyCode::Char('D') => {
                self.editor.delete_line_by_head();
            }
            // Undo
            KeyCode::Char('u') => {
                self.editor.undo();
            }
            KeyCode::Char('r') => {
                self.editor.redo();
            }
            // Yank & Paste
            KeyCode::Char('p') => self.paste_from_clipboard(),
            KeyCode::Char('y') => self.yank(),
            // Format json
            KeyCode::Char('f') => self.format_json(),
            _ => {}
        }
    }

    /// Key bindings in insert mode
    pub fn on_key_insert_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.mode = EditorMode::Normal,
            KeyCode::Down => self.editor.move_cursor(CursorMove::Down),
            KeyCode::Up => self.editor.move_cursor(CursorMove::Up),
            KeyCode::Right => self.editor.move_cursor(CursorMove::Forward),
            KeyCode::Left => self.editor.move_cursor(CursorMove::Back),
            _ => {
                self.editor.input_without_shortcuts(Input::from(key));
            }
        }
    }

    /// Set the editors cursor line style
    pub(crate) fn set_cursor_line_style(&mut self, cursor_line_style: ratatui::style::Style) {
        self.editor.set_cursor_line_style(cursor_line_style);
    }

    /// Set the editors cursor style
    pub(crate) fn set_cursor_style(&mut self, cursor_style: ratatui::style::Style) {
        self.editor.set_cursor_style(cursor_style);
    }
}

/// The editor mode, i.e. Normal or Insert.
#[derive(Clone, PartialEq, Eq, Default)]
pub enum EditorMode {
    #[default]
    Normal,
    Insert,
}

/// The error of the request. Can hold a kind value
/// to distinguish between format and grpc errors.
#[derive(Clone)]
pub struct ErrorKind {
    pub kind: String,
    pub msg: String,
}

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
