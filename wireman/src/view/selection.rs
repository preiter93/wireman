#![allow(clippy::cast_possible_truncation)]
use crate::context::SelectionTab;
use crate::model::SelectionModel;
use crate::widgets::list::ListItem;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, StatefulWidget, Widget};
use theme::Theme;
use tui_widget_list::List;

/// The page where to select services and methods.
pub struct SelectionPage<'a> {
    pub model: &'a mut SelectionModel,
    pub tab: SelectionTab,
}

impl<'a> SelectionPage<'a> {
    pub fn footer_keys(sub: SelectionTab) -> Vec<(&'static str, &'static str)> {
        let mut keys = vec![
            ("^c", "Quit"),
            ("j/↓", "Next"),
            ("k/↑", "Prev"),
            ("Enter", "Select"),
        ];
        if sub == SelectionTab::Methods {
            keys.push(("Esc", "Unselect"));
        }
        keys.push(("?", "Show help"));
        keys
    }
}

impl Widget for SelectionPage<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let theme = Theme::global();
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
            .padding(Padding::new(1, 1, 1, 1));

        // Services
        let services = self
            .model
            .services()
            .into_iter()
            .map(|service| ListItem::new(service.clone()));
        let services_state = &mut self.model.services_state;
        let mut services_block = block
            .clone()
            .title("Services")
            .title_style(theme.border.text.0)
            .border_style(theme.border.border.0)
            .border_type(theme.border.border_type.0);
        if [SelectionTab::Services, SelectionTab::SearchServices].contains(&self.tab) {
            services_block = services_block
                .title_style(theme.border.text.1)
                .border_style(theme.border.border.1)
                .border_type(theme.border.border_type.1);
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
        let mut methods_block = block
            .clone()
            .title("Methods")
            .title_style(theme.border.text.0)
            .border_style(theme.border.border.0)
            .border_type(theme.border.border_type.0);
        if [SelectionTab::Methods, SelectionTab::SearchMethods].contains(&self.tab) {
            methods_block = methods_block
                .title_style(theme.border.text.1)
                .border_style(theme.border.border.1)
                .border_type(theme.border.border_type.1);
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
