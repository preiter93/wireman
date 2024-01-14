#![allow(clippy::cast_possible_truncation)]
use crate::model::SelectionModel;
use crate::widgets::list::ListItem;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Padding, StatefulWidget, Widget};
use tui_widget_list::List;

use super::theme::THEME;

/// The page where to select services and methods.
pub struct SelectionTab<'a> {
    pub model: &'a mut SelectionModel,
    pub sub: usize,
}

impl<'a> SelectionTab<'a> {
    pub fn footer_keys(sub: usize) -> Vec<(&'static str, &'static str)> {
        let last = if sub == 0 {
            ("Enter", "Select")
        } else {
            ("Esc", "Unselect")
        };
        vec![
            ("q", "Quit"),
            ("Tab", "Next Tab"),
            ("j", "Next"),
            ("k", "Prev"),
            ("↑", "Up"),
            ("↓", "Down"),
            last,
        ]
    }
}

impl Widget for SelectionTab<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        // Layout
        let area = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        // Block
        let block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .style(THEME.content)
            .padding(Padding::new(1, 1, 1, 1));

        // Services
        let services = (&self.model.services)
            .iter()
            .map(|service| ListItem::new(service.clone()));
        let services_state = &mut self.model.services_state;
        let mut services_block = block.clone().title("Services").bold().white();
        if self.sub == 0 {
            services_block = services_block.border_type(BorderType::Double);
        }
        List::new(services.collect())
            .block(services_block)
            .render(area[0], buf, services_state);

        // Methods
        let methods = (&self.model.methods)
            .iter()
            .map(|method| ListItem::new(method.clone()));
        let methods_state = &mut self.model.methods_state;
        let mut methods_block = block.clone().title("Methods").bold().white();
        if self.sub == 1 {
            methods_block = methods_block.border_type(BorderType::Double);
        }
        List::new(methods.collect())
            .block(methods_block)
            .render(area[1], buf, methods_state);
    }
}
