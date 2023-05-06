#![allow(clippy::module_name_repetitions)]
use crate::controller::Controller;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    widgets::Block,
    Frame,
};

/// Draw the widget that lets the user input metadata
pub fn render_address<'a, B>(
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
    let chunks = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .split(area);

    // Render metadata
    f.render_widget(editor.widget(), chunks[0]);
}
