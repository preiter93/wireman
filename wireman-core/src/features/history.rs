use crate::{error::Error, Result};
use prost_reflect::MethodDescriptor;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

/// A request saved in the history.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct HistoryData {
    pub message: String,
    pub address: String,
    pub authentication: Option<String>,
    pub metadata: BTreeMap<String, String>,
}

/// The history stores one request per method and save spot as a JSON file.
#[derive(Debug, Clone, Default)]
pub struct History {
    base_path: PathBuf,
}

impl History {
    /// Creates a history that stores its files in `base_path`.
    #[must_use]
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    /// Returns whether a request is saved for the method in the save spot.
    #[must_use]
    pub fn exists(&self, method: &MethodDescriptor, save_spot: usize) -> bool {
        self.path(method, save_spot).is_ok_and(|path| path.exists())
    }

    /// Saves a request for the method in the save spot.
    ///
    /// # Errors
    ///
    /// Errors if the history directory does not exist or the file cannot be written.
    pub fn save(
        &self,
        method: &MethodDescriptor,
        save_spot: usize,
        data: &HistoryData,
    ) -> Result<()> {
        let path = self.path(method, save_spot)?;

        if let Some(directory) = path.parent() {
            std::fs::create_dir_all(directory).map_err(|err| {
                Error::HistoryError(format!("cannot create {directory:?}: {err}"))
            })?;
        }

        let json = serde_json::to_string_pretty(data)
            .map_err(|err| Error::HistoryError(format!("cannot serialize {path:?}: {err}")))?;
        std::fs::write(&path, json)
            .map_err(|err| Error::HistoryError(format!("cannot write {path:?}: {err}")))
    }

    /// Loads the request of the method in the save spot. Returns `None` if
    /// nothing is saved there.
    ///
    /// # Errors
    ///
    /// Errors if the history directory does not exist or the file cannot be read.
    pub fn load(&self, method: &MethodDescriptor, save_spot: usize) -> Result<Option<HistoryData>> {
        let path = self.path(method, save_spot)?;
        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|err| Error::HistoryError(format!("cannot read {path:?}: {err}")))?;
        let data = serde_json::from_str(&content)
            .map_err(|err| Error::HistoryError(format!("cannot parse {path:?}: {err}")))?;

        Ok(Some(data))
    }

    /// Deletes the request of the method in the save spot.
    ///
    /// # Errors
    ///
    /// Errors if the history directory does not exist or the file cannot be removed.
    pub fn delete(&self, method: &MethodDescriptor, save_spot: usize) -> Result<()> {
        let path = self.path(method, save_spot)?;
        std::fs::remove_file(&path)
            .map_err(|err| Error::HistoryError(format!("cannot delete {path:?}: {err}")))
    }

    fn path(&self, method: &MethodDescriptor, save_spot: usize) -> Result<PathBuf> {
        if !self.base_path.exists() {
            return Err(Error::HistoryError(format!(
                "directory {:?} does not exist",
                self.base_path
            )));
        }

        let file_name = format!("{save_spot}.json");
        Ok(self.base_path.join(method.full_name()).join(file_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProtoDescriptor;

    #[test]
    fn test_save_and_load() {
        struct TestCase {
            name: &'static str,
            given_base_path: PathBuf,
            expected_error: bool,
            expected_data: Option<HistoryData>,
        }

        let base_path =
            std::env::temp_dir().join(format!("wireman-history-test-{}", std::process::id()));
        std::fs::create_dir_all(&base_path).unwrap();

        let test_cases = vec![
            TestCase {
                name: "success: round trip",
                given_base_path: base_path.clone(),
                expected_error: false,
                expected_data: Some(test_data()),
            },
            TestCase {
                name: "failure: base dir missing",
                given_base_path: base_path.join("missing"),
                expected_error: true,
                expected_data: None,
            },
        ];

        let descriptor = ProtoDescriptor::new(vec!["."], vec!["test_files/test.proto"]).unwrap();
        let method = descriptor
            .get_method_by_name("proto.TestService", "Simple")
            .unwrap();

        for test_case in test_cases {
            let history = History::new(test_case.given_base_path);

            let result = history
                .save(&method, 1, &test_data())
                .and_then(|()| history.load(&method, 1));

            assert_eq!(
                result.is_err(),
                test_case.expected_error,
                "{}",
                test_case.name
            );
            assert_eq!(
                result.unwrap_or_default(),
                test_case.expected_data,
                "{}",
                test_case.name
            );
        }

        std::fs::remove_dir_all(base_path).unwrap();
    }

    #[test]
    fn test_load_existing_file_format() {
        let base_path = std::env::temp_dir().join(format!(
            "wireman-history-format-test-{}",
            std::process::id()
        ));
        let directory = base_path.join("proto.TestService.Simple");
        std::fs::create_dir_all(&directory).unwrap();
        let given_file = r#"{
  "message": "{\"number\":5}",
  "address": "http://localhost:50051",
  "authentication": "Bearer token",
  "metadata": {
    "key": "value"
  }
}"#;
        std::fs::write(directory.join("1.json"), given_file).unwrap();

        let descriptor = ProtoDescriptor::new(vec!["."], vec!["test_files/test.proto"]).unwrap();
        let method = descriptor
            .get_method_by_name("proto.TestService", "Simple")
            .unwrap();
        let data = History::new(base_path.clone()).load(&method, 1).unwrap();

        assert_eq!(data, Some(test_data()));
        std::fs::remove_dir_all(base_path).unwrap();
    }

    fn test_data() -> HistoryData {
        HistoryData {
            message: String::from("{\"number\":5}"),
            address: String::from("http://localhost:50051"),
            authentication: Some(String::from("Bearer token")),
            metadata: BTreeMap::from([(String::from("key"), String::from("value"))]),
        }
    }
}
