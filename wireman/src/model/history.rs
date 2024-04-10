use super::{headers::MetaHeaders, MessagesModel};
use crate::term::Term;
use config::Config;
use core::MethodDescriptor;
use logger::Logger;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    error::Error,
    path::{Path, PathBuf},
    str::FromStr,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone)]
pub struct HistoryModel {
    /// The filepath where the files are stored
    base_path: PathBuf,

    /// The selected save spot (1-5)
    save_spot: usize,

    /// Whether history is enabled
    pub(crate) disabled: bool,

    /// Whether autosave is enabled
    pub(crate) autosave: bool,
}

impl Default for HistoryModel {
    fn default() -> Self {
        Self {
            base_path: PathBuf::default(),
            save_spot: 1,
            disabled: false,
            autosave: false,
        }
    }
}

impl HistoryModel {
    pub fn new(env: &Config) -> Result<Self> {
        let path = PathBuf::from_str(&env.history.directory_expanded()).map_err(|err| {
            Term::stop().unwrap();
            err
        })?;
        Ok(Self {
            base_path: path,
            save_spot: 1,
            disabled: env.history.disabled,
            autosave: env.history.autosave,
        })
    }

    /// Get the currently selected save spot.
    pub fn save_spot(&self) -> usize {
        self.save_spot
    }

    /// Returns which of the 5 save spots are enabled
    pub fn save_spots_enabled(&self, method: &MethodDescriptor) -> Vec<bool> {
        (1..=5)
            .map(|i| {
                let Some(path) = self.path(i, method).clone() else {
                    return false;
                };
                path.exists()
            })
            .collect()
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

        let Some(path) = self.path(self.save_spot, method).clone() else {
            return;
        };

        if let Some(dir) = path.parent() {
            if std::fs::create_dir_all(dir).is_err() {
                Logger::debug(format!("history: cannot create dir: {dir:?}"));
                return;
            }
        } else {
            Logger::debug(format!("history: no parent dir found: {path:?}",));
            return;
        }

        let address = messages.headers_model.borrow().address();
        let auth_str = messages.headers_model.borrow().auth.value();
        let auth = Option::from(!auth_str.is_empty()).map(|_| auth_str);
        let metadata = messages.headers_model.borrow().meta.as_btree();
        let request = HistoryData {
            message,
            address,
            authentication: auth,
            metadata,
        };

        match serde_json::to_string_pretty(&request) {
            Ok(data) => {
                std::fs::write(path, data).unwrap_or_else(|_| {
                    Logger::debug("history: unable to write file");
                });
            }
            Err(_) => {
                Logger::debug("history: unable to convert to json");
            }
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
        let path = self.path(self.save_spot(), method).clone()?;
        if !path.exists() {
            return None;
        }
        let Ok(content) = std::fs::read_to_string(path.clone()) else {
            Logger::debug(format!("history: failed to read file {path:?}"));
            return None;
        };
        let history: HistoryData = if let Ok(history) = serde_json::from_str(&content) {
            history
        } else {
            Logger::debug("history: failed to parse from str");
            return None;
        };
        history.apply(messages);
        Some(())
    }

    /// Deletes a save spot
    pub fn delete(&self, method: &MethodDescriptor) {
        let Some(path) = self.path(self.save_spot, method).clone() else {
            return;
        };
        let _ = std::fs::remove_file(path);
    }

    /// Select a history save spot.
    pub fn select(&mut self, index: usize) {
        self.save_spot = index;
    }

    /// Convenience method to construct a path from a method
    /// Fails if the history base folder does not exist.
    /// If the method sub-folder does not exist, it is created.
    fn path(&self, save_spot: usize, method: &MethodDescriptor) -> Option<PathBuf> {
        if !Path::new(&self.base_path).exists() {
            let p = self.base_path.to_str().unwrap_or("");
            Logger::debug(format!("failed to save history: path {p} does not exist"));
            return None;
        }
        let path = self.base_path.join(method.full_name());
        let fname = format!("{save_spot}.json");
        Some(path.join(PathBuf::from(fname)))
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct HistoryData {
    pub message: String,
    pub address: String,
    pub authentication: Option<String>,
    pub metadata: BTreeMap<String, String>,
}

impl HistoryData {
    pub fn new(
        message: String,
        address: String,
        authentication: Option<String>,
        metadata: BTreeMap<String, String>,
    ) -> Self {
        Self {
            message,
            address,
            authentication,
            metadata,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| {
            Logger::debug("history: failed to parse to json");
            String::new()
        })
    }

    /// Applies a history.
    fn apply(&self, messages: &mut MessagesModel) {
        let mut headers_model = messages.headers_model.borrow_mut();
        headers_model.clear();
        headers_model.addr.set_text_raw(&self.address);
        if let Some(auth) = &self.authentication {
            headers_model.auth.set_text(auth);
        } else {
            headers_model.auth.set_text("");
        }
        headers_model.meta.set_btree(&self.metadata);
        if headers_model.meta.headers.is_empty() {
            headers_model.meta = MetaHeaders::default();
        }
        messages.request.editor.set_text_raw(&self.message);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_history_data_to_json() {
        // given
        let mut metadata = BTreeMap::new();
        metadata.insert("key1".to_string(), "value1".to_string());
        metadata.insert("key2".to_string(), "value2".to_string());

        let history_data = HistoryData {
            message: "Test message".to_string(),
            address: "Test address".to_string(),
            authentication: Some("Bearer Test".to_string()),
            metadata,
        };

        // when
        let pretty_json = history_data.to_json();

        //  then
        let expected_pretty_json = r#"{
  "message": "Test message",
  "address": "Test address",
  "authentication": "Bearer Test",
  "metadata": {
    "key1": "value1",
    "key2": "value2"
  }
}"#;
        assert_eq!(pretty_json, expected_pretty_json);
    }

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
        history_data.apply(&mut messages);

        //  then
        let expected_message = r#"Test message"#;
        assert_eq!(messages.request.editor.get_text_raw(), expected_message);

        let expected_address = "Test address";
        assert_eq!(messages.headers_model.borrow().address(), expected_address);

        let expected_auth = "Bearer test";
        assert_eq!(messages.headers_model.borrow().auth.value(), expected_auth);
    }
}
