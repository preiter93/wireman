use super::{headers::MetaHeaders, MessagesModel};
use std::path::PathBuf;
use wireman_config::Config;
use wireman_core::{
    features::{is_auth_command, History, HistoryData},
    MethodDescriptor,
};
use wireman_logger::Logger;

#[derive(Clone)]
pub struct HistoryModel {
    /// The stored requests
    history: History,

    /// The selected save spot (1-5)
    save_spot: usize,

    /// Whether history is disabled
    pub(crate) disabled: bool,

    /// Whether autosave is enabled
    pub(crate) autosave: bool,

    /// Whether a literal authentication header is saved
    unsafe_save_auth_header: bool,
}

impl Default for HistoryModel {
    fn default() -> Self {
        Self {
            history: History::default(),
            save_spot: 1,
            disabled: true,
            autosave: false,
            unsafe_save_auth_header: false,
        }
    }
}

impl HistoryModel {
    pub fn new(env: &Config) -> Self {
        Self {
            history: History::new(PathBuf::from(env.history.directory_expanded())),
            save_spot: 1,
            disabled: env.history.disabled,
            autosave: env.history.autosave,
            unsafe_save_auth_header: env.history.unsafe_save_auth_header,
        }
    }

    /// Get the currently selected save spot.
    pub fn save_spot(&self) -> usize {
        self.save_spot
    }

    /// Returns which of the 5 save spots are enabled
    pub fn save_spots_enabled(&self, method: &MethodDescriptor) -> Vec<bool> {
        (1..=5).map(|i| self.history.exists(method, i)).collect()
    }

    /// Saves a request message to history.
    pub fn save(&self, messages: &MessagesModel) {
        if self.disabled {
            return;
        }

        let Some(method) = &messages.selected_method else {
            Logger::debug("history: no method selected");
            return;
        };

        let Ok(message) = messages.request.editor.get_text_json() else {
            Logger::debug("history: failed to parse request");
            return;
        };

        let address = messages.headers.borrow().address();
        let auth_str = messages.headers.borrow().auth.value();
        let auth = saved_authentication(&auth_str, self.unsafe_save_auth_header);
        let metadata = messages.headers.borrow().meta.as_btree();
        let request = HistoryData {
            message,
            address,
            authentication: auth,
            metadata,
        };

        if let Err(err) = self.history.save(method, self.save_spot, &request) {
            Logger::debug(err.to_string());
        }
    }

    /// Loads a request from history.
    pub fn load(&self, messages: &mut MessagesModel) -> Option<()> {
        if self.disabled {
            return None;
        }

        let Some(method) = &messages.selected_method else {
            Logger::debug("history: no method selected");
            return None;
        };

        let request = match self.history.load(method, self.save_spot) {
            Ok(request) => request?,
            Err(err) => {
                Logger::debug(err.to_string());
                return None;
            }
        };
        apply_history(&request, messages);
        Some(())
    }

    /// Deletes a save spot
    pub fn delete(&self, method: &MethodDescriptor) {
        if let Err(err) = self.history.delete(method, self.save_spot) {
            Logger::debug(err.to_string());
        }
    }

    /// Select a history save spot.
    pub fn select(&mut self, index: usize) {
        self.save_spot = index;
    }
}

/// Returns the authentication header to save. A literal token is only saved
fn saved_authentication(auth: &str, unsafe_save_auth_header: bool) -> Option<String> {
    if auth.is_empty() {
        return None;
    }

    (is_auth_command(auth) || unsafe_save_auth_header).then(|| auth.to_string())
}

/// Applies a history to the messages and headers.
fn apply_history(history: &HistoryData, messages: &mut MessagesModel) {
    let mut headers = messages.headers.borrow_mut();

    // Literal tokens are not saved, so a history without one keeps the current one.
    let authentication = history
        .authentication
        .clone()
        .unwrap_or_else(|| headers.auth.value());

    headers.clear();
    headers.addr.set_text_raw(&history.address);
    headers.auth.set_text(&authentication);
    headers.meta.set_btree(&history.metadata);
    if headers.meta.headers.is_empty() {
        headers.meta = MetaHeaders::default();
    }
    messages.request.editor.set_text_raw(&history.message);
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_history_apply() {
        struct TestCase {
            name: &'static str,
            given_current_auth: &'static str,
            given_saved_auth: Option<&'static str>,
            expected_auth: &'static str,
        }

        let test_cases = vec![
            TestCase {
                name: "success: saved auth is loaded",
                given_current_auth: "Bearer current",
                given_saved_auth: Some("Bearer saved"),
                expected_auth: "Bearer saved",
            },
            TestCase {
                name: "success: missing auth keeps the current one",
                given_current_auth: "Bearer current",
                given_saved_auth: None,
                expected_auth: "Bearer current",
            },
        ];

        for test_case in test_cases {
            let history_data = HistoryData {
                message: "Test message".to_string(),
                address: "Test address".to_string(),
                authentication: test_case.given_saved_auth.map(str::to_string),
                metadata: BTreeMap::from([("key".to_string(), "value".to_string())]),
            };
            let mut messages = MessagesModel::default();
            messages
                .headers
                .borrow_mut()
                .auth
                .set_text(test_case.given_current_auth);

            apply_history(&history_data, &mut messages);

            assert_eq!(
                messages.request.editor.get_text_raw(),
                "Test message",
                "{}",
                test_case.name
            );
            assert_eq!(
                messages.headers.borrow().address(),
                "Test address",
                "{}",
                test_case.name
            );
            assert_eq!(
                messages.headers.borrow().auth.value(),
                test_case.expected_auth,
                "{}",
                test_case.name
            );
        }
    }

    #[test]
    fn test_saved_authentication() {
        struct TestCase {
            name: &'static str,
            given_auth: &'static str,
            given_unsafe_save_auth_header: bool,
            expected_auth: Option<&'static str>,
        }

        let test_cases = vec![
            TestCase {
                name: "success: literal token is not saved",
                given_auth: "Bearer eyJhbGciOi",
                given_unsafe_save_auth_header: false,
                expected_auth: None,
            },
            TestCase {
                name: "success: command is saved",
                given_auth: "Bearer $(getToken.sh)",
                given_unsafe_save_auth_header: false,
                expected_auth: Some("Bearer $(getToken.sh)"),
            },
            TestCase {
                name: "success: literal token is saved when allowed",
                given_auth: "Basic dXNlcjpwYXNz",
                given_unsafe_save_auth_header: true,
                expected_auth: Some("Basic dXNlcjpwYXNz"),
            },
            TestCase {
                name: "success: empty auth is not saved",
                given_auth: "",
                given_unsafe_save_auth_header: true,
                expected_auth: None,
            },
        ];

        for test_case in test_cases {
            let auth = saved_authentication(
                test_case.given_auth,
                test_case.given_unsafe_save_auth_header,
            );

            assert_eq!(
                auth.as_deref(),
                test_case.expected_auth,
                "{}",
                test_case.name
            );
        }
    }
}
