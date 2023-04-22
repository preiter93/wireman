use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{style::Style, widgets::Widget};
use tui_textarea::{CursorMove, Input, TextArea};

use crate::theme;

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

impl<'a> TextEditor<'a> {
    /// Returns an empty editor
    pub fn new() -> Self {
        Self {
            editor: TextArea::new(Vec::new()),
            error: None,
            mode: EditorMode::Normal,
        }
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

    /// Set the default style
    pub fn set_style_default(&mut self) {
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
            // Delete
            KeyCode::Char('x') => {
                let _ = self.editor.delete_next_char();
            }
            KeyCode::Char('d') => {
                let _ = self.editor.delete_line_by_end();
                let _ = self.editor.delete_line_by_head();
            }
            // Undo
            KeyCode::Char('u') => {
                self.editor.undo();
            }
            KeyCode::Char('r') => {
                self.editor.redo();
            }
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

    /// Set the editors block
    pub(crate) fn set_block(&mut self, block: ratatui::widgets::Block<'a>) {
        self.editor.set_block(block);
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
/// to distinguish between format and gRPC errors.
#[derive(Clone)]
pub struct ErrorKind {
    pub kind: String,
    pub msg: String,
}

impl ErrorKind {
    fn format_error(msg: String) -> Self {
        Self {
            kind: "Format Error".to_owned(),
            msg,
        }
    }

    pub fn default_error(msg: String) -> Self {
        Self {
            kind: "Error".to_owned(),
            msg,
        }
    }
}

impl From<serde_json::Error> for ErrorKind {
    fn from(err: serde_json::Error) -> Self {
        Self::format_error(err.to_string())
    }
}

impl From<core::error::PTError> for ErrorKind {
    fn from(err: core::error::PTError) -> Self {
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
