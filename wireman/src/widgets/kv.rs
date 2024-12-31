use crate::widgets::editor::{view_single_selected, view_single_unselected};
use edtui::EditorState;
use ratatui::{prelude::*, widgets::Widget};

#[derive(Clone)]
pub(crate) struct KV {
    pub(crate) key: EditorState,
    pub(crate) val: EditorState,
    pub(crate) key_selected: bool,
    pub(crate) val_selected: bool,
    pub(crate) show_border_title: bool,
}

impl Widget for KV {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        use ratatui::layout::Constraint::Percentage;
        let [left, right] = Layout::horizontal([Percentage(50), Percentage(50)]).areas(area);

        let key_title = if self.show_border_title {
            String::from(" Key (H) ")
        } else {
            String::new()
        };
        let val_title = if self.show_border_title {
            String::from(" Value (L) ")
        } else {
            String::new()
        };

        if self.key_selected {
            view_single_selected(&mut self.key, key_title).render(left, buf);
        } else {
            view_single_unselected(&mut self.key, key_title).render(left, buf);
        }
        if self.val_selected {
            view_single_selected(&mut self.val, val_title).render(right, buf);
        } else {
            view_single_unselected(&mut self.val, val_title).render(right, buf);
        }
    }
}
