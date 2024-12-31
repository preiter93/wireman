use crate::widgets::editor::TextEditor;
use edtui::{EditorMode, Index2};
use std::collections::BTreeMap;

pub struct MetaHeaders {
    /// The key value pairs.
    pub(crate) headers: Vec<(TextEditor, TextEditor)>,

    /// The selected field. Headers are organized in a grid
    /// where the column represents the key (=0) and val (=1).
    pub(crate) selected: Option<Index2>,
}

impl Default for MetaHeaders {
    fn default() -> Self {
        Self {
            headers: vec![(TextEditor::single(), TextEditor::single())],
            selected: None,
        }
    }
}

impl MetaHeaders {
    pub fn len(&self) -> usize {
        self.headers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }

    pub fn first_row_selected(&self) -> bool {
        if self.headers.is_empty() {
            return true;
        }
        let Some(selected) = self.selected else {
            return false;
        };
        selected.row == 0
    }

    pub fn last_row_selected(&self) -> bool {
        if self.headers.is_empty() {
            return true;
        }
        let Some(selected) = self.selected else {
            return false;
        };
        selected.row == self.headers.len().saturating_sub(1)
    }

    pub fn next_row(&mut self) {
        if let Some(selected) = &mut self.selected {
            selected.row += 1;
            selected.row = selected.row.min(self.headers.len().saturating_sub(1));
        }
    }

    pub fn prev_row(&mut self) {
        if let Some(selected) = &mut self.selected {
            selected.row = selected.row.saturating_sub(1);
        }
    }

    pub fn next_col(&mut self) {
        if let Some(selected) = &mut self.selected {
            next_col(&mut selected.col);
        }
    }

    pub fn prev_col(&mut self) {
        if let Some(selected) = &mut self.selected {
            next_col(&mut selected.col);
        }
    }

    pub fn is_hidden(&self) -> bool {
        self.headers.is_empty()
    }

    pub(super) fn mode(&self) -> EditorMode {
        self.selected_editor()
            .map_or(EditorMode::Normal, |x| x.state.mode)
    }

    pub(crate) fn select(&mut self) {
        self.selected = Some(Index2::default());
    }

    pub(super) fn select_last(&mut self) {
        let row = self.headers.len().saturating_sub(1);
        self.selected = Some(Index2::new(row, 0));
    }

    pub(super) fn unselect(&mut self) {
        self.selected = None;
    }

    /// Block selection of next header in case we are inside
    /// the metadata list selection.
    pub(crate) fn block_next(&self) -> bool {
        if let Some(selected) = self.selected {
            return selected.row + 1 < self.headers.len();
        }
        false
    }

    /// Block selection of previos header in case we are inside
    /// the metadata list selection.
    pub(crate) fn block_prev(&self) -> bool {
        if let Some(selected) = self.selected {
            return selected.row != 0;
        }
        false
    }

    pub fn selected_editor(&self) -> Option<&TextEditor> {
        if let Some(selected) = self.selected {
            let pair = &self.headers[selected.row];
            if selected.col == 1 {
                return Some(&pair.1);
            }
            return Some(&pair.0);
        }
        None
    }

    pub fn selected_editor_mut<'b, 'a: 'b>(&'a mut self) -> Option<&'b mut TextEditor> {
        if let Some(selected) = self.selected {
            let pair = &mut self.headers[selected.row];
            if selected.col == 1 {
                return Some(&mut pair.1);
            }
            return Some(&mut pair.0);
        }
        None
    }

    /// Adds an empty header key value pair
    pub(crate) fn add(&mut self) {
        self.headers
            .push((TextEditor::single(), TextEditor::single()));
        self.select();
    }

    /// Removes a header key value pair.
    pub(crate) fn remove(&mut self, index: usize) {
        self.headers.remove(index);
        if self.headers.is_empty() {
            self.selected = None;
        }
        if let Some(selected) = &mut self.selected {
            selected.row = selected.row.saturating_sub(1);
            selected.col = 0;
        }
    }

    pub(crate) fn clear(&mut self) {
        self.headers.clear();
        self.selected = None;
    }

    pub(crate) fn as_btree(&self) -> BTreeMap<String, String> {
        self.headers
            .iter()
            .filter(|(key, _)| !key.is_empty())
            .map(|(key, val)| (key.get_text_raw(), val.get_text_raw()))
            .collect()
    }

    pub(crate) fn set_btree(&mut self, data: &BTreeMap<String, String>) {
        for (key, val) in data {
            let mut k = TextEditor::single();
            let mut v = TextEditor::single();
            k.set_text_raw(key);
            v.set_text_raw(val);
            self.headers.push((k, v));
        }
        self.selected = None;
    }
}

fn next_col(col: &mut usize) {
    *col = (*col + 1) % 2;
}
