#![allow(clippy::module_name_repetitions)]
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    widgets::Block,
    Frame,
};

use crate::controller::AddressController;

/// Draw the widget that lets the user input metadata
pub fn draw_address<'a, B>(
    f: &mut Frame<B>,
    area: Rect,
    controller: &mut AddressController<'a>,
    block: Block<'a>,
) where
    B: Backend,
{
    let editor = &mut controller.model.borrow_mut().editor;
    editor.set_style_default();
    editor.set_block(block);

    // Determine the widget size
    let chunks = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .split(area);

    // Render metadata
    f.render_widget(editor.widget(), chunks[0]);
}
