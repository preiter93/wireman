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
        let items = &self.model.items;

        // Block
        let block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .style(THEME.content)
            .padding(Padding::new(1, 1, 1, 1));

        // Services
        let svcs = items.iter().map(|e| ListItem::new(e.service.clone()));
        let index = self.model.selected_service_index();
        let state = &mut self.model.svc_state;
        state.select(index);
        let mut svc_block = block.clone().title("Services").bold().white();
        if self.sub == 0 {
            svc_block = svc_block.border_type(BorderType::Double);
        }
        List::new(svcs.collect())
            .block(svc_block)
            .render(area[0], buf, state);

        // Methods
        if let Some(svc_index) = self.model.selected_service_index() {
            let mths = &self.model.items[svc_index].methods;
            let mths = mths.iter().map(|e| ListItem::new(e.to_string()));
            let state = &mut self.model.mth_state;
            state.select(self.model.selection.selected_child());
            let mut block = block.title("Methods").bold().white();
            if self.sub == 1 {
                block = block.border_type(BorderType::Double);
            }
            List::new(mths.collect())
                .block(block)
                .render(area[1], buf, state);
        }
    }
}
