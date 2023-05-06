#![allow(clippy::cast_possible_truncation)]
use crate::commons::window_border;
use crate::controller::Controller;
use crate::theme;
use crate::widgets::list_with_children::ListItem;
use crate::widgets::list_with_children::ListWithChildren;
use crate::widgets::list_with_children::ListWithChildrenItem;
use ratatui::backend::Backend;
use ratatui::layout::Constraint;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::Block;
use ratatui::widgets::Cell;
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::Frame;

/// Draws the service/method selection tile.
pub fn render_selection<B>(f: &mut Frame<B>, area: Rect, controller: &mut Controller, block: Block)
where
    B: Backend,
{
    let model = &mut controller.selection;
    let widget = model
        .items
        .iter()
        .map(|item| {
            ListWithChildrenItem::new(
                item.service.clone(),
                item.methods
                    .iter()
                    .map(|child| ListItem::new(child.clone()))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let list = ListWithChildren::new(widget)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(theme::COL_LIST_HIGHLIGHTED_SERVICE_BG)
                .fg(theme::COL_LIST_HIGHLIGHTED_SERVICE_FG),
        )
        .highlight_sub_style(
            Style::default()
                .bg(theme::COL_LIST_HIGHLIGHTED_METHOD_BG)
                .fg(theme::COL_LIST_HIGHLIGHTED_METHOD_FG),
        )
        .highlight_symbol(">>");

    f.render_stateful_widget(list, area, &mut model.state);
}

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
