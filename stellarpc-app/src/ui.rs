use ratatui::layout::Rect;
use std::cmp::max;
use std::rc::Rc;

use ratatui::backend::Backend;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::Frame;

use crate::commons::window_border;
use crate::controller::Controller;
use crate::controller::Window;
use crate::view::selection::draw_help2;
use crate::view::selection::draw_selection;

/// render the widgets of this page
pub fn render<B: Backend>(f: &mut Frame<B>, controller: &mut Controller) {
    // Split window vertically
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    // Left column
    let chunks_l = split_left_column(chunks[0], controller);

    draw_selection(
        f,
        chunks_l[0],
        controller,
        window_border("Selection", controller.window == Window::Selection),
    );

    draw_help2(f, chunks_l[1], controller, window_border("Help", false));

    // let help_actions = controller.help();
    // draw_selection_and_help(
    //     f,
    //     chunks[0],
    //     &mut controller.selection_controller,
    //     window_border("Selection", controller.window == Window::Selection),
    //     &help_actions,
    // );

    // let address_length = if controller.show_address { 3 } else { 0 };
    // let metadata_length = if controller.show_metadata { 5 } else { 0 };
    // let chunks = Layout::default()
    //     .constraints(
    //         [
    //             Constraint::Length(address_length),
    //             Constraint::Length(metadata_length),
    //             Constraint::Min(0),
    //         ]
    //         .as_ref(),
    //     )
    //     .split(chunks[1]);
    //
    // draw_address(
    //     f,
    //     chunks[0],
    //     &mut controller.address_controller,
    //     window_border("Address", controller.window == Window::Address),
    // );
    //
    // draw_metadata(
    //     f,
    //     chunks[1],
    //     &mut controller.metadata_controller,
    //     window_border("Metadata", controller.window == Window::Metadata),
    // );
    //
    // draw_request(
    //     f,
    //     chunks[2],
    //     &mut controller.messages_controller,
    //     window_border("Request", controller.window == Window::Request),
    // );
}

/// Split left column into selection and help
fn split_left_column(area: Rect, controller: &Controller) -> Rc<[Rect]> {
    // Determine the size of the help widget
    let help_height = controller
        .help_hint()
        .as_ref()
        .map_or(0, |action| max(14, action.len()) as u16 + 2);
    Layout::default()
        .constraints([Constraint::Min(0), Constraint::Length(help_height)].as_ref())
        .split(area)
}
