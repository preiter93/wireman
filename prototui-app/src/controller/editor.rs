use crate::{
    commons::HelpActions,
    model::{editor::Mode, EditorModel},
};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use tui_textarea::{CursorMove, Input};

pub struct EditorController<'a> {
    pub model: EditorModel<'a>,
}

impl<'a> EditorController<'a> {
    pub fn new(model: EditorModel<'a>) -> Self {
        Self { model }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            if self.model.mode == Mode::Normal {
                self.on_key_normal_mode(key);
            } else {
                self.on_key_insert_mode(key);
            }
        }
    }

    /// Key bindings in normal mode
    fn on_key_normal_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('i') => self.model.mode = Mode::Insert,
            KeyCode::Char('a') => {
                self.model.mode = Mode::Insert;
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
            KeyCode::Esc => self.model.mode = Mode::Normal,
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
            Mode::Normal => {
                let mut actions = HelpActions::default();
                actions.insert("Tab", "Go to Selection");
                // actions.insert("j", "Down");
                // actions.insert("k", "Up");
                // actions.insert("h", "Left");
                // actions.insert("l", "Right");
                // actions.insert("w", "Next word");
                // actions.insert("b", "Previous word");
                // actions.insert("x", "Delete char");
                // actions.insert("d", "Delete line");
                // actions.insert("k", "Up");
                // actions.insert("u", "Undo");
                // actions.insert("r", "Redo");
                actions.insert("i", "Insert mode");
                actions.insert("Enter", "gRPC request");
                actions
            }
            Mode::Insert => {
                let mut actions = HelpActions::new();
                actions.insert("Esc", "Normal mode");
                actions
            }
        }
    }
}
