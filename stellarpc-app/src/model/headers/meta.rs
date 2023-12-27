use std::collections::BTreeMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use edtui::{EditorMode, Index2};

use crate::commons::editor::TextEditor;

#[derive(Default)]
pub struct MetaHeaders {
    headers: Vec<KV>,
    selected: Option<Index2>,
}

impl MetaHeaders {
    pub fn on_key(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Left | KeyCode::Right if self.mode() == EditorMode::Normal => {
                if let Some(selected) = &mut self.selected {
                    selected.col ^= 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') if self.mode() == EditorMode::Normal => {
                if let Some(selected) = &mut self.selected {
                    selected.row = selected.row.saturating_sub(1);
                }
            }
            KeyCode::Down | KeyCode::Char('j') if self.mode() == EditorMode::Normal => {
                if let Some(selected) = &mut self.selected {
                    selected.row += 1;
                    selected.row = selected.row.min(self.headers.len().saturating_sub(1));
                }
            }
            KeyCode::Char('h')
                if event.modifiers == KeyModifiers::CONTROL
                    && self.mode() == EditorMode::Normal =>
            {
                self.add();
            }
            KeyCode::Char('d')
                if event.modifiers == KeyModifiers::CONTROL
                    && self.mode() == EditorMode::Normal =>
            {
                if let Some(index) = self.selected_index() {
                    self.remove(index.row);
                }
            }
            _ => {
                if let Some(input) = self.selected_editor_mut() {
                    input.on_key(event);
                }
            }
        }
    }

    pub(super) fn mode(&self) -> EditorMode {
        self.selected_editor()
            .map_or(EditorMode::Normal, |x| x.state.mode)
    }

    pub(super) fn select(&mut self) {
        self.selected = Some(Index2::default());
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
                return Some(&pair.val);
            }
            return Some(&pair.key);
        }
        None
    }

    fn selected_editor_mut(&mut self) -> Option<&mut TextEditor> {
        if let Some(selected) = self.selected {
            let pair = &mut self.headers[selected.row];
            if selected.col == 1 {
                return Some(&mut pair.val);
            }
            return Some(&mut pair.key);
        }
        None
    }

    pub(crate) fn add(&mut self) {
        self.headers.push(KV::default());
    }

    pub(crate) fn remove(&mut self, index: usize) {
        self.headers.remove(index);
        if self.headers.is_empty() {
            self.selected = None;
            return;
        }
        if let Some(selected) = &mut self.selected {
            selected.row = selected.row.saturating_sub(1);
            selected.col = 0;
        }
    }

    pub(crate) fn headers(&self) -> Vec<(&TextEditor, &TextEditor)> {
        self.headers.iter().map(|x| (&x.key, &x.val)).collect()
    }

    pub(crate) fn headers_raw(&self) -> Vec<(String, String)> {
        self.headers
            .iter()
            .map(|x| (x.key.get_text_raw(), x.val.get_text_raw()))
            .collect()
    }

    pub(crate) fn selected_index(&self) -> Option<Index2> {
        self.selected
    }

    /// Get the headers as a map.
    pub fn as_btree(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        for (key, val) in self.headers_raw() {
            if !key.is_empty() {
                let _ = map.insert(key, val);
            }
        }
        map
    }

    /// Get the headers as a map.
    pub fn set_btree(&mut self, data: &BTreeMap<String, String>) {
        self.headers.clear();
        for (key, val) in data {
            let mut k = TextEditor::new();
            k.set_text_raw(key);
            let mut v = TextEditor::new();
            v.set_text_raw(val);
            self.headers.push(KV { key: k, val: v });
        }
    }

    pub(super) fn clear(&mut self) {
        self.headers.clear();
        self.selected = None;
    }
}

#[derive(Default)]
struct KV {
    key: TextEditor,
    val: TextEditor,
}
