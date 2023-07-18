#![allow(clippy::module_name_repetitions)]
use crate::{commons::editor::TextEditor, widgets::key_value::KeyValue};
use std::collections::HashMap;
use tui_widget_list::SelectableWidgetList;

/// The metadata model
pub struct MetadataModel<'a> {
    /// The key value pairs
    items: Vec<KeyValue<'a>>,

    /// The currently selected key-value pair
    selected: Option<usize>,
}

impl<'a> MetadataModel<'a> {
    pub fn new() -> Self {
        Self {
            items: vec![KeyValue::default()],
            selected: Some(0),
        }
    }

    pub fn from_raw(map: &HashMap<String, String>) -> Self {
        let items = if map.is_empty() {
            vec![KeyValue::default()]
        } else {
            map.iter()
                .map(|(key, val)| KeyValue::new(key, val))
                .collect::<Vec<_>>()
        };
        Self {
            items,
            selected: Some(0),
        }
    }

    pub fn as_raw(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for item in &self.items {
            let key = item.get_key().get_text_raw();
            let val = item.get_val().get_text_raw();
            if !key.is_empty() {
                map.insert(key, val);
            }
        }
        map
    }

    /// Returns the metadata as a scrollable widget list
    pub fn as_widget(&self) -> SelectableWidgetList<'a, KeyValue<'a>> {
        let mut widget = SelectableWidgetList::new(self.items.clone());
        widget.state.select(self.selected);
        widget
    }

    pub fn select_key(&mut self) {
        if let Some(item) = self.selected.and_then(|index| self.items.get_mut(index)) {
            item.select_key();
        }
    }

    pub fn select_val(&mut self) {
        if let Some(item) = self.selected.and_then(|index| self.items.get_mut(index)) {
            item.select_val();
        }
    }

    pub fn is_key_selected(&self) -> bool {
        self.selected
            .and_then(|index| self.items.get(index))
            .map_or(false, KeyValue::is_key_selected)
    }

    pub fn get_selected(&self) -> Option<&'_ TextEditor<'a>> {
        self.selected
            .and_then(|index| self.items.get(index))
            .map(KeyValue::get_selected)
    }

    pub fn get_selected_mut(&mut self) -> Option<&'_ mut TextEditor<'a>> {
        self.selected
            .and_then(|index| self.items.get_mut(index))
            .map(KeyValue::get_selected_mut)
    }
}

// pub fn add_key_val(&mut self) {
//     let item = new_key_val_widget("", "");
//     self.content.items.push(item);
// }
//
// pub fn del_key_val(&mut self) {
//     if let Some(index) = self.content.state.selected() {
//         self.content.items.remove(index);
//         self.content.state.select(if self.content.items.is_empty() {
//             None
//         } else {
//             Some(index.saturating_sub(1))
//         });
//     }
// }
//
