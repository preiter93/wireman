#![allow(clippy::module_name_repetitions)]
use crate::controller::Controller;
use ratatui::{layout::Rect, widgets::Block, Frame};

pub fn render_metadata_popup<'a>(
    _f: &mut Frame,
    _area: Rect,
    _controller: &mut Controller<'a>,
    _block: Block<'a>,
) {
    // let mut widget = controller.metadata.borrow_mut().as_widget();
    // widget = widget.block(block);
    //
    // let popup_area = centered_rect_percentage(90, 20, area);
    //
    // f.render_widget(Clear, popup_area);
    // f.render_widget(&mut widget, popup_area);
}
