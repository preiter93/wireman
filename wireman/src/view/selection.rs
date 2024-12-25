#![allow(clippy::cast_possible_truncation)]

use crate::model::selection::SelectionMode;
use crate::model::SelectionModel;
use crate::view::reflection_dialog::ReflectionDialog;
use crate::widgets::list::ListItem;
use crate::widgets::modal::centered_rect;
use crate::{context::SelectionTab, model::reflection::ReflectionModel};
use ratatui::style::Stylize;
use ratatui::text::Span;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Layout, Rect},
    prelude::StatefulWidget,
    text::Line,
    widgets::{Block, Borders, Clear, Padding, Widget},
};
use theme::Theme;
use tui_widget_list::{ListBuilder, ListView};

/// The page where to select services and methods.
pub struct SelectionPage<'a> {
    pub model: &'a mut SelectionModel,
    pub reflection_model: &'a mut ReflectionModel,
    pub tab: SelectionTab,
}

impl SelectionPage<'_> {
    pub fn footer_keys(sub: SelectionTab) -> Vec<(&'static str, &'static str)> {
        let mut keys = vec![
            ("^c", "Quit"),
            ("Tab", "Next Page"),
            ("j/k", "Scroll"),
            ("/", "Search"),
            ("⏎", "Select"),
        ];
        if sub == SelectionTab::Methods {
            keys.push(("Esc", "Unselect"));
        }
        keys.push(("?", "Show help"));
        keys
    }
}

impl Widget for SelectionPage<'_> {
    #[allow(clippy::too_many_lines)]
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        use ratatui::layout::Constraint::{Length, Min, Percentage};
        let theme = Theme::global();
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
        let services = self.model.services();
        let services_state = &mut self.model.services_state;
        let mut services_block = block
            .clone()
            .title("|Services|")
            .title_style(theme.border.text.0)
            .border_style(theme.border.border.0)
            .border_type(theme.border.border_type.0);
        if [SelectionTab::Services, SelectionTab::SearchServices].contains(&self.tab) {
            services_block = services_block
                .title_style(theme.border.text.1)
                .border_style(theme.border.border.1)
                .border_type(theme.border.border_type.1);
        }
        let inner_area = services_block.inner(svc_content);

        let item_count = services.len();
        let builder = ListBuilder::new(move |context| {
            let theme = Theme::global();
            let title = &services[context.index];
            let mut widget = ListItem::new(title.to_string());

            if context.is_selected {
                widget.prefix = Some(">");
                widget.style = theme.list.focused;
            } else {
                widget.style = theme.list.text;
            }
            (widget, 1)
        });

        ListView::new(builder, item_count)
            .block(services_block.clone())
            .scroll_padding(1)
            .render(svc_content, buf, services_state);

        if !self.model.has_services() {
            let [l1, l2, l3, l4] =
                Layout::vertical([Length(1), Length(1), Length(1), Min(0)]).areas(inner_area);
            Span::from("It seems you don't have any proto services available. ").render(l1, buf);
            Span::from("Please check your files and proto includes, see: ").render(l2, buf);
            Span::from("https://preiter93.github.io/wireman/")
                .underlined()
                .render(l3, buf);
            Span::from("(Copy link with \"y\")").render(l4, buf);
        }

        // Search line for services
        if show_services_search == 1 {
            SearchLine::new(self.model.services_filter.clone().unwrap_or_default())
                .render(svc_search, buf);
        }

        // Methods
        let methods = self.model.methods();
        let methods_state = &mut self.model.methods_state;
        let mut methods_block = block
            .clone()
            .title("|Methods|")
            .title_style(theme.border.text.0)
            .border_style(theme.border.border.0)
            .border_type(theme.border.border_type.0);

        if [SelectionTab::Methods, SelectionTab::SearchMethods].contains(&self.tab) {
            methods_block = methods_block
                .title_style(theme.border.text.1)
                .border_style(theme.border.border.1)
                .border_type(theme.border.border_type.1);
        }

        let item_count = methods.len();
        let builder = ListBuilder::new(move |context| {
            let theme = Theme::global();
            let title = &methods[context.index];
            let mut widget = ListItem::new(title.to_string());

            if context.is_selected {
                widget.prefix = Some(">>");
                widget.style = theme.list.focused;
            } else {
                widget.style = theme.list.text;
            }
            (widget, 1)
        });

        ListView::new(builder, item_count)
            .block(methods_block)
            .scroll_padding(1)
            .render(mtd_content, buf, methods_state);

        // Search line for methods
        if show_methods_search == 1 {
            SearchLine::new(self.model.methods_filter.clone().unwrap_or_default())
                .render(mtd_search, buf);
        }

        // Show a dialog with address and authentication input fields
        // which are required for server reflection
        if self.model.selection_mode == SelectionMode::ReflectionDialog {
            let popup_area = centered_rect(80, 80, area);
            Clear.render(popup_area, buf);
            ReflectionDialog::new(self.reflection_model.clone()).render(popup_area, buf);
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
        Line::from(format!("/{}", self.text)).render(area, buf);
    }
}
