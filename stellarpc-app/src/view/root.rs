use crate::app::AppContext;

use super::theme::THEME;
use ratatui::{prelude::*, widgets::*};
use std::rc::Rc;

pub struct Root<'a> {
    context: &'a AppContext,
}

impl<'a> Root<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Root { context }
    }
}
impl Root<'_> {
    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        let area = layout(area, Direction::Horizontal, vec![0, 45]);

        Paragraph::new(Span::styled("StellaRPC", THEME.app_title)).render(area[0], buf);
        let titles = vec![" Message ", " Headers "];
        Tabs::new(titles)
            .style(THEME.tabs)
            .highlight_style(THEME.tabs_selected)
            .select(self.context.tab_index)
            .divider("")
            .render(area[1], buf);
    }

    fn render_bottom_bar(&self, area: Rect, buf: &mut Buffer) {
        let keys = [
            ("q", "Quit"),
            ("Tab", "Next Tab"),
            ("↑/k", "Up"),
            ("↓/j", "Down"),
            ("←/h", "Left"),
            ("→/l", "Right"),
        ];
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

impl Widget for Root<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new().style(THEME.root).render(area, buf);
        let area = layout(area, Direction::Vertical, vec![1, 0, 1]);
        self.render_title_bar(area[0], buf);
        self.render_bottom_bar(area[2], buf);
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
