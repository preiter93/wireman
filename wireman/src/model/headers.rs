pub mod auth;
pub use auth::{AuthHeader, AuthSelection};
pub mod meta;
use crate::widgets::editor::TextEditor;
use edtui::EditorMode;
pub use meta::MetaHeaders;
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
        let mut address = TextEditor::single();
        address.set_text_raw(default_address);
        Self {
            addr: address,
            auth: AuthHeader::default(),
            meta: MetaHeaders::default(),
            selected: HeadersSelection::default(),
        }
    }
    /// Get the address as a string
    pub fn address(&self) -> String {
        self.addr.get_text_raw()
    }

    /// Get the selected editor
    pub fn selected_editor_mut<'b, 'a: 'b>(&'a mut self) -> Option<&'b mut TextEditor> {
        match self.selected {
            HeadersSelection::Addr => Some(&mut self.addr),
            HeadersSelection::Auth => Some(self.auth.selected_editor_mut()),
            HeadersSelection::Meta => self.meta.selected_editor_mut(),
            _ => None,
        }
    }

    /// Returns the editor mode
    pub fn mode(&self) -> EditorMode {
        [self.auth.mode(), self.addr.state.mode, self.meta.mode()]
            .into_iter()
            .find(|&x| x != EditorMode::Normal)
            .unwrap_or(EditorMode::Normal)
    }

    /// Get the raw headers as a map
    pub fn headers(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        // Authorization
        if !self.auth.is_empty() {
            map.insert(AuthHeader::key(), self.auth.value());
        }

        // Metadata
        for (key, val) in &self.meta.headers {
            if !key.is_empty() {
                let _ = map.insert(key.get_text_raw(), val.get_text_raw());
            }
        }
        map
    }

    /// Get the shell expanded headers as a map
    pub fn headers_expanded(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        // Authorization
        if !self.auth.is_empty() {
            map.insert(AuthHeader::key(), self.auth.value_expanded());
        }

        // Metadata
        for (key, val) in &self.meta.headers {
            if !key.is_empty() {
                let _ = map.insert(key.get_text_raw(), val.get_text_raw());
            }
        }
        map
    }

    /// Get the next header tab
    /// TODO: Simplify
    pub fn next(&mut self) -> HeadersSelection {
        match self.selected {
            HeadersSelection::None => HeadersSelection::Addr,
            HeadersSelection::Addr => HeadersSelection::Auth,
            HeadersSelection::Auth => {
                if self.meta.is_hidden() {
                    return HeadersSelection::Addr;
                }
                self.meta.select();
                HeadersSelection::Meta
            }
            HeadersSelection::Meta => {
                self.meta.unselect();
                HeadersSelection::Addr
            }
        }
    }

    /// Get the previous header tab.
    /// TODO: Simplify
    pub fn prev(&mut self) -> HeadersSelection {
        match self.selected {
            HeadersSelection::None | HeadersSelection::Auth => HeadersSelection::Addr,
            HeadersSelection::Addr => {
                if self.meta.is_hidden() {
                    return HeadersSelection::Auth;
                }
                self.meta.select_last();
                HeadersSelection::Meta
            }
            HeadersSelection::Meta => {
                self.meta.unselect();
                HeadersSelection::Auth
            }
        }
    }

    /// Clears the headers state.
    pub fn clear(&mut self) {
        self.auth.clear();
        self.meta.clear();
        self.selected = HeadersSelection::None;
    }
}

/// The selection state of `HeadersModel`.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum HeadersSelection {
    #[default]
    None,
    Addr,
    Auth,
    Meta,
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
