#![allow(clippy::module_name_repetitions)]
use crate::controller::Controller;
use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Clear},
    Frame,
};

use super::util::centered_rect_length;

/// Draw the widget that lets the user input metadata
pub fn render_address_popup<'a, B>(
    f: &mut Frame<B>,
    area: Rect,
    controller: &mut Controller<'a>,
    block: Block<'a>,
) where
    B: Backend,
{
    let editor = &mut controller.address.borrow_mut().editor;
    editor.update_style();
    editor.set_block(block);

    // Determine the widget size
    let popup_area = centered_rect_length(90, 4, area);

    // Render metadata
    f.render_widget(Clear, popup_area);
    f.render_widget(editor.widget(), popup_area);
}
