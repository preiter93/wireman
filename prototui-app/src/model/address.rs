#![allow(clippy::module_name_repetitions)]
use crate::commons::editor::TextEditor;

pub struct AddressModel<'a> {
    pub editor: TextEditor<'a>,
}

impl<'a> AddressModel<'a> {
    pub fn new(default_address: &str) -> Self {
        let mut editor = TextEditor::new();
        editor.set_text_raw(default_address);
        Self { editor }
    }
}
