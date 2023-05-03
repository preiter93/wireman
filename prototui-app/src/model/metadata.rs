use crate::{commons::editor::TextEditor, widgets::key_value::KeyValueWidget};
use std::collections::HashMap;
use tui_widget_list::SelectableWidgetList;

/// The metadata model
pub struct MetadataModel<'a> {
    /// Holds the key value editors and the selection state
    pub content: SelectableWidgetList<'a, KeyValueWidget<'a>>,
}

impl<'a> MetadataModel<'a> {
    pub fn new() -> Self {
        let items = vec![KeyValueWidget::new()];

        let mut content = SelectableWidgetList::new(items);
        content.state.select(Some(0));
        Self { content }
    }

    pub fn collect(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for item in &self.content.items {
            let key = item.get_key().get_text_raw();
            let val = item.get_val().get_text_raw();
            if !key.is_empty() {
                map.insert(key, val);
            }
        }
        map
    }

    pub fn select_key(&mut self) {
        self.content
            .items
            .iter_mut()
            .for_each(KeyValueWidget::select_key);
    }

    pub fn select_val(&mut self) {
        self.content
            .items
            .iter_mut()
            .for_each(KeyValueWidget::select_val);
    }

    pub fn is_key_selected(&self) -> bool {
        self.content
            .get_selected()
            .map_or(false, KeyValueWidget::is_key_selected)
    }

    pub fn get_selected(&self) -> Option<&'_ TextEditor<'a>> {
        self.content
            .get_selected()
            .map(KeyValueWidget::get_selected)
    }

    pub fn get_selected_mut(&mut self) -> Option<&'_ mut TextEditor<'a>> {
        self.content
            .get_selected_mut()
            .map(KeyValueWidget::get_selected_mut)
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
