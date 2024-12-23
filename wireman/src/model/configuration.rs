use std::str::FromStr;

use config::Config;
use logger::Logger;

use crate::widgets::editor::TextEditor;

#[derive(Clone)]
pub(crate) enum Message {
    Info(String),
    Success(String),
    Error(String),
}

#[derive(Clone)]
pub(crate) struct ConfigurationModel {
    /// The editor for the configuration.
    pub editor: Option<TextEditor>,
    /// The path to the configuration file.
    pub file_path: String,
    /// Display a info/error message.
    pub message: Option<Message>,
}

impl ConfigurationModel {
    pub fn new(file_path: String) -> Self {
        Self {
            editor: None,
            file_path,
            message: None,
        }
    }

    /// Toggles the configuration mode.
    pub fn toggle(&mut self) {
        if self.editor.is_some() {
            self.editor = None;
        } else {
            let mut editor = TextEditor::new();
            let content = Config::read_to_string(&self.file_path).unwrap();
            editor.set_text_raw(&content);
            self.editor = Some(editor);
            self.message = Some(Message::Info(String::from(&self.file_path)));
        }
    }

    /// Whether the configuration dialog is toggled
    pub fn toggled(&self) -> bool {
        self.editor.is_some()
    }

    /// Save the file
    pub fn save_to_file(&mut self) -> Option<Config> {
        let editor = self.editor.clone()?;

        let data = editor.get_text_raw();
        match Config::from_str(&data) {
            Ok(config) => {
                if let Err(err) = std::fs::write(&self.file_path, data) {
                    let err = format!("failed to write to file: {err}");
                    Logger::critical(&err);
                    self.message = Some(Message::Error(err));
                    return None;
                }
                self.message = Some(Message::Success(String::from("Successfully saved")));
                Some(config)
            }
            Err(err) => {
                let err = format!("failed to parse config: {err}");
                Logger::critical(&err);
                self.message = Some(Message::Error(err));
                None
            }
        }
    }
}
