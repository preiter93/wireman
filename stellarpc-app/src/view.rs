pub mod address;
pub mod help;
pub mod messages;
pub mod metadata;
pub mod selection;

use crate::commons::window_border;
use crate::controller::Controller;
use crate::controller::Window;
use crate::view::address::render_address;
use crate::view::help::render_help;
use crate::view::messages::render_messages;
use crate::view::metadata::render_metadata;
use crate::view::selection::render_selection;
use ratatui::backend::Backend;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::Frame;
use std::cmp::max;
use std::rc::Rc;

/// render the widgets of this page
pub fn render<B: Backend>(f: &mut Frame<B>, controller: &mut Controller) {
    // Split window vertically
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    // Left column
    let chunks_l = split_left_column(chunks[0], controller);

    render_selection(
        f,
        chunks_l[0],
        controller,
        window_border("Selection", controller.window == Window::Selection),
    );

    render_help(f, chunks_l[1], controller, &window_border("Help", false));

    // Right column
    let chunks_r = split_right_column(chunks[1], controller);

    render_address(
        f,
        chunks_r[0],
        controller,
        window_border("Address", controller.window == Window::Address),
    );

    render_metadata(
        f,
        chunks_r[1],
        controller,
        window_border("Metadata", controller.window == Window::Metadata),
    );

    render_messages(
        f,
        chunks_r[2],
        controller,
        window_border("Request", controller.window == Window::Request),
    );
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

fn split_right_column(area: Rect, controller: &Controller) -> Rc<[Rect]> {
    let address_length = if controller.show_address { 3 } else { 0 };
    let metadata_length = if controller.show_metadata { 5 } else { 0 };
    Layout::default()
        .constraints(
            [
                Constraint::Length(address_length),
                Constraint::Length(metadata_length),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(area)
}
