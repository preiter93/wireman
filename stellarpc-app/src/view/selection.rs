#![allow(clippy::cast_possible_truncation)]
use crate::controller::Controller;
use crate::widgets::list::ListItem as ListItem2;
use crate::widgets::list_with_children::ItemWithChildren;
use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::widgets::Block;
use ratatui::Frame;
use tui_widget_list::WidgetList;

/// Draws the service/method selection tile.
pub fn render_selection<B>(f: &mut Frame<B>, area: Rect, controller: &mut Controller, block: Block)
where
    B: Backend,
{
    let model = &mut controller.selection;
    let items: Vec<_> = model
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let service = item.service.clone();
            let is_expanded = model.state.expanded_parent();
            let methods = match is_expanded {
                Some(expanded) if expanded == i => item
                    .methods
                    .iter()
                    .map(|x| ListItem2::new(x.clone()).prefix(Some("    ")))
                    .collect(),
                _ => Vec::new(),
            };
            let mut methods_list = WidgetList::new(methods);
            methods_list.state.select(model.state.selected_child());
            ItemWithChildren::new(service, methods_list)
        })
        .collect();

    let mut widget = WidgetList::new(items);
    widget.state.select(model.state.selected_parent());
    widget = widget.block(block);

    f.render_widget(&mut widget, area);
}
