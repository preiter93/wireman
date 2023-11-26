use core::MethodDescriptor;
use std::path::{Path, PathBuf};

use crate::commons::debug::log;

use super::MessagesModel;

#[derive(Clone)]
pub struct HistoryModel {
    // The filepath where the files are stored
    base_path: PathBuf,
    // The the selected save spot
    save_spot: usize,
}

impl HistoryModel {
    pub fn new(path: PathBuf) -> Self {
        Self {
            base_path: path,
            save_spot: 1,
        }
    }

    /// Saves a request message.
    pub fn save(&self, messages: &MessagesModel) {
        let method = match &messages.selected_method {
            Some(method) => method,
            None => {
                log("history: no method selected");
                return;
            }
        };

        let path = match self.path_from_method(method).clone() {
            Some(path) => path,
            None => return,
        };

        let request = match messages.build_request(method) {
            Ok(request) => request,
            Err(_) => {
                log("history: failed to build request");
                return;
            }
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

    /// Select a history save spot.
    pub fn select(&mut self, index: usize) {
        self.save_spot = index;
    }

    /// Convenience method to construct a path from a method
    /// Fails if the history base folder does not exist.
    /// If the method sub-folder does not exist, it is created.
    fn path_from_method(&self, method: &MethodDescriptor) -> Option<PathBuf> {
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
        let fname = format!("{}.json", self.save_spot);
        Some(path.join(PathBuf::from(fname)))
    }

    // pub fn load(&mut self, method: &MethodDescriptor) {
    //     self.clear();
    //     let files = list_files(&self.path);
    //     for file in files {
    //         if file.ends_with(method.full_name()) {
    //             self.items.push(file);
    //         }
    //     }
    // }
    //
    // pub fn apply(&self, messages: &mut MessagesModel) {
    //     // if let Some(index) = self.selected {
    //     // let item = &self.items[index];
    //     // if item == "Default" {
    //     //     messages.apply_template();
    //     // } else {
    //     messages.apply_history(&HistoryData::read_from_file(self.selected));
    //     // }
    //     // }
    // }

    // pub fn delete_selected(&mut self) {
    //     if let Some(index) = self.selected {
    //         let item = &self.items[index];
    //         if item == "Default" {
    //             return;
    //         }
    //         let res = std::fs::remove_file(item);
    //         if res.is_ok() {
    //             self.items.remove(index);
    //             if index >= self.items.len() {
    //                 self.selected = Some(index - 1);
    //             }
    //         }
    //     }
    // }

    // fn clear(&mut self) {
    //     self.items.clear();
    //     self.items.push("Default".to_string());
    //     self.selected = None;
    // }

    // fn filename_from_method(method: &MethodDescriptor) -> String {
    //     let method_name = method.full_name();
    //     let time: String = Utc::now().to_rfc3339();
    //     format!("{}_{}", time, method_name)
    // }

    // pub fn as_widget(&self) -> List<'_, ListItem<'_>> {
    //     let items = self
    //         .items
    //         .iter()
    //         .map(|e| ListItem::new(Span::from(e.as_str())))
    //         .collect::<Vec<_>>();
    //     let mut widget = List::new(items);
    //     widget.state.select(self.selected);
    //     widget
    // }

    // pub fn next(&mut self) {
    //     let i = match self.selected {
    //         Some(i) => {
    //             if i >= self.items.len() - 1 {
    //                 0
    //             } else {
    //                 i + 1
    //             }
    //         }
    //         None => 0,
    //     };
    //     self.selected = Some(i);
    // }
    //
    // pub fn previous(&mut self) {
    //     let i = match self.selected {
    //         Some(i) => {
    //             if i == 0 {
    //                 self.items.len() - 1
    //             } else {
    //                 i - 1
    //             }
    //         }
    //         None => 0,
    //     };
    //     self.selected = Some(i);
    // }
}

// fn list_files(dir: &PathBuf) -> Vec<String> {
//     let paths = std::fs::read_dir(dir).unwrap();
//     let mut files = Vec::new();
//     for path in paths {
//         let path_str = path.unwrap().path().to_str().unwrap().to_string();
//         files.push(path_str);
//     }
//     files.sort();
//     files
// }

// #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
// pub struct HistoryData {
//     pub message: String,
//     pub address: String,
//     pub metadata: HashMap<String, String>,
// }
//
// impl HistoryData {
//     pub fn from_model(model: &MessagesModel) -> Self {
//         let metadata = model.metadata_model.borrow().as_raw();
//         let address = model.headers_model.borrow().address.get_text_raw();
//         let message = model.to_json();
//         Self {
//             metadata,
//             address,
//             message,
//         }
//     }
//
//     // pub fn write_to_file(&self, path: &PathBuf) {
//     //     match serde_json::to_string_pretty(self) {
//     //         Ok(data) => {
//     //             let content = format!("{}", data);
//     //             std::fs::write(path, content.clone()).unwrap_or_else(|_| {
//     //                 log(format!("failed to save history: unable to write file"));
//     //             });
//     //         }
//     //         Err(_) => {
//     //             log(format!("failed to save history: unable to convert to json",));
//     //         }
//     //     }
//     // }
//
//     pub fn read_from_file(path: &str) -> Self {
//         let data = std::fs::read_to_string(path).expect("Unable to read file");
//         serde_json::from_str(&data).unwrap()
//     }
// }
