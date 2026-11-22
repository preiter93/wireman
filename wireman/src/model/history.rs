use super::{headers::MetaHeaders, MessagesModel};
use std::path::PathBuf;
use wireman_config::Config;
use wireman_core::{
    features::{History, HistoryData},
    MethodDescriptor,
};
use wireman_logger::Logger;

#[derive(Clone)]
pub struct HistoryModel {
    /// The stored requests
    history: History,

    /// The selected save spot (1-5)
    save_spot: usize,

    /// Whether history is enabled
    pub(crate) enabled: bool,

    /// Whether autosave is enabled
    pub(crate) autosave: bool,
}

impl Default for HistoryModel {
    fn default() -> Self {
        Self {
            history: History::default(),
            save_spot: 1,
            enabled: true,
            autosave: false,
        }
    }
}

impl HistoryModel {
    pub fn new(env: &Config) -> Self {
        Self {
            history: History::new(PathBuf::from(env.history.directory_expanded())),
            save_spot: 1,
            enabled: env.history.disabled,
            autosave: env.history.autosave,
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
        if self.enabled {
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
        let auth = (!auth_str.is_empty()).then_some(auth_str);
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
        if self.enabled {
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

/// Applies a history to the messages and headers.
fn apply_history(history: &HistoryData, messages: &mut MessagesModel) {
    let mut headers = messages.headers.borrow_mut();
    headers.clear();
    headers.addr.set_text_raw(&history.address);
    if let Some(auth) = &history.authentication {
        headers.auth.set_text(auth);
    } else {
        headers.auth.set_text("");
    }
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
        // given
        let mut metadata = BTreeMap::new();
        metadata.insert("key1".to_string(), "value1".to_string());
        metadata.insert("key2".to_string(), "value2".to_string());

        let history_data = HistoryData {
            message: "Test message".to_string(),
            address: "Test address".to_string(),
            authentication: Some("Bearer test".to_string()),
            metadata,
        };

        // when
        let mut messages = MessagesModel::default();
        apply_history(&history_data, &mut messages);

        //  then
        let expected_message = r#"Test message"#;
        assert_eq!(messages.request.editor.get_text_raw(), expected_message);

        let expected_address = "Test address";
        assert_eq!(messages.headers.borrow().address(), expected_address);

        let expected_auth = "Bearer test";
        assert_eq!(messages.headers.borrow().auth.value(), expected_auth);
    }
}
