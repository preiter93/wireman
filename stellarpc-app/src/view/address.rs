#![allow(clippy::module_name_repetitions)]
use super::util::centered_rect_length;
use crate::controller::Controller;
use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Clear},
    Frame,
};

pub fn render_address_popup<'a>(f: &mut Frame, area: Rect, _: &mut Controller<'a>, _: Block<'a>) {
    // let _ = &mut controller.address.borrow_mut().editor;
    // editor.update_style();
    // editor.set_block(block);

    let popup_area = centered_rect_length(90, 4, area);

    f.render_widget(Clear, popup_area);
    // f.render_widget(&*editor, popup_area);
}
