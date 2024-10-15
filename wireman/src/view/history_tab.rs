use super::root::layout;
use crate::{model::history::HistoryModel, widgets::tabs::ActivatableTabs};

use core::MethodDescriptor;
use ratatui::prelude::*;

pub struct HistoryTabs<'a> {
    pub model: &'a HistoryModel,
    pub selected_method: Option<MethodDescriptor>,
}

impl<'a> HistoryTabs<'a> {
    pub fn new(model: &'a HistoryModel, selected_method: Option<MethodDescriptor>) -> Self {
        Self {
            model,
            selected_method,
        }
    }
}

impl Widget for HistoryTabs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = theme::Theme::global();
        if !self.model.enabled {
            let [_, _, right] = layout(area, Direction::Horizontal, &[0, 3, 25]);

            // Line::from("💾").render(symbol, buf);

            let titles = vec![" 1 ", " 2 ", " 3 ", " 4 ", " 5 "];
            let mut tabs = ActivatableTabs::new(titles)
                .style(theme.history.inactive.0)
                .active_style(theme.history.active.0)
                .highlight_style(theme.history.inactive.1)
                .active_highlight_style(theme.history.active.1)
                .select(self.model.save_spot().saturating_sub(1));
            if let Some(method) = &self.selected_method {
                tabs = tabs.active(self.model.save_spots_enabled(method));
            }
            tabs.render(right, buf);
        }
    }
}
