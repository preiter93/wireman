#![allow(clippy::module_name_repetitions)]
use crate::controller::MetadataController;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    widgets::Block,
    Frame,
};

/// Draw the widget that lets the user input metadata
pub fn draw_metadata<'a, B>(
    f: &mut Frame<B>,
    area: Rect,
    controller: &mut MetadataController<'a>,
    block: Block<'a>,
) where
    B: Backend,
{
    // Determine the widget size
    let chunks = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .split(area);

    let mut widget = controller.model.borrow_mut().content.clone();
    widget = widget.block(block);

    // Render metadata
    f.render_widget(&mut widget, chunks[0]);
}
