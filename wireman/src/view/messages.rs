#![allow(clippy::module_name_repetitions, clippy::cast_possible_truncation)]
use crate::context::MessagesTab;
use crate::model::MessagesModel;
use crate::widgets::tabs::ActivatableTabs;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::prelude::Direction;
use ratatui::widgets::Widget;

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
        // Layout
        let area = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Length(1),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(area);

        // Request
        let editor = if self.tab == MessagesTab::Request {
            super::editor::view_selected(&mut self.model.request.editor.state, "Request")
        } else {
            super::editor::view_unselected(&mut self.model.request.editor.state, "Request")
        };
        editor.render(area[0], buf);

        // History
        if self.model.history_model.enabled {
            let area_s = crate::view::root::layout(area[1], Direction::Horizontal, &[0, 25]);
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
            tabs.render(area_s[1], buf);
        }

        // Request
        let editor = if self.tab == MessagesTab::Response {
            super::editor::view_selected(&mut self.model.response.editor.state, "Response")
        } else {
            super::editor::view_unselected(&mut self.model.response.editor.state, "Response")
        };
        editor.render(area[2], buf);
    }
}
