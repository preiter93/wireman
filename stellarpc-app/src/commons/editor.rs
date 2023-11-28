#![allow(clippy::module_name_repetitions)]
use arboard::Clipboard;
use crossterm::event::KeyEvent;
use edtui::{EditorMode, EditorState, Input};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref CLIPBOARD: Mutex<Option<Clipboard>> = Mutex::new(Clipboard::new().ok());
}

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
        Self {
            state: EditorState::new(),
            input: Input::default(),
            error: None,
            focus: false,
        }
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
        self.state
            .lines()
            .iter()
            .map(std::convert::Into::into)
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Gets the editors content and formats it to json
    pub fn get_text_json(&self) -> Result<String, ErrorKind> {
        pretty_format_json(&self.get_text_raw())
    }

    /// Set the editors content from raw text
    pub fn set_text_raw(&mut self, text: &str) {
        self.state.clear();
        for line in text.lines() {
            self.state.push(line);
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

    /// Clear all text
    pub fn clear(&mut self) {
        self.state.clear();
    }

    /// Whether the editor is in insert mode
    pub fn insert_mode(&self) -> bool {
        self.state.mode == EditorMode::Insert
    }

    /// Paste text from clipboard to editor
    pub fn paste_from_clipboard(&mut self) {
        if let Ok(mut clipboard) = CLIPBOARD.lock() {
            if let Some(clipboard) = &mut *clipboard {
                if let Ok(text) = clipboard.get_text() {
                    self.state.insert_string(&text);
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
    fn insert_char(&mut self, c: char) {
        self.state.insert_char(c);
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
        self.state.lines().is_empty()
    }

    /// Key bindings in normal mode
    pub fn on_key(&mut self, key: KeyEvent) {
        self.input.on_key(key, &mut self.state);
    }
}

// /// The editor mode, i.e. Normal or Insert.
// #[derive(Clone, PartialEq, Eq, Default)]
// pub enum EditorMode {
//     #[default]
//     None,
//     Normal,
//     Insert,
// }

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
