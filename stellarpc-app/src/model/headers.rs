use std::{collections::HashMap, process::Command};

use crate::commons::editor::TextEditor;
use crossterm::event::KeyEvent;

/// The data model for the `gRPC` headers. Contains authorization
/// headers and metadata key value headers.
pub struct HeadersModel {
    /// The server address.
    pub address: TextEditor,
    /// The bearer token.
    pub auth: AuthHeader,
    /// The selection state.
    pub selected: HeadersSelection,
}

#[derive(Default)]
pub struct AuthHeader {
    pub(crate) bearer: TextEditor,
    pub(crate) basic: TextEditor,
    pub(crate) selected: AuthSelection,
}

/// The selection state of `HeadersModel`.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum AuthSelection {
    #[default]
    Bearer,
    Basic,
}

impl AuthHeader {
    pub fn is_empty(&self) -> bool {
        match self.selected {
            AuthSelection::Bearer => self.bearer.is_empty(),
            AuthSelection::Basic => self.basic.is_empty(),
        }
    }

    pub fn next(&mut self) {
        match self.selected {
            AuthSelection::Bearer => self.selected = AuthSelection::Basic,
            AuthSelection::Basic => self.selected = AuthSelection::Bearer,
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        match self.selected {
            AuthSelection::Bearer => self.bearer.on_key(key),
            AuthSelection::Basic => self.basic.on_key(key),
        }
    }

    pub fn insert_mode(&self) -> bool {
        self.bearer.insert_mode() || self.basic.insert_mode()
    }

    pub fn key(&self) -> String {
        "authorization".to_string()
    }

    pub fn value(&self) -> String {
        self._value(false)
    }

    pub fn value_expanded(&self) -> String {
        self._value(true)
    }

    fn _value(&self, expanded: bool) -> String {
        match self.selected {
            AuthSelection::Bearer => {
                let mut value = self.bearer.get_text_raw();
                if expanded {
                    value = try_expand(&value);
                };
                if value.is_empty() {
                    String::new()
                } else {
                    "Bearer ".to_owned() + &value
                }
            }
            AuthSelection::Basic => {
                let mut value = self.basic.get_text_raw();
                if expanded {
                    value = try_expand(&value);
                };
                if value.is_empty() {
                    String::new()
                } else {
                    "Basic ".to_owned() + &value
                }
            }
        }
    }

    pub fn set_text(&mut self, value: &str) {
        if value.starts_with("Bearer ") {
            self.bearer.set_text_raw(&value.replacen("Bearer ", "", 1));
            self.selected = AuthSelection::Bearer;
            return;
        }
        if value.starts_with("Basic ") {
            self.basic.set_text_raw(&value.replacen("Basic ", "", 1));
            self.selected = AuthSelection::Basic;
            return;
        }
    }
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
            selected: HeadersSelection::Auth,
        }
    }
    /// Get the address as a string
    pub fn address(&self) -> String {
        self.address.get_text_raw()
    }

    /// Get the headers as a map
    pub fn headers(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if !self.auth.is_empty() {
            map.insert(self.auth.key(), self.auth.value());
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
}
impl HeadersSelection {
    pub fn next(&self) -> Self {
        match &self {
            Self::None | Self::Auth => Self::Address,
            Self::Address => Self::Auth,
        }
    }

    pub fn prev(&self) -> Self {
        match &self {
            Self::None | Self::Address => Self::Auth,
            Self::Auth => Self::Address,
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
