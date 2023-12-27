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
    pub addr: TextEditor,

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
            addr: address,
            auth: AuthHeader::default(),
            meta: MetaHeaders::default(),
            selected: HeadersSelection::Auth,
        }
    }
    /// Get the address as a string
    pub fn address(&self) -> String {
        self.addr.get_text_raw()
    }

    /// Returns the editor mode
    pub fn mode(&self) -> EditorMode {
        [self.auth.mode(), self.addr.state.mode, self.meta.mode()]
            .into_iter()
            .find(|&x| x != EditorMode::Normal)
            .unwrap_or(EditorMode::Normal)
    }

    /// Get the headers as a map
    pub fn headers(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        // Authorization
        if !self.auth.is_empty() {
            map.insert(AuthHeader::key(), self.auth.value());
        }

        // Metadata
        for (key, val) in self.meta.headers_raw() {
            if !key.is_empty() {
                let _ = map.insert(key, val);
            }
        }
        map
    }

    /// Get the next header tab
    pub fn next(&mut self) -> HeadersSelection {
        match self.selected {
            HeadersSelection::Auth => {
                if self.meta.headers().is_empty() {
                    return HeadersSelection::Addr;
                }
                self.meta.select();
                HeadersSelection::Meta
            }
            HeadersSelection::Addr => HeadersSelection::Auth,
            HeadersSelection::Meta => {
                if self.meta.block_next() {
                    return HeadersSelection::Meta;
                }
                self.meta.unselect();
                HeadersSelection::Addr
            }
        }
    }

    /// Get the previous header tab
    pub fn prev(&mut self) -> HeadersSelection {
        match self.selected {
            HeadersSelection::Addr => {
                if self.meta.headers().is_empty() {
                    return HeadersSelection::Auth;
                }
                self.meta.select();
                HeadersSelection::Meta
            }
            HeadersSelection::Auth => HeadersSelection::Addr,
            HeadersSelection::Meta => {
                if self.meta.block_prev() {
                    return HeadersSelection::Meta;
                }
                self.meta.unselect();
                HeadersSelection::Auth
            }
        }
    }
    /// Clears headers
    pub fn clear(&mut self) {
        self.auth.clear();
        self.meta.clear();
    }
}

/// The selection state of `HeadersModel`.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum HeadersSelection {
    Addr,
    #[default]
    Auth,
    Meta,
}

impl HeadersSelection {
    pub fn next(&self) -> Self {
        match &self {
            Self::Auth => Self::Meta,
            Self::Addr => Self::Auth,
            Self::Meta => Self::Addr,
        }
    }

    pub fn prev(&self) -> Self {
        match &self {
            Self::Addr => Self::Meta,
            Self::Auth => Self::Addr,
            Self::Meta => Self::Auth,
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
