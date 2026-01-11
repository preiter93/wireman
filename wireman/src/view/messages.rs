#![allow(clippy::module_name_repetitions, clippy::cast_possible_truncation)]

use crate::context::MessagesTab;
use crate::model::MessagesModel;
use crate::view::history_tab::HistoryTabs;
use crate::widgets::editor::{view_selected, view_unselected};
use edtui::{EditorMode, EditorStatusLine};
use ratatui::prelude::*;

/// The request and response tab
pub struct MessagesPage<'a> {
    pub model: &'a mut MessagesModel,
    pub tab: MessagesTab,
    pub history_tabs_area: Option<&'a mut Option<[Rect; 5]>>,
}

impl MessagesPage<'_> {
    pub fn footer_keys(tab: MessagesTab, insert_mode: bool) -> Vec<(&'static str, &'static str)> {
        let mut keys = vec![("^c", "Quit")];
        if tab == MessagesTab::Request {
            keys.push(("⏎", "Request"));
            keys.push(("<C-s>", "Save"));
            keys.push(("<C-q>", "Reset"));
        } else {
            keys.push(("Y", "Copy"));
        }
        if insert_mode {
            keys.push(("Esc", "Normal"));
        } else {
            keys.push(("<C-e>", "Open in Editor"));
        }
        keys.push(("?", "Show help"));
        keys
    }
}

impl Widget for MessagesPage<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        use ratatui::layout::Constraint::{Length, Min, Percentage};
        let theme = theme::Theme::global();
        let sl = u16::from(!theme.hide_status);
        let request_window_size = self.model.request.window_size;
        let [top, bottom, status] =
            Layout::vertical([Percentage(request_window_size), Min(0), Length(sl)]).areas(area);

        // Request
        let editor = if self.tab == MessagesTab::Request {
            view_selected(&mut self.model.request.editor.state, " Request (K) ")
        } else {
            view_unselected(&mut self.model.request.editor.state, " Request (K) ")
        };
        editor.render(top, buf);
        self.model.request.content_area = Some(top);

        // Request
        let editor = if self.tab == MessagesTab::Response {
            view_selected(&mut self.model.response.editor.state, " Response (J) ")
        } else {
            view_unselected(&mut self.model.response.editor.state, " Response (J) ")
        };
        editor.render(bottom, buf);
        self.model.response.content_area = Some(bottom);

        // Status line
        if !theme.hide_status {
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
                .style_mode(theme.highlight.unfocused.reversed())
                .style_search(theme.base.unfocused)
                .style_line(theme.base.unfocused)
                .mode(mode.name());
            if mode == EditorMode::Search {
                status_line = status_line.search(Some(search));
            }

            let [s, h] = Layout::horizontal([Min(0), Length(60)]).areas(status);

            status_line.render(s, buf);

            let mut history = HistoryTabs::new(
                self.model.history.borrow().clone(),
                self.model.selected_method.clone(),
                true,
            );
            if let Some(areas_ref) = self.history_tabs_area {
                history = history.with_tab_areas(areas_ref);
            }
            history.render(h, buf);
        }
    }
}
