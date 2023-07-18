use crate::model::{AddressModel, MetadataModel};

use super::MessagesModel;
use core::MethodDescriptor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static DIR_HISTORY: &str = "history";

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Request {
    pub message: String,
    pub address: String,
    pub metadata: HashMap<String, String>,
}

impl Request {
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

    pub fn write_to_file(&self, fname: &str) {
        let err = std::fs::create_dir_all(DIR_HISTORY);
        if err.is_ok() {
            let data = self
                .to_json()
                .unwrap_or_else(|_| "Unable converting Request to json".to_string());
            std::fs::write(fname, data).unwrap();
        }
    }

    pub fn read_from_file(fname: &str) -> Self {
        let data = std::fs::read_to_string(fname).expect("Unable to read file");
        serde_json::from_str(&data).unwrap()
    }

    pub fn set_model(&self, model: &mut MessagesModel) {
        *model.metadata_model.borrow_mut() = MetadataModel::from_raw(&self.metadata);
        *model.address_model.borrow_mut() = AddressModel::new(&self.address);
        model.request.editor.set_text_raw(&self.message);
    }

    pub fn fname_from_method(method: &MethodDescriptor) -> String {
        let method_name = method.full_name();
        format!("{}/{}", DIR_HISTORY, method_name)
    }
}
