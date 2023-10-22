use crate::{app::AppContext, controller::Controller};

use super::{headers::HeadersTab, messages::MessagesTab, selection::SelectionTab, theme::THEME};
use ratatui::{prelude::*, widgets::*};
use std::rc::Rc;

pub struct Root<'a, 'b> {
    context: &'a AppContext,
    ctrl: &'a Controller<'b>,
}

impl<'a, 'b> Root<'a, 'b> {
    pub fn new(context: &'a AppContext, ctrl: &'a Controller<'b>) -> Self {
        Root { context, ctrl }
    }
}
impl Root<'_, '_> {
    fn render_navbar(&self, area: Rect, buf: &mut Buffer) {
        let area = layout(area, Direction::Horizontal, vec![0, 45]);

        Paragraph::new(Span::styled("StellaRPC", THEME.app_title)).render(area[0], buf);
        let titles = vec![" Selection ", " Messages ", " Headers "];
        Tabs::new(titles)
            .style(THEME.tabs)
            .highlight_style(THEME.tabs_selected)
            .select(self.context.tab_index)
            .divider("")
            .render(area[1], buf);
    }

    fn render_content(&self, area: Rect, buf: &mut Buffer) {
        match self.context.tab_index {
            0 => SelectionTab::new(&self.ctrl.selection).render(area, buf),
            1 => MessagesTab::new(&self.ctrl.messages).render(area, buf),
            2 => HeadersTab::new(&self.ctrl.metadata.borrow()).render(area, buf),
            _ => unreachable!(),
        };
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let keys = match self.context.tab_index {
            0 => SelectionTab::footer_keys(),
            1 => SelectionTab::footer_keys(),
            2 => HeadersTab::footer_keys(),
            _ => unreachable!(),
        };
        let spans: Vec<Span> = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(format!(" {} ", key), THEME.key_binding.key);
                let desc = Span::styled(format!(" {} ", desc), THEME.key_binding.description);
                [key, desc]
            })
            .collect();
        Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .fg(Color::Indexed(236))
            .bg(Color::Indexed(232))
            .render(area, buf);
    }
}

impl Widget for Root<'_, '_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new().style(THEME.root).render(area, buf);
        let area = layout(area, Direction::Vertical, vec![1, 0, 1]);
        self.render_navbar(area[0], buf);
        self.render_content(area[1], buf);
        self.render_footer(area[2], buf);
    }
}

/// simple helper method to split an area into multiple sub-areas
pub fn layout(area: Rect, direction: Direction, heights: Vec<u16>) -> Rc<[Rect]> {
    let constraints: Vec<Constraint> = heights
        .iter()
        .map(|&h| {
            if h > 0 {
                Constraint::Length(h)
            } else {
                Constraint::Min(0)
            }
        })
        .collect();
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
}
