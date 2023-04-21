use ratatui::layout::Rect;
use ratatui::{backend::Backend, widgets::Block, Frame};

use crate::commons::editor::TextEditor;

/// Draw the widget that lets the user input metadata
pub fn draw_metadata<'a, B>(
    f: &mut Frame<B>,
    area: Rect,
    controller: &mut TextEditor<'a>,
    block: Block<'a>,
) where
    B: Backend,
{
    controller.set_block(block);
    f.render_widget(controller.widget(), area);
}
