use chrono::Utc;
use core::MethodDescriptor;
use ratatui::text::Span;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use tui_widget_list::WidgetList;

use crate::{commons::debug::log_to_file, widgets::list::ListItem};

use super::MessagesModel;

#[derive(Clone)]
pub struct HistoryModel {
    // The filepath where the files are stored
    path: PathBuf,
    // List of history files for the current method
    pub items: Vec<String>,
    // The index of the selected file
    pub selected: Option<usize>,
}

impl HistoryModel {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            items: vec!["Default".to_string()],
            selected: None,
        }
    }

    pub fn save(&self, messages: &MessagesModel) {
        let req = HistoryData::from_model(messages);
        if let Some(method) = &messages.selected_method {
            let path = self.path_from_method(method);
            req.write_to_file(path)
        }
    }

    pub fn load(&mut self, method: &MethodDescriptor) {
        self.clear();
        let files = list_files(&self.path);
        for file in files {
            if file.ends_with(method.full_name()) {
                log_to_file(file.clone());
                self.items.push(file);
            }
        }
    }

    pub fn apply(&self, messages: &mut MessagesModel) {
        if let Some(index) = self.selected {
            let item = &self.items[index];
            if item == "Default" {
                messages.apply_template();
            } else {
                messages.apply_history(&HistoryData::read_from_file(item));
            }
        }
    }

    pub fn delete_selected(&mut self) {
        if let Some(index) = self.selected {
            let item = &self.items[index];
            if item == "Default" {
                return;
            }
            let res = std::fs::remove_file(item);
            if res.is_ok() {
                self.items.remove(index);
                if index >= self.items.len() {
                    self.selected = Some(index - 1);
                }
            }
        }
    }

    fn clear(&mut self) {
        self.items.clear();
        self.items.push("Default".to_string());
        self.selected = None;
    }

    fn filename_from_method(method: &MethodDescriptor) -> String {
        let method_name = method.full_name();
        let time: String = Utc::now().to_rfc3339();
        format!("{}_{}", time, method_name)
    }

    fn path_from_method(&self, method: &MethodDescriptor) -> PathBuf {
        let fname = Self::filename_from_method(method);
        self.path.join(PathBuf::from(fname))
    }

    pub fn as_widget(&self) -> WidgetList<'_, ListItem<'_>> {
        let items = self
            .items
            .iter()
            .map(|e| ListItem::new(Span::from(e.as_str())))
            .collect::<Vec<_>>();
        let mut widget = WidgetList::new(items);
        widget.state.select(self.selected);
        widget
    }

    pub fn next(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    pub fn previous(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }
}

fn list_files(dir: &PathBuf) -> Vec<String> {
    let paths = std::fs::read_dir(dir).unwrap();
    let mut files = Vec::new();
    for path in paths {
        let path_str = path.unwrap().path().to_str().unwrap().to_string();
        files.push(path_str);
    }
    files.sort();
    files
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct HistoryData {
    pub message: String,
    pub address: String,
    pub metadata: HashMap<String, String>,
}

impl HistoryData {
    pub fn from_model(model: &MessagesModel) -> Self {
        let metadata = model.metadata_model.borrow().as_raw();
        let address = model.address_model.borrow().editor.get_text_raw();
        let message = model.request.editor.get_text_raw();
        Self {
            metadata,
            address,
            message,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn write_to_file(&self, path: PathBuf) {
        let data = self
            .to_json()
            .unwrap_or_else(|_| "Unable converting Request to json".to_string());
        std::fs::write(path, data).unwrap();
    }

    pub fn read_from_file(path: &str) -> Self {
        let data = std::fs::read_to_string(path).expect("Unable to read file");
        serde_json::from_str(&data).unwrap()
    }
}
