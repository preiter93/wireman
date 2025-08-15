use super::root::layout;
use crate::{model::history::HistoryModel, widgets::tabs::ActivatableTabs};

use core::MethodDescriptor;
use logger::Logger;
use ratatui::prelude::*;

pub struct HistoryTabs {
    pub model: HistoryModel,
    pub selected_method: Option<MethodDescriptor>,
    pub show_help: bool,
}

impl HistoryTabs {
    pub fn new(
        model: HistoryModel,
        selected_method: Option<MethodDescriptor>,
        show_help: bool,
    ) -> Self {
        Self {
            model,
            selected_method,
            show_help,
        }
    }
}

impl Widget for HistoryTabs {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = theme::Theme::global();
        if !self.model.enabled {
            let [_, right] = layout(area, Direction::Horizontal, &[0, 25]);

            let titles = vec![" 1 ", " 2 ", " 3 ", " 4 ", " 5 "];
            let mut tabs = ActivatableTabs::new(titles)
                .style(theme.base.unfocused)
                .active_style(theme.highlight.unfocused)
                .highlight_style(theme.base.unfocused.reversed())
                .active_highlight_style(theme.highlight.unfocused.reversed())
                .select(self.model.save_spot().saturating_sub(1));
            if let Some(method) = &self.selected_method {
                tabs = tabs.active(self.model.save_spots_enabled(method));
            }
            tabs.render(right, buf);
        }
    }
}
