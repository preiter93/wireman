#![allow(clippy::module_name_repetitions, clippy::cast_possible_truncation)]
use crate::model::MessagesModel;
use crate::widgets::tabs::ActivatableTabs;
use edtui::EditorTheme;
use edtui::EditorView;
use edtui::StatusLine;
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::prelude::Direction;
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Widget;

use super::theme::THEME;

/// The request and response tab
pub struct MessagesTab<'a> {
    pub model: &'a mut MessagesModel,
    pub sub: usize,
}

impl<'a> MessagesTab<'a> {
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

impl Widget for MessagesTab<'_> {
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

        // Block
        let block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .style(THEME.content);

        // Request
        let editor = EditorView::new(&mut self.model.request.editor.state);
        let mut theme = EditorTheme::default();
        let block_req = block.clone().title("Request").bold().white();
        if self.sub == 0 {
            theme = theme
                .block(block_req.clone().border_type(BorderType::Double))
                .status_line(Some(
                    StatusLine::default()
                        .style_text(THEME.status_line.0)
                        .style_line(THEME.status_line.1),
                ));
        } else {
            theme = theme
                .block(block_req.clone().border_type(BorderType::Plain))
                .cursor_style(EditorTheme::default().base_style())
                .status_line(None);
        }
        editor.theme(theme).render(area[0], buf);

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

        // Response
        let editor = EditorView::new(&mut self.model.response.editor.state);
        let mut theme = EditorTheme::default();
        let block_resp = block.title("Response").bold().white();
        if self.sub == 1 {
            theme = theme.block(block_resp.clone().border_type(BorderType::Double));
        } else {
            theme = theme
                .block(block_resp.clone().border_type(BorderType::Plain))
                .cursor_style(EditorTheme::default().base_style())
                .status_line(None);
        }
        editor.theme(theme).render(area[2], buf);
    }
}
