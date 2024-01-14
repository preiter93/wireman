use std::collections::BTreeMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use edtui::{EditorMode, Index2};

use crate::commons::editor::TextEditor;

#[derive(Default)]
pub struct MetaHeaders {
    /// The key value pairs.
    pub(crate) headers: Vec<(TextEditor, TextEditor)>,

    /// The selected field. Headers are organized in a grid
    /// where the column represents the key (=0) and val (=1).
    pub(crate) selected: Option<Index2>,
}

impl MetaHeaders {
    pub fn on_key(&mut self, event: KeyEvent) {
        let navigation_enabled = self.mode() == EditorMode::Normal;
        match event.code {
            KeyCode::Right if navigation_enabled => {
                if let Some(selected) = &mut self.selected {
                    if selected.col == 0 {
                        selected.col = 1;
                    }
                }
            }
            KeyCode::Left if navigation_enabled => {
                if let Some(selected) = &mut self.selected {
                    if selected.col == 1 {
                        selected.col = 0;
                    }
                }
            }
            KeyCode::Up | KeyCode::Char('k') if navigation_enabled => {
                if let Some(selected) = &mut self.selected {
                    selected.row = selected.row.saturating_sub(1);
                }
            }
            KeyCode::Down | KeyCode::Char('j') if navigation_enabled => {
                if let Some(selected) = &mut self.selected {
                    selected.row += 1;
                    selected.row = selected.row.min(self.headers.len().saturating_sub(1));
                }
            }
            KeyCode::Char('h' | 'a')
                if event.modifiers == KeyModifiers::CONTROL && navigation_enabled =>
            {
                self.add();
            }
            KeyCode::Char('d')
                if event.modifiers == KeyModifiers::CONTROL && navigation_enabled =>
            {
                if let Some(index) = self.selected {
                    self.remove(index.row);
                }
            }
            _ => {
                if let Some(input) = self.selected_editor_mut() {
                    input.on_key(event, true);
                }
            }
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

    fn selected_editor(&self) -> Option<&TextEditor> {
        if let Some(selected) = self.selected {
            let pair = &self.headers[selected.row];
            if selected.col == 1 {
                return Some(&pair.1);
            }
            return Some(&pair.0);
        }
        None
    }

    fn selected_editor_mut(&mut self) -> Option<&mut TextEditor> {
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
        self.headers.push((TextEditor::new(), TextEditor::new()));
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

    pub(super) fn clear(&mut self) {
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
            let mut k = TextEditor::new();
            let mut v = TextEditor::new();
            k.set_text_raw(key);
            v.set_text_raw(val);
            self.headers.push((k, v));
        }
        self.selected = None;
    }
}
