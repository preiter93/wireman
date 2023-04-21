use crate::commons::editor::EditorMode;
use crate::commons::{editor::TextEditor, window_border};
use crate::view::draw_metadata;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

/// The Page for settings. It lets the user choose the metadata
/// and the server config.
pub struct ConfigPage<'a> {
    /// The controller for the metadata field
    pub editor: TextEditor<'a>,
}

impl<'a> ConfigPage<'a> {
    /// Instantiate the settings page
    pub fn new() -> Self {
        let text = TextEditor::new();
        Self { editor: text }
    }

    /// The key bindings
    pub fn on_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => return true,
            _ => {}
        }
        if self.editor.mode() == EditorMode::Normal {
            self.editor.on_key_normal_mode(key);
        } else {
            self.editor.on_key_insert_mode(key);
        }
        return false;
    }

    /// render the widgets of this page
    pub fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {
        // Create two chunks with equal horizontal screen space
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(f.size());

        draw_metadata(
            f,
            chunks[1],
            &mut self.editor,
            window_border("Metadata", false),
        );
    }

    /// Return metadata
    pub fn metadata(&self) -> String {
        self.editor.get_text_raw()
    }
}
