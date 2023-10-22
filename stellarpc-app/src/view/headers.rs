use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Padding, Widget},
};

use crate::model::MetadataModel;

use super::theme::THEME;

/// The request and response tab
pub struct HeadersTab<'a, 'b> {
    meta: &'a MetadataModel<'b>,
}

impl<'a, 'b> HeadersTab<'a, 'b> {
    pub fn new(meta: &'a MetadataModel<'b>) -> Self {
        Self { meta }
    }

    pub fn footer_keys() -> Vec<(&'static str, &'static str)> {
        vec![
            ("q", "Quit"),
            ("Tab", "Next Tab"),
            ("↑/k", "Up"),
            ("↓/j", "Down"),
            ("Enter", "Select"),
        ]
    }
}

impl Widget for HeadersTab<'_, '_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        // Layout
        let area = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        // Block
        let block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .style(THEME.content)
            .padding(Padding::new(1, 1, 0, 1));
    }
}
