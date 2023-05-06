use ratatui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::Style,
    text::Span,
    widgets::{Block, Cell, Row, Table},
    Frame,
};

use crate::{commons::window_border, controller::Controller, theme};

/// Draws the help tile
pub fn render_help<B>(f: &mut Frame<B>, area: Rect, controller: &mut Controller, _block: &Block)
where
    B: Backend,
{
    if let Some(actions) = controller.help_hint() {
        let key_style = Style::default().fg(theme::COL_HELP_KEY_FG);
        let msg_style = Style::default().fg(theme::COL_HELP_MSG_FG);

        let mut rows = vec![];
        for (key, msg) in actions.iter() {
            let row = Row::new(vec![
                Cell::from(Span::styled((*key).to_string(), key_style)),
                Cell::from(Span::styled((*msg).to_string(), msg_style)),
            ]);
            rows.push(row);
        }

        let table = Table::new(rows)
            .block(window_border("Help", false))
            .widths(&[Constraint::Length(5), Constraint::Min(15)])
            .column_spacing(1);

        f.render_widget(table, area);
    }
}
