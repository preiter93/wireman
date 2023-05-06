#![allow(clippy::module_name_repetitions)]
use crate::controller::Controller;
use ratatui::{backend::Backend, layout::Rect, widgets::Block, Frame};

/// Draw the widget that lets the user input metadata
pub fn render_metadata<'a, B>(
    f: &mut Frame<B>,
    area: Rect,
    controller: &mut Controller<'a>,
    block: Block<'a>,
) where
    B: Backend,
{
    let mut widget = controller.metadata.borrow_mut().content.clone();
    widget = widget.block(block);
    f.render_widget(&mut widget, area);
}
