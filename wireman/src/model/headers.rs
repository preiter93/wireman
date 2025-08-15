pub mod auth;
pub use auth::{AuthHeader, AuthSelection};
pub mod meta;
use crate::{model::history::HistoryModel, widgets::editor::TextEditor};
use core::MethodDescriptor;
use edtui::EditorMode;
pub use meta::MetaHeaders;
use std::{cell::RefCell, collections::HashMap, process::Command, rc::Rc};

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
    pub tab: HeadersTab,

    /// The currently selected method
    pub selected_method: Option<MethodDescriptor>,

    /// The model for the request history.
    pub history: Rc<RefCell<HistoryModel>>,
}

impl Default for HeadersModel {
    fn default() -> Self {
        Self::new("", "", Rc::new(RefCell::new(HistoryModel::default())))
    }
}

impl HeadersModel {
    /// Create a new `HeadersModel` instance
    pub fn new(
        default_address: &str,
        default_auth_header: &str,
        history: Rc<RefCell<HistoryModel>>,
    ) -> Self {
        let mut address = TextEditor::single();
        address.set_text_raw(default_address);
        let mut auth_header = AuthHeader::default();
        auth_header.set_text(default_auth_header);
        Self {
            addr: address,
            auth: auth_header,
            meta: MetaHeaders::default(),
            tab: HeadersTab::default(),
            selected_method: None,
            history,
        }
    }

    /// Get the address as a string
    pub fn address(&self) -> String {
        self.addr.get_text_raw()
    }

    /// Sets the selected method
    pub fn set_method(&mut self, method: &MethodDescriptor) {
        self.selected_method = Some(method.clone());
    }

    /// Get the selected editor
    pub fn selected_editor<'b, 'a: 'b>(&'a self) -> Option<&'b TextEditor> {
        match self.tab {
            HeadersTab::Addr => Some(&self.addr),
            HeadersTab::Auth => Some(self.auth.selected_editor()),
            HeadersTab::Meta => self.meta.selected_editor(),
            HeadersTab::None => None,
        }
    }

    /// Get the selected editor
    pub fn selected_editor_mut<'b, 'a: 'b>(&'a mut self) -> Option<&'b mut TextEditor> {
        match self.tab {
            HeadersTab::Addr => Some(&mut self.addr),
            HeadersTab::Auth => Some(self.auth.selected_editor_mut()),
            HeadersTab::Meta => self.meta.selected_editor_mut(),
            HeadersTab::None => None,
        }
    }
    /// Whether any editor is currently in insert mode
    pub fn disabled_root_events(&self) -> bool {
        self.selected_editor()
            .is_some_and(|editor| editor.state.mode != EditorMode::Normal)
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

    /// Get the shell expanded authentication headers as a map
    pub fn auth_headers_expanded(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        // Authorization
        if !self.auth.is_empty() {
            map.insert(AuthHeader::key(), self.auth.value_expanded());
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
                let key = try_expand(&key.get_text_raw());
                let val = try_expand(&val.get_text_raw());
                let _ = map.insert(key, val);
            }
        }
        map
    }

    /// Get the next header tab
    /// TODO: Simplify
    pub fn next_tab(&mut self) -> HeadersTab {
        match self.tab {
            HeadersTab::None => HeadersTab::Addr,
            HeadersTab::Addr => HeadersTab::Auth,
            HeadersTab::Auth => {
                if self.meta.is_hidden() {
                    return HeadersTab::Addr;
                }
                self.meta.select();
                HeadersTab::Meta
            }
            HeadersTab::Meta => {
                self.meta.unselect();
                HeadersTab::Addr
            }
        }
    }

    /// Get the previous header tab.
    /// TODO: Simplify
    pub fn prev_tab(&mut self) -> HeadersTab {
        match self.tab {
            HeadersTab::None | HeadersTab::Auth => HeadersTab::Addr,
            HeadersTab::Addr => {
                if self.meta.is_hidden() {
                    return HeadersTab::Auth;
                }
                self.meta.select_last();
                HeadersTab::Meta
            }
            HeadersTab::Meta => {
                self.meta.unselect();
                HeadersTab::Auth
            }
        }
    }

    pub fn next_row(&mut self) {
        if self.tab == HeadersTab::Meta && !self.meta.last_row_selected() {
            self.meta.next_row();
        } else {
            self.tab = self.next_tab();
        }
    }

    pub fn prev_row(&mut self) {
        if self.tab == HeadersTab::Meta && !self.meta.first_row_selected() {
            self.meta.prev_row();
        } else {
            self.tab = self.prev_tab();
        }
    }

    pub fn next_col(&mut self) {
        match self.tab {
            HeadersTab::Meta => self.meta.next_col(),
            HeadersTab::Auth => self.auth.next(),
            _ => (),
        }
    }

    pub fn prev_col(&mut self) {
        match self.tab {
            HeadersTab::Meta => self.meta.prev_col(),
            HeadersTab::Auth => self.auth.next(),
            _ => (),
        }
    }

    /// Clears the headers state.
    pub fn clear(&mut self) {
        self.auth.clear();
        self.meta.clear();
        self.tab = HeadersTab::None;
    }
}

/// The selection state of `HeadersModel`.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum HeadersTab {
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
