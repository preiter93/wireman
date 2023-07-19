#![allow(clippy::module_name_repetitions)]
use super::util::centered_rect_percentage;
use crate::controller::Controller;
use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Clear},
    Frame,
};

pub fn render_history_popup<'a, B>(
    f: &mut Frame<B>,
    area: Rect,
    controller: &mut Controller<'a>,
    block: Block<'a>,
) where
    B: Backend,
{
    let mut widget = controller.history.as_widget();
    widget = widget.block(block);

    let popup_area = centered_rect_percentage(90, 20, area);

    f.render_widget(Clear, popup_area);
    f.render_widget(&mut widget, popup_area);
}
