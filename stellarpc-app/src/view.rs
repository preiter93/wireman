pub mod headers;
pub mod help;
pub mod history;
pub mod messages;
pub mod metadata;
pub mod root;
pub mod selection;
pub mod theme;
pub(super) mod util;

// use crate::commons::window_border;
// use crate::controller::Controller;
// use crate::view::help::render_help;
// use ratatui::backend::Backend;
// use ratatui::layout::Constraint;
// use ratatui::layout::Layout;
// use ratatui::layout::Rect;
// use ratatui::prelude::Alignment;
// use ratatui::style::Color;
// use ratatui::style::Style;
// use ratatui::style::Stylize;
// use ratatui::text::Line;
// use ratatui::text::Span;
// use ratatui::widgets::Paragraph;
// use ratatui::Frame;
// use std::cmp::max;
// use std::rc::Rc;
//
// use self::address::render_address_popup;
// use self::history::render_history_popup;
// use self::metadata::render_metadata_popup;

// /// render the widgets of this page
// pub fn render<B: Backend>(f: &mut Frame<B>, controller: &mut Controller) {
//     let root = Layout::default()
//         .direction(ratatui::layout::Direction::Vertical)
//         .constraints([Constraint::Min(0), Constraint::Length(2)].as_ref())
//         .split(f.size());
//
//     // Split window vertically
//     let chunks = Layout::default()
//         .direction(ratatui::layout::Direction::Horizontal)
//         .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
//         .split(root[0]);
//
//     // Left column
//     let chunks_l = split_left_column(chunks[0], controller);
//
//     render_help(f, chunks_l[1], controller, &window_border("Help", false));
//
//     // if controller.show_address {
//     //     render_address_popup(
//     //         f,
//     //         f.size(),
//     //         controller,
//     //         window_border("Address", controller.window == Window::Address),
//     //     );
//     // }
//     //
//     // if controller.show_metadata {
//     //     render_metadata_popup(
//     //         f,
//     //         f.size(),
//     //         controller,
//     //         window_border("Metadata", controller.window == Window::Metadata),
//     //     );
//     // }
//     //
//     // if controller.show_history {
//     //     render_history_popup(
//     //         f,
//     //         f.size(),
//     //         controller,
//     //         window_border("History", controller.window == Window::History),
//     //     );
//     // }
//
//     // render_bottom_bar(f, root[1]);
// }
//
// /// Split left column into selection and help
// fn split_left_column(area: Rect, controller: &Controller) -> Rc<[Rect]> {
//     // Determine the size of the help widget
//     let help_height = controller
//         .help_hint()
//         .as_ref()
//         .map_or(0, |action| max(14, action.len()) as u16 + 2);
//     Layout::default()
//         .constraints([Constraint::Min(0), Constraint::Length(help_height)].as_ref())
//         .split(area)
// }
//
// fn render_bottom_bar<B: Backend>(f: &mut Frame<B>, area: Rect) {
//     let keys = [
//         ("q", "Quit"),
//         ("Tab", "Next Tab"),
//         ("↑/k", "Up"),
//         ("↓/j", "Down"),
//     ];
//     let spans: Vec<Span> = keys
//         .iter()
//         .flat_map(|(key, desc)| {
//             let key = Span::styled(format!(" {} ", key), Style::new().fg(BLACK).bg(DARK_GRAY));
//             let desc = Span::styled(format!(" {} ", desc), Style::new().fg(DARK_GRAY).bg(BLACK));
//             [key, desc]
//         })
//         .collect();
//     let paragraph = Paragraph::new(Line::from(spans))
//         .alignment(Alignment::Center)
//         .fg(Color::Indexed(236))
//         .bg(Color::Indexed(232));
//     f.render_widget(paragraph, area);
// }
// const BLACK: Color = Color::Indexed(232); // not really black, often #080808
// const DARK_GRAY: Color = Color::Indexed(238);
