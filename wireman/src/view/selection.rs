#![allow(clippy::cast_possible_truncation)]
use crate::context::SelectionTab;
use crate::model::SelectionModel;
use crate::widgets::list::ListItem;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, StatefulWidget, Widget};
use tui_widget_list::List;

use super::theme::THEME;

/// The page where to select services and methods.
pub struct SelectionPage<'a> {
    pub model: &'a mut SelectionModel,
    pub tab: SelectionTab,
}

impl<'a> SelectionPage<'a> {
    pub fn footer_keys(sub: SelectionTab) -> Vec<(&'static str, &'static str)> {
        let last = if sub == SelectionTab::Services {
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

impl Widget for SelectionPage<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        use ratatui::layout::Constraint::{Length, Min, Percentage};
        let [top, bottom] = Layout::vertical([Percentage(50), Percentage(50)]).areas(area);

        let mut show_services_search = 0;
        if self.model.services_filter.is_some() || self.tab == SelectionTab::SearchServices {
            show_services_search = 1;
        }
        let [svc_content, svc_search] =
            Layout::vertical([Min(0), Length(show_services_search)]).areas(top);

        let mut show_methods_search = 0;
        if self.model.methods_filter.is_some() || self.tab == SelectionTab::SearchMethods {
            show_methods_search = 1;
        }
        let [mtd_content, mtd_search] =
            Layout::vertical([Min(0), Length(show_methods_search)]).areas(bottom);

        // Block
        let block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .style(THEME.content)
            .padding(Padding::new(1, 1, 1, 1));

        // Services
        let services = self
            .model
            .services()
            .into_iter()
            .map(|service| ListItem::new(service.clone()));
        let services_state = &mut self.model.services_state;
        let mut services_block = block.clone().title("Services").white();
        if [SelectionTab::Services, SelectionTab::SearchServices].contains(&self.tab) {
            services_block = services_block.border_type(BorderType::Double);
        }
        List::new(services.collect()).block(services_block).render(
            svc_content,
            buf,
            services_state,
        );

        // Search line for services
        if show_services_search == 1 {
            SearchLine::new(self.model.services_filter.clone().unwrap_or_default())
                .render(svc_search, buf);
        }

        // Methods
        let methods = self
            .model
            .methods()
            .into_iter()
            .map(|method| ListItem::new(method.clone()));
        let methods_state = &mut self.model.methods_state;
        let mut methods_block = block.clone().title("Methods").white();
        if self.tab == SelectionTab::Methods {
            methods_block = methods_block.border_type(BorderType::Double);
        }
        List::new(methods.collect())
            .block(methods_block)
            .render(mtd_content, buf, methods_state);

        // Search line for methods
        if show_methods_search == 1 {
            SearchLine::new(self.model.methods_filter.clone().unwrap_or_default())
                .render(mtd_search, buf);
        }
    }
}
struct SearchLine {
    text: String,
}

impl SearchLine {
    fn new<T: Into<String>>(text: T) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for SearchLine {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Text::from(format!("/{}", self.text))).render(area, buf);
    }
}
