use crate::context::HelpContext;
use ratatui::{
    prelude::{Buffer, Constraint, Rect},
    text::Line,
    widgets::{Block, Borders, Cell, Row, Table, Widget},
};
use theme::Theme;

pub struct HelpDialog {
    key_map: Vec<(String, String)>,
}

impl HelpDialog {
    pub fn new(ctx: &HelpContext) -> Self {
        Self {
            key_map: ctx.key_mappings.clone(),
        }
    }
}

impl Widget for HelpDialog {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let theme = Theme::global();
        let style = theme.help_dialog.style;
        let block = Block::default()
            .borders(Borders::ALL)
            .title_top(Line::from("Help").centered());
        let mut rows = Vec::new();
        for (key, msg) in self.key_map {
            rows.push(Row::new(vec![
                Cell::from(key.to_string()),
                Cell::from(msg.to_string()),
            ]));
        }
        rows.push(Row::new(vec![
            Cell::from("?".to_string()),
            Cell::from("Close help".to_string()),
        ]));
        rows.push(Row::new(vec![
            Cell::from("C-c".to_string()),
            Cell::from("Quit app".to_string()),
        ]));

        let widths = [Constraint::Length(15), Constraint::Length(25)];
        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(style)
            .block(block);
        table.render(area, buf);
    }
}
