use crate::commons::editor::TextEditor;

/// The metadata model
pub struct MetadataModel<'a> {
    /// The editor
    pub editor: TextEditor<'a>,
}

impl<'a> MetadataModel<'a> {
    /// Instantiate the model
    pub fn new() -> Self {
        let mut text = TextEditor::new();
        // Initialize with empty json for convenience
        text.set_text_raw("{}");
        Self { editor: text }
    }
}
