use core::MethodDescriptor;

use crate::{
    commons::HelpActions,
    model::request::{EditorMode, ErrorKind, RequestModel},
};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{style::Style, widgets::Block};
use tui_textarea::{CursorMove, Input};

/// Request controller manages the request and response messages.
pub struct RequestController<'a> {
    model: RequestModel<'a>,
}

impl<'a> RequestController<'a> {
    /// Instantiate a Controller from the model.
    pub fn new(model: RequestModel<'a>) -> Self {
        Self { model }
    }

    /// Handle user input.
    pub fn on_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            if self.model.mode == EditorMode::Normal {
                self.on_key_normal_mode(key);
            } else {
                self.on_key_insert_mode(key);
            }
        }
    }

    /// Key bindings in normal mode
    fn on_key_normal_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('i') => self.model.mode = EditorMode::Insert,
            KeyCode::Char('a') => {
                self.model.mode = EditorMode::Insert;
                self.model.editor.move_cursor(CursorMove::Forward);
            }
            KeyCode::Enter => self.model.call_grpc(),
            // Cursor movement
            KeyCode::Down | KeyCode::Char('j') => self.model.editor.move_cursor(CursorMove::Down),
            KeyCode::Up | KeyCode::Char('k') => self.model.editor.move_cursor(CursorMove::Up),
            KeyCode::Left | KeyCode::Char('h') => self.model.editor.move_cursor(CursorMove::Back),
            KeyCode::Right | KeyCode::Char('l') => {
                self.model.editor.move_cursor(CursorMove::Forward)
            }
            KeyCode::Char('w') => self.model.editor.move_cursor(CursorMove::WordForward),
            KeyCode::Char('b') => self.model.editor.move_cursor(CursorMove::WordBack),
            // Delete
            KeyCode::Char('x') => {
                let _ = self.model.editor.delete_next_char();
            }
            KeyCode::Char('d') => {
                let _ = self.model.editor.delete_line_by_end();
                let _ = self.model.editor.delete_line_by_head();
            }
            // Undo
            KeyCode::Char('u') => {
                self.model.editor.undo();
            }
            KeyCode::Char('r') => {
                self.model.editor.redo();
            }
            // Format json
            KeyCode::Char('f') => self.model.format_json(),
            _ => {}
        }
    }

    /// Key bindings in insert mode
    fn on_key_insert_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.model.mode = EditorMode::Normal,
            KeyCode::Down => self.model.editor.move_cursor(CursorMove::Down),
            KeyCode::Up => self.model.editor.move_cursor(CursorMove::Up),
            KeyCode::Right => self.model.editor.move_cursor(CursorMove::Forward),
            KeyCode::Left => self.model.editor.move_cursor(CursorMove::Back),
            // Use default key mappings in insert mode
            _ => {
                self.model.editor.input_without_shortcuts(Input::from(key));
            }
        }
    }

    /// Return a map of help actions. This is displayed in the
    /// helper wndow.
    pub fn help(&self) -> HelpActions {
        match self.model.mode {
            EditorMode::Normal => {
                let mut actions = HelpActions::default();
                actions.insert("Tab", "Go to Selection");
                actions.insert("i", "Insert mode");
                actions.insert("Enter", "gRPC request");
                actions
            }
            EditorMode::Insert => {
                let mut actions = HelpActions::new();
                actions.insert("Esc", "Normal mode");
                actions
            }
        }
    }

    /// Load a method in the request model
    pub fn load_method(&mut self, method: &MethodDescriptor) {
        self.model.load_method(method)
    }

    /// Returns the error to be displayed.
    pub fn error(&'a self) -> &'a Option<ErrorKind> {
        &self.model.error
    }

    /// Returns the response message to be displayed.
    pub fn response(&'a self) -> &'a Option<String> {
        &self.model.response
    }

    /// Returns the request editor widget
    pub fn request(&self) -> &'a tui_textarea::TextArea {
        &self.model.editor
    }

    /// Returns wether the editor is in insert mode
    pub fn insert_mode(&self) -> bool {
        self.model.mode == EditorMode::Insert
    }

    // Unfortunately the editor style is stored in the text area widget in the
    // model, so the model has some presentation logic responsibilities.
    pub fn set_cursor_style(
        &mut self,
        cursor_line_style: Style,
        block: Block<'a>,
        cursor_style: Style,
    ) {
        // Set the cursor line style
        self.model.editor.set_cursor_line_style(cursor_line_style);
        self.model.editor.set_block(block);
        // Set the cursor style depending on the mode
        self.model.editor.set_cursor_style(cursor_style);
    }
}
