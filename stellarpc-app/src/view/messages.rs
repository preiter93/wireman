#![allow(clippy::module_name_repetitions, clippy::cast_possible_truncation)]
use crate::model::MessagesModel;
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::prelude::Direction;
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Tabs;
use ratatui::widgets::Widget;
use tui_vim_editor::editor::theme::EditorTheme;
use tui_vim_editor::Editor;

use super::root::layout;
use super::theme::THEME;

/// The request and response tab
pub struct MessagesTab<'a> {
    pub model: &'a MessagesModel,
    pub sub: usize,
}

impl<'a, 'b> MessagesTab<'a> {
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
        let buffer = &self.model.request.editor.buffer;
        let mut editor = Editor::new(buffer);
        let mut theme = EditorTheme::default();
        let block = block.title("Request").bold().white();
        if self.sub == 0 {
            theme = theme.block(block.clone().border_type(BorderType::Double));
        } else {
            theme = theme
                .block(block.clone().border_type(BorderType::Plain))
                .cursor_style(EditorTheme::default().base_style())
                .status_line(None);
        }
        editor.theme(theme).render(area[0], buf);

        // Save spot
        let area_s = layout(area[1], Direction::Horizontal, vec![0, 25]);
        let titles = vec![" 1 ", " 2 ", " 3 ", " 4 ", " 5 "];
        Tabs::new(titles)
            .style(THEME.tabs)
            .highlight_style(THEME.tabs_selected)
            .select(self.model.history_model.save_spot().saturating_sub(1))
            .divider("")
            .render(area_s[1], buf);

        // Response
        let buffer = &self.model.response.editor.buffer;
        editor = Editor::new(buffer);
        let mut theme = EditorTheme::default();
        let block = block.title("Response").bold().white();
        if self.sub == 1 {
            theme = theme.block(block.clone().border_type(BorderType::Double));
        } else {
            theme = theme
                .block(block.clone().border_type(BorderType::Plain))
                .cursor_style(EditorTheme::default().base_style())
                .status_line(None);
        }
        editor.theme(theme).render(area[2], buf);
    }
}
