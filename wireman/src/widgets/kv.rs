use edtui::EditorState;
use ratatui::{prelude::*, widgets::Widget};
use tui_widget_list::ListableWidget;

use crate::view::editor::{view_single_selected, view_single_unselected};

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
    fn main_axis_size(&self) -> usize {
        3
    }
}

impl Widget for KV {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        if self.key_selected {
            view_single_selected(&mut self.key, String::new()).render(area[0], buf);
        } else {
            view_single_unselected(&mut self.key, String::new()).render(area[0], buf);
        }
        if self.val_selected {
            view_single_selected(&mut self.val, String::new()).render(area[1], buf);
        } else {
            view_single_unselected(&mut self.val, String::new()).render(area[1], buf);
        }
    }
}
