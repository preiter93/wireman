pub mod auth;
pub use auth::{AuthHeader, AuthSelection};
pub mod meta;
pub use meta::MetaHeaders;

use crate::commons::editor::TextEditor;
use edtui::EditorMode;
use std::{collections::HashMap, process::Command};

/// The data model for the `gRPC` headers. Contains authorization
/// headers and metadata key value headers.
pub struct HeadersModel {
    /// The host address.
    pub address: TextEditor,

    /// The authentication header.
    pub auth: AuthHeader,

    /// The metadata headers.
    pub meta: MetaHeaders,

    /// The selection state.
    pub selected: HeadersSelection,
}

impl Default for HeadersModel {
    fn default() -> Self {
        Self::new("")
    }
}

impl HeadersModel {
    /// Create a new `HeadersModel` instance
    pub fn new(default_address: &str) -> Self {
        let mut address = TextEditor::new();
        address.set_text_raw(default_address);
        Self {
            address,
            auth: AuthHeader::default(),
            meta: MetaHeaders::default(),
            selected: HeadersSelection::Auth,
        }
    }
    /// Get the address as a string
    pub fn address(&self) -> String {
        self.address.get_text_raw()
    }

    pub fn mode(&self) -> EditorMode {
        [self.auth.mode(), self.address.state.mode]
            .into_iter()
            .find(|&x| x != EditorMode::Normal)
            .unwrap_or(EditorMode::Normal)
    }

    /// Get the headers as a map
    pub fn headers(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if !self.auth.is_empty() {
            map.insert(AuthHeader::key(), self.auth.value());
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
    Auth,
    Metadata,
}
impl HeadersSelection {
    pub fn next(&self) -> Self {
        match &self {
            Self::None | Self::Auth => Self::Metadata,
            Self::Address => Self::Auth,
            Self::Metadata => Self::Address,
        }
    }

    pub fn prev(&self) -> Self {
        match &self {
            Self::None | Self::Address => Self::Metadata,
            Self::Auth => Self::Address,
            Self::Metadata => Self::Auth,
        }
    }
}

fn try_expand(raw: &str) -> String {
    if raw.starts_with("$(") && raw.ends_with(')') {
        let command = &raw[2..raw.len() - 1];
        if let Some(command) = execute_command(command) {
            return command;
        };
    }
    raw.to_string()
}

fn execute_command(command: &str) -> Option<String> {
    let mut parts = command.split_whitespace();
    let program = parts.next()?;
    let args: Vec<&str> = parts.collect();
    let output = Command::new(program).args(args).output();

    match output {
        Ok(output) if output.status.success() => {
            if let Ok(mut stdout) = String::from_utf8(output.stdout) {
                stdout.pop(); // Remove the newline character at the end
                return Some(stdout);
            }
        }
        _ => {}
    };
    None
}
