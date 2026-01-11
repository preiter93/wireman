use super::root::layout;
use crate::{model::history::HistoryModel, widgets::tabs::ActivatableTabs};

use core::MethodDescriptor;
use ratatui::prelude::*;

pub struct HistoryTabs<'a> {
    pub model: HistoryModel,
    pub selected_method: Option<MethodDescriptor>,
    pub show_help: bool,
    pub tab_areas: Option<&'a mut Option<[Rect; 5]>>,
}

impl<'a> HistoryTabs<'a> {
    pub fn new(
        model: HistoryModel,
        selected_method: Option<MethodDescriptor>,
        show_help: bool,
    ) -> Self {
        Self {
            model,
            selected_method,
            show_help,
            tab_areas: None,
        }
    }

    pub fn with_tab_areas(mut self, tab_areas: &'a mut Option<[Rect; 5]>) -> Self {
        self.tab_areas = Some(tab_areas);
        self
    }
}

impl Widget for HistoryTabs<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
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

            // Store tab areas for click detection
            if let Some(ref mut areas_ref) = self.tab_areas {
                let mut areas = [Rect::default(); 5];
                let title_width = 3u16;

                for (i, area) in areas.iter_mut().enumerate() {
                    let x = right
                        .left()
                        .saturating_add(1)
                        .saturating_add((i as u16) * 5);
                    if x < right.right() {
                        *area = Rect {
                            x,
                            y: right.top(),
                            width: title_width.min(right.right().saturating_sub(x)),
                            height: 1,
                        };
                    }
                }

                **areas_ref = Some(areas);
            }
        } else if let Some(ref mut areas_ref) = self.tab_areas {
            **areas_ref = None;
        }
    }
}
