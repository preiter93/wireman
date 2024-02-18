#![allow(clippy::module_name_repetitions, clippy::cast_possible_truncation)]
use crate::context::MessagesTab;
use crate::model::MessagesModel;
use crate::view::root::layout;
use crate::widgets::{
    editor::{view_selected, view_unselected},
    tabs::ActivatableTabs,
};
use ratatui::prelude::*;

use super::theme::THEME;

/// The request and response tab
pub struct MessagesPage<'a> {
    pub model: &'a mut MessagesModel,
    pub tab: MessagesTab,
}

impl<'a> MessagesPage<'a> {
    pub fn footer_keys() -> Vec<(&'static str, &'static str)> {
        vec![
            ("q", "Quit"),
            ("Tab", "Next Tab"),
            ("↑", "Up"),
            ("↓", "Down"),
            ("Enter", "gRPC"),
        ]
    }
}

impl Widget for MessagesPage<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        use ratatui::layout::Constraint::{Length, Min, Percentage};
        let [top, center, bottom] =
            Layout::vertical([Percentage(50), Length(1), Min(0)]).areas(area);

        // Request
        let editor = if self.tab == MessagesTab::Request {
            view_selected(&mut self.model.request.editor.state, "Request")
        } else {
            view_unselected(&mut self.model.request.editor.state, "Request")
        };
        editor.render(top, buf);

        // History
        if !self.model.history_model.disabled {
            let [_, right] = layout(center, Direction::Horizontal, &[0, 25]);
            let titles = vec![" 1 ", " 2 ", " 3 ", " 4 ", " 5 "];
            let mut tabs = ActivatableTabs::new(titles)
                .style(THEME.tabs)
                .active_style(THEME.tabs_active)
                .highlight_style(THEME.tabs_selected)
                .select(self.model.history_model.save_spot().saturating_sub(1))
                .divider("");
            if let Some(method) = &self.model.selected_method {
                tabs = tabs.active(self.model.history_model.save_spots_enabled(method));
            }
            tabs.render(right, buf);
        }

        // Request
        let editor = if self.tab == MessagesTab::Response {
            view_selected(&mut self.model.response.editor.state, "Response")
        } else {
            view_unselected(&mut self.model.response.editor.state, "Response")
        };
        editor.render(bottom, buf);
    }
}
