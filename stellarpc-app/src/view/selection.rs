#![allow(clippy::cast_possible_truncation)]
use crate::model::SelectionModel;
use crate::widgets::list::ListItem;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Widget};
use tui_widget_list::WidgetList;

use super::theme::THEME;

/// The page where to select services and methods.
pub struct SelectionTab<'a> {
    pub model: &'a SelectionModel,
    pub sub: usize,
}

impl<'a> SelectionTab<'a> {
    pub fn footer_keys() -> Vec<(&'static str, &'static str)> {
        vec![
            ("q", "Quit"),
            ("Tab", "Next Tab"),
            ("↑/k", "Up"),
            ("↓/j", "Down"),
            ("Enter", "Select"),
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
        let items = self.model.items.clone();

        // Block
        let block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .style(THEME.content)
            .padding(Padding::new(1, 1, 1, 1));

        // Services
        let svcs = items.into_iter().map(|e| ListItem::new(e.service));
        let mut svc_list = WidgetList::new(svcs.collect());
        svc_list.state.select(self.model.selected_service_index());
        let mut svc_block = block.clone().title("Services").bold().white();
        if self.sub == 0 {
            svc_block = svc_block.border_type(BorderType::Double)
        }
        svc_list = svc_list.block(svc_block);
        svc_list.render(area[0], buf);

        // Methods
        if let Some(svc_index) = self.model.selected_service_index() {
            let mthds = &self.model.items[svc_index].methods;
            let mthds = mthds.iter().map(|e| ListItem::new(e.to_string()));
            let mut mth_list = WidgetList::new(mthds.collect());
            mth_list.state.select(self.model.selection.selected_child());
            let mut mth_block = block.title("Methods").bold().white();
            if self.sub == 1 {
                mth_block = mth_block.border_type(BorderType::Double)
            }
            mth_list = mth_list.block(mth_block);
            mth_list.render(area[1], buf);
        }
    }
}
