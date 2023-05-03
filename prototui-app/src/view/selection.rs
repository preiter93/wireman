use crate::commons::window_border;
use crate::commons::HelpActions;
use crate::controller::SelectionController;
use crate::theme;
use crate::widgets::list_with_children::ListItem;
use crate::widgets::list_with_children::ListWithChildren;
use crate::widgets::list_with_children::ListWithChildrenItem;
use ratatui::backend::Backend;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::Block;
use ratatui::widgets::Cell;
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::Frame;
use std::cmp::max;

const MIN_HELP_SIZE: usize = 18;

/// Draw the services and methods list and draw the helper widget
/// below is `help_actions` has values.
pub fn draw_selection_and_help<B>(
    f: &mut Frame<B>,
    area: Rect,
    controller: &mut SelectionController,
    block: Block,
    help_actions: Option<HelpActions>,
) where
    B: Backend,
{
    // Determine the size of the help widget
    let help_length = help_actions
        .as_ref()
        .map(|action| max(MIN_HELP_SIZE, action.len()) as u16 + 2)
        .unwrap_or(0);
    let chunks = Layout::default()
        .constraints([Constraint::Min(0), Constraint::Length(help_length)].as_ref())
        .split(area);

    // Render the list message
    let list = draw_list_with_children(controller, block);
    f.render_stateful_widget(list, chunks[0], controller.list_state());

    // Render the help widget
    if let Some(actions) = &help_actions {
        let help = draw_help(actions);
        f.render_widget(help, chunks[1]);
    }
}

fn draw_list_with_children<'a>(
    controller: &mut SelectionController,
    block: Block<'a>,
) -> ListWithChildren<'a> {
    let widget = controller
        .items()
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

    ListWithChildren::new(widget)
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
        .highlight_symbol(">>")
}

fn draw_help(actions: &HelpActions) -> Table {
    let key_style = Style::default().fg(theme::COL_HELP_KEY_FG);
    let msg_style = Style::default().fg(theme::COL_HELP_MSG_FG);

    let mut rows = vec![];
    for (key, msg) in actions.iter() {
        let row = Row::new(vec![
            Cell::from(Span::styled(key.to_string(), key_style)),
            Cell::from(Span::styled(msg.to_string(), msg_style)),
        ]);
        rows.push(row);
    }

    Table::new(rows)
        .block(window_border("Help", false))
        .widths(&[Constraint::Length(5), Constraint::Min(MIN_HELP_SIZE as u16)])
        .column_spacing(1)
}
