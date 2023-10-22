use crate::commons::editor::TextEditor;

/// The data model for the gRPC headers. Contains authorization
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
            Self::None => Self::Address,
            Self::Address => Self::Bearer,
            Self::Bearer => Self::Address,
        }
    }

    pub fn prev(&self) -> Self {
        match &self {
            Self::None => Self::Bearer,
            Self::Address => Self::Bearer,
            Self::Bearer => Self::Address,
        }
    }
}
