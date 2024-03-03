use ratatui::{
    prelude::{Alignment, Buffer, Constraint, Rect},
    style::Style,
    text::Span,
    widgets::{block::Title, Block, Borders, Cell, Row, Table, Widget},
};
use std::collections::BTreeMap;

use crate::context::AppContext;

pub struct HelpDialog {
    key_map: BTreeMap<String, String>,
}

impl HelpDialog {
    pub fn new() -> Self {
        let key_map: BTreeMap<String, String> = [
            ("a".to_string(), "Do that".to_string()),
            ("b".to_string(), "Do that".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        Self { key_map }
    }

    pub fn from_ctx(_: &AppContext) -> Self {
        // let key_map = AppEventHandler::get_key_mappings(ctx).into_iter().collect();
        // Self { key_map }
        Self::new()
    }
}

impl Widget for HelpDialog {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let key_style = Style::default();
        let msg_style = Style::default();
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Title::from("Help").alignment(Alignment::Center));
        let rows: Vec<_> = self
            .key_map
            .iter()
            .map(|(key, msg)| {
                Row::new(vec![
                    Cell::from(Span::styled(key.to_string(), key_style)),
                    Cell::from(Span::styled(msg.to_string(), msg_style)),
                ])
            })
            .collect();

        let widths = [Constraint::Length(10), Constraint::Length(25)];
        let table = Table::new(rows, widths).column_spacing(1).block(block);
        table.render(area, buf);
    }
}
