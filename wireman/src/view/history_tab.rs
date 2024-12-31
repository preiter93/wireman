use super::root::layout;
use crate::{model::history::HistoryModel, widgets::tabs::ActivatableTabs};

use core::MethodDescriptor;
use ratatui::prelude::*;

pub struct HistoryTabs<'a> {
    pub model: &'a HistoryModel,
    pub selected_method: Option<MethodDescriptor>,
    pub show_help: bool,
}

impl<'a> HistoryTabs<'a> {
    pub fn new(
        model: &'a HistoryModel,
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

impl Widget for HistoryTabs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = theme::Theme::global();
        if !self.model.enabled {
            let [text, _, right] = layout(area, Direction::Horizontal, &[0, 3, 25]);

            if self.show_help {
                Line::from(vec![
                    Span::from("<C-s>: ").style(theme.title.unfocused),
                    Span::from("Save  ").style(theme.base.unfocused),
                    Span::from("<C-q>: ").style(theme.title.unfocused),
                    Span::from("Reset").style(theme.base.unfocused),
                ])
                .render(text, buf);
            }

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
