use crate::widgets::editor::{view_single_selected, view_single_unselected};
use edtui::EditorState;
use ratatui::{prelude::*, widgets::Widget};
use tui_widget_list::{ListableWidget, ScrollAxis};

#[derive(Clone)]
pub(crate) struct KV {
    pub(crate) key: EditorState,
    pub(crate) val: EditorState,
    pub(crate) key_selected: bool,
    pub(crate) val_selected: bool,
}

impl KV {
    fn new(key: &EditorState, val: &EditorState) -> Self {
        Self {
            key: key.clone(),
            val: val.clone(),
            key_selected: false,
            val_selected: false,
        }
    }
}

impl ListableWidget for KV {
    fn size(&self, _: &ScrollAxis) -> usize {
        3
    }
}

impl Widget for KV {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        use ratatui::layout::Constraint::Percentage;
        let [left, right] = Layout::horizontal([Percentage(50), Percentage(50)]).areas(area);

        if self.key_selected {
            view_single_selected(&mut self.key, String::new()).render(left, buf);
        } else {
            view_single_unselected(&mut self.key, String::new()).render(left, buf);
        }
        if self.val_selected {
            view_single_selected(&mut self.val, String::new()).render(right, buf);
        } else {
            view_single_unselected(&mut self.val, String::new()).render(right, buf);
        }
    }
}
