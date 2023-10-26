use std::collections::HashMap;

use crate::commons::editor::TextEditor;

/// The data model for the `gRPC` headers. Contains authorization
/// headers and metadata key value headers.
pub struct HeadersModel {
    /// The server address.
    pub address: TextEditor,
    /// The bearer token.
    pub bearer: TextEditor,
    /// The selection state.
    pub selected: HeadersSelection,
}

impl HeadersModel {
    /// Create a new `HeadersModel` instance
    pub fn new(default_address: &str) -> Self {
        let mut address = TextEditor::new();
        address.set_text_raw(default_address);
        Self {
            address,
            bearer: TextEditor::new(),
            selected: HeadersSelection::Bearer,
        }
    }
    /// Get the address as a string
    pub fn address(&self) -> String {
        self.address.get_text_raw()
    }

    /// Get the headers as a map
    pub fn headers(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if !self.bearer.is_empty() {
            map.insert(
                "authorization".to_string(),
                "Bearer ".to_owned() + &self.bearer.get_text_raw(),
            );
        }
        map
    }
}

/// The selection state of `HeadersModel`.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum HeadersSelection {
    #[default]
    None,
    Address,
    Bearer,
}
impl HeadersSelection {
    pub fn next(&self) -> Self {
        match &self {
            Self::None | Self::Bearer => Self::Address,
            Self::Address => Self::Bearer,
        }
    }

    pub fn prev(&self) -> Self {
        match &self {
            Self::None | Self::Address => Self::Bearer,
            Self::Bearer => Self::Address,
        }
    }
}
