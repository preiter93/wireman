#![allow(clippy::module_name_repetitions, clippy::cast_possible_truncation)]

use crate::context::MessagesTab;
use crate::model::MessagesModel;
use crate::widgets::editor::{view_selected, view_unselected};
use edtui::{EditorMode, EditorStatusLine};
use ratatui::prelude::*;

/// The request and response tab
pub struct MessagesPage<'a> {
    pub model: &'a mut MessagesModel,
    pub tab: MessagesTab,
}

impl MessagesPage<'_> {
    pub fn footer_keys(tab: MessagesTab) -> Vec<(&'static str, &'static str)> {
        let mut keys = vec![("^c", "Quit"), ("Tab", "Next Page")];
        if tab == MessagesTab::Request {
            keys.push(("J", "Down"));
            keys.push(("⏎", "Make request"));
        } else {
            keys.push(("K", "Up"));
        }
        keys.push(("?", "Show help"));
        keys
    }
}

impl Widget for MessagesPage<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        use ratatui::layout::Constraint::{Length, Min, Percentage};
        let theme = theme::Theme::global();
        let sl = u16::from(!theme.editor.hide_status_line);
        let request_window_size = self.model.request.window_size;
        let [top, bottom, status] =
            Layout::vertical([Percentage(request_window_size), Min(0), Length(sl)]).areas(area);

        // Request
        let editor = if self.tab == MessagesTab::Request {
            view_selected(&mut self.model.request.editor.state, "Request")
        } else {
            view_unselected(&mut self.model.request.editor.state, "Request")
        };
        editor.render(top, buf);

        // Request
        let editor = if self.tab == MessagesTab::Response {
            view_selected(&mut self.model.response.editor.state, "Response")
        } else {
            view_unselected(&mut self.model.response.editor.state, "Response")
        };
        editor.render(bottom, buf);

        // Status line
        if !theme.editor.hide_status_line {
            let (mode, search) = match self.tab {
                MessagesTab::Request => (
                    self.model.request.editor.state.mode,
                    self.model.request.editor.state.search_pattern(),
                ),
                MessagesTab::Response => (
                    self.model.response.editor.state.mode,
                    self.model.response.editor.state.search_pattern(),
                ),
            };

            let mut status_line = EditorStatusLine::default()
                .style_text(theme.editor.status_text)
                .style_line(theme.editor.status_line)
                .mode(mode.name());
            if mode == EditorMode::Search {
                status_line = status_line.search(Some(search));
            }

            status_line.render(status, buf);
        }
    }
}
