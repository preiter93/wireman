#![allow(clippy::cast_possible_truncation)]
use crate::controller::Controller;
use crate::theme;
use crate::widgets::list_with_children::ListItem;
use crate::widgets::list_with_children::ListWithChildren;
use crate::widgets::list_with_children::ListWithChildrenItem;
use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Block;
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
