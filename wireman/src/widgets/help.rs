use crate::context::HelpContext;
use ratatui::{
    prelude::{Buffer, Constraint, Rect},
    text::Line,
    widgets::{Block, Borders, Cell, Padding, Row, Table, Widget},
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
        let block = Block::default()
            .borders(Borders::ALL)
            .title_style(theme.title.focused)
            .padding(Padding::horizontal(1))
            .title_top(Line::from(" Help ").centered());

        let mut rows = Vec::new();
        for (key, msg) in self.key_map {
            rows.push(Row::new(vec![
                Cell::from(key.to_string()).style(theme.title.focused),
                Cell::from(msg.to_string()).style(theme.base.focused),
            ]));
        }
        rows.push(Row::new(vec![
            Cell::from("?".to_string()).style(theme.title.focused),
            Cell::from("Close help".to_string()).style(theme.base.focused),
        ]));
        rows.push(Row::new(vec![
            Cell::from("C-c".to_string()).style(theme.title.focused),
            Cell::from("Quit app".to_string()).style(theme.base.focused),
        ]));

        let widths = [Constraint::Length(15), Constraint::Length(25)];
        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(theme.base.focused)
            .block(block);
        table.render(area, buf);
    }
}
