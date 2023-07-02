pub mod address;
pub mod help;
pub mod messages;
pub mod metadata;
pub mod selection;
pub(super) mod util;

use crate::commons::window_border;
use crate::controller::Controller;
use crate::controller::Window;
use crate::view::help::render_help;
use ratatui::backend::Backend;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::Frame;
use std::cmp::max;
use std::rc::Rc;

use self::address::render_address_popup;
use self::messages::render_messages;
use self::metadata::render_metadata_popup;
use self::selection::render_selection;

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

    render_messages(
        f,
        chunks[1],
        controller,
        window_border("Request", controller.window == Window::Request),
    );

    if controller.show_address {
        render_address_popup(
            f,
            f.size(),
            controller,
            window_border("Address", controller.window == Window::Address),
        );
    }

    if controller.show_metadata {
        render_metadata_popup(
            f,
            f.size(),
            controller,
            window_border("Metadata", controller.window == Window::Metadata),
        );
    }
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
