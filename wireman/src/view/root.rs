use super::{headers::HeadersPage, messages::MessagesPage, selection::SelectionPage, theme::THEME};
use crate::context::{AppContext, Tab};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Tabs, Widget},
};
use std::rc::Rc;

pub struct Root<'a> {
    ctx: &'a AppContext,
}

impl<'a> Root<'a> {
    pub fn new(ctx: &'a AppContext) -> Self {
        Root { ctx }
    }
}
impl Root<'_> {
    fn render_navbar(&self, area: Rect, buf: &mut Buffer) {
        let area = layout(area, Direction::Horizontal, &[0, 50]);

        Paragraph::new(Span::styled("WireMan", THEME.app_title)).render(area[0], buf);
        let titles = vec![" Selection ", " Messages ", " Address & Headers "];
        Tabs::new(titles)
            .style(THEME.tabs)
            .highlight_style(THEME.tabs_selected)
            .select(self.ctx.tab.index())
            .divider("")
            .render(area[1], buf);
    }

    fn render_content(&self, area: Rect, buf: &mut Buffer) {
        match self.ctx.tab {
            Tab::Selection => SelectionPage {
                model: &mut self.ctx.selection.borrow_mut(),
                tab: self.ctx.selection_tab,
            }
            .render(area, buf),
            Tab::Messages => MessagesPage {
                model: &mut self.ctx.messages.borrow_mut(),
                tab: self.ctx.messages_tab,
            }
            .render(area, buf),
            Tab::Headers => HeadersPage::new(&self.ctx.headers.borrow()).render(area, buf),
        };
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let keys = match self.ctx.tab {
            Tab::Selection => SelectionPage::footer_keys(self.ctx.selection_tab),
            Tab::Messages => MessagesPage::footer_keys(),
            Tab::Headers => HeadersPage::new(&self.ctx.headers.borrow()).footer_keys(),
        };
        let spans: Vec<Span> = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(format!(" {key} "), THEME.key_binding.key);
                let desc = Span::styled(format!(" {desc} "), THEME.key_binding.description);
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

impl Widget for Root<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new().style(THEME.root).render(area, buf);
        let area = layout(area, Direction::Vertical, &[1, 0, 1]);
        self.render_navbar(area[0], buf);
        self.render_content(area[1], buf);
        self.render_footer(area[2], buf);
    }
}

/// simple helper method to split an area into multiple sub-areas
pub fn layout(area: Rect, direction: Direction, heights: &[u16]) -> Rc<[Rect]> {
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