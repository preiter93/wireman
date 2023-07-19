use chrono::Utc;
use core::MethodDescriptor;
use ratatui::text::Span;
use tui_widget_list::SelectableWidgetList;

use crate::{commons::debug::log_to_file, widgets::list::ListItem};

use super::{request::Request, MessagesModel};

static DIR_HISTORY: &str = "history";

#[derive(Clone)]
pub struct HistoryModel {
    // List of history files for the current method
    pub items: Vec<String>,
    // The index of the selected file
    pub selected: Option<usize>,
    // // The loaded request
    // pub request: Option<Request>,
}

impl HistoryModel {
    pub fn new() -> Self {
        Self {
            items: vec!["Test".to_string()],
            selected: None,
        }
    }

    pub fn save_history(&self, messages: &MessagesModel) {
        let req = Request::from_model(messages);
        if let Some(method) = &messages.selected_method {
            let path = Self::path_from_method(method);
            req.write_to_file(&path)
        }
    }

    pub fn load_history(&mut self, method: &MethodDescriptor) {
        self.items.clear();

        let files = list_files();
        for file in files {
            if file.ends_with(method.full_name()) {
                log_to_file(file.clone());
                self.items.push(file);
            }
        }
    }

    pub fn apply_history(&self, messages: &mut MessagesModel) {
        if let Some(selected) = self.selected {
            let req = Request::read_from_file(&self.items[selected]);
            req.set_model(messages);
        }
    }

    fn filename_from_method(method: &MethodDescriptor) -> String {
        let method_name = method.full_name();
        let time: String = Utc::now().to_rfc3339();
        format!("{}_{}", time, method_name)
    }

    pub fn path_from_method(method: &MethodDescriptor) -> String {
        let fname = Self::filename_from_method(method);
        format!("{}/{}", DIR_HISTORY, fname)
    }

    pub fn as_widget(&self) -> SelectableWidgetList<'_, ListItem<'_>> {
        let items = self
            .items
            .iter()
            .map(|e| ListItem::new(Span::from(e.as_str())))
            .collect::<Vec<_>>();
        let mut widget = SelectableWidgetList::new(items);
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

fn list_files() -> Vec<String> {
    let paths = std::fs::read_dir(DIR_HISTORY).unwrap();

    let mut files = Vec::new();
    log_to_file("path");
    for path in paths {
        let path_str = path.unwrap().path().to_str().unwrap().to_string();
        log_to_file(path_str.clone());
        files.push(path_str);
        // println!("Name: {}", path.unwrap().path().display())
    }
    files
}
