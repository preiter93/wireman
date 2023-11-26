use super::MessagesModel;
use crate::commons::debug::log;
use core::MethodDescriptor;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct HistoryModel {
    // The filepath where the files are stored
    base_path: PathBuf,
    // The selected save spot (1-5)
    save_spot: usize,
}

impl Default for HistoryModel {
    fn default() -> Self {
        Self::new(PathBuf::new())
    }
}

impl HistoryModel {
    pub fn new(path: PathBuf) -> Self {
        Self {
            base_path: path,
            save_spot: 1,
        }
    }

    /// Get the currently selected save spot.
    pub fn save_spot(&self) -> usize {
        self.save_spot
    }

    /// Returns which of the 5 save spots are enabled
    pub fn save_spots_enabled(&self, method: &MethodDescriptor) -> Vec<bool> {
        (1..=5)
            .map(|i| {
                let path = match self.path(i, method).clone() {
                    Some(path) => path,
                    None => return false,
                };
                path.exists()
            })
            .collect()
    }

    /// Saves a request message to history.
    pub fn save(&self, messages: &MessagesModel) {
        let method = match &messages.selected_method {
            Some(method) => method,
            None => {
                log("history: no method selected");
                return;
            }
        };

        let message = match messages.request.editor.get_text_json() {
            Ok(message) => message,
            Err(_) => {
                log("history: failed to parse request");
                return;
            }
        };

        let path = match self.path(self.save_spot, method).clone() {
            Some(path) => path,
            None => return,
        };

        let address = messages.headers_model.borrow().address();
        let bearer_str = messages.headers_model.borrow().bearer.get_text_raw();
        let bearer = Option::from(!bearer_str.is_empty()).and_then(|_| Some(bearer_str));
        let request = HistoryData {
            message,
            address,
            bearer,
            metadata: BTreeMap::new(),
        };

        match serde_json::to_string_pretty(&request) {
            Ok(data) => {
                std::fs::write(path, data).unwrap_or_else(|_| {
                    log(format!("history: unable to write file"));
                });
            }
            Err(_) => {
                log(format!("history: unable to convert to json",));
            }
        }
    }

    /// Loads a request from history.
    pub fn load(&self, messages: &mut MessagesModel) -> Option<()> {
        let method = match &messages.selected_method {
            Some(method) => method,
            None => {
                log("history: no method selected");
                return None;
            }
        };
        let path = match self.path(self.save_spot(), method).clone() {
            Some(path) => path,
            None => return None,
        };
        if !path.exists() {
            return None;
        }
        let content = match std::fs::read_to_string(path.clone()) {
            Ok(content) => content,
            Err(_) => {
                log(format!("history: failed to read file {:?}", path));
                return None;
            }
        };
        let history: HistoryData = match serde_json::from_str(&content) {
            Ok(history) => history,
            Err(_) => {
                log(format!("history: failed to parse from str"));
                return None;
            }
        };
        history.apply(messages);
        Some(())
    }

    /// Deletes a save spot
    pub fn delete(&self, method: &MethodDescriptor) {
        let path = match self.path(self.save_spot, method).clone() {
            Some(path) => path,
            None => return,
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
            log(format!("failed to save history: path {} does not exist", p));
            return None;
        }
        let path = self.base_path.join(method.full_name());
        std::fs::create_dir_all(&path).unwrap_or_else(|_| {
            let p = path.to_str().unwrap_or("");
            log(format!("failed to save history: cannot create dir: {}", p));
        });
        let fname = format!("{}.json", save_spot);
        Some(path.join(PathBuf::from(fname)))
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct HistoryData {
    pub message: String,
    pub address: String,
    pub bearer: Option<String>,
    pub metadata: BTreeMap<String, String>,
}

impl HistoryData {
    pub fn new(
        message: String,
        address: String,
        bearer: Option<String>,
        metadata: BTreeMap<String, String>,
    ) -> Self {
        Self {
            message,
            address,
            bearer,
            metadata,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| {
            log("history: failed to parse to json");
            String::new()
        })
    }

    /// Applies a history.
    fn apply(&self, messages: &mut MessagesModel) {
        let mut headers_model = messages.headers_model.borrow_mut();
        headers_model.address.set_text_raw(&self.address);
        if let Some(bearer) = &self.bearer {
            headers_model.bearer.set_text_raw(bearer);
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
            bearer: Some("Test bearer".to_string()),
            metadata,
        };

        // when
        let pretty_json = history_data.to_json();

        //  then
        let expected_pretty_json = r#"{
  "message": "Test message",
  "address": "Test address",
  "bearer": "Test bearer",
  "metadata": {
    "key1": "value1",
    "key2": "value2"
  },
}"#;
        assert_eq!(pretty_json, expected_pretty_json);
        // std::fs::write("file.txt", pretty_json).unwrap();
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
            bearer: Some("Test bearer".to_string()),
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
    }
}
