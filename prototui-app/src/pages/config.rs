use crate::commons::window_border;
use crate::controller::MetadataController;
use crate::view::draw_metadata;
use crossterm::event::KeyEvent;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

/// The Page for additional server configs.
/// It lets the user choose the metadata and the server address.
pub struct GrpcConfigPage<'a> {
    /// The controller for the metadata field
    pub metadata: MetadataController<'a>,
}

impl<'a> GrpcConfigPage<'a> {
    /// Instantiate the settings page
    pub fn new() -> Self {
        let metadata = MetadataController::new();
        Self { metadata }
    }

    /// The key bindings
    pub fn on_key(&mut self, key: KeyEvent) -> bool {
        self.metadata.on_key(key)
    }

    /// render the widgets of this page
    pub fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {
        // Create two chunks with equal horizontal screen space
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(f.size());

        draw_metadata(
            f,
            chunks[1],
            &mut self.metadata,
            window_border("Metadata", false),
        );
    }

    /// Returns the metadata as a string
    pub fn get_metadata(&self) -> String {
        self.metadata.get_editor().get_text_raw()
    }

    /// This is called when we switch back from this page to the home page. We validate
    /// the config settings and only allow to switch if there are no errors
    pub fn finish(&mut self) -> Result<(), String> {
        if let Some(err) = self.metadata.get_error() {
            return Err(err.msg);
        }
        Ok(())
    }
}
