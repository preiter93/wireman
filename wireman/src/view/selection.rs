#![allow(clippy::cast_possible_truncation)]

use crate::model::selection::SelectionMode;
use crate::model::SelectionModel;
use crate::view::reflection_dialog::ReflectionDialog;
use crate::widgets::list::ListItem;
use crate::widgets::modal::centered_rect;
use crate::{context::SelectionTab, model::reflection::ReflectionModel};
use ratatui::style::{Style, Stylize};
use ratatui::text::Text;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Layout, Rect},
    prelude::StatefulWidget,
    text::Line,
    widgets::{Block, Borders, Clear, Padding, Widget},
};
use theme::Theme;
use tui_widget_list::{ListBuilder, ListView};

pub(crate) struct SelectionPage<'a> {
    pub model: &'a mut SelectionModel,
    pub reflection_model: &'a mut ReflectionModel,
    pub tab: SelectionTab,
}

impl SelectionPage<'_> {
    pub fn footer_keys(sub: SelectionTab) -> Vec<(&'static str, &'static str)> {
        let mut keys = vec![("^c", "Quit"), ("j/k", "Scroll"), ("⏎", "Select")];
        if sub == SelectionTab::Methods {
            keys.push(("Esc", "Unselect"));
        }
        keys.push(("/", "Search"));
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

        let block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .padding(Padding::uniform(1));

        // Services
        let is_selected = [SelectionTab::Services].contains(&self.tab);

        let services = self.model.services();
        let services_state = &mut self.model.services_state;

        let (border_style, title_style) = if is_selected {
            (theme.border.focused, theme.title.focused)
        } else {
            (theme.border.unfocused, theme.title.unfocused)
        };
        let services_block = block
            .clone()
            .title(" Services ")
            .title_style(title_style)
            .border_style(border_style)
            .border_type(theme.border.border_type);
        let inner_area = services_block.inner(svc_content);

        let item_count = services.len();
        let builder = ListBuilder::new(move |context| {
            let title = &services[context.index];
            let mut widget = ListItem::new(title.to_string());

            let style = match (is_selected, context.is_selected) {
                (true, true) => theme.highlight.focused.reversed(),
                (true, false) => theme.base.focused,
                (false, true) => theme.highlight.unfocused.reversed(),
                (false, false) => theme.base.unfocused,
            };

            widget.style = style;
            if context.is_selected {
                widget.prefix = Some(">>");
            }

            (widget, 1)
        });

        ListView::new(builder, item_count)
            .block(services_block.clone())
            .scroll_padding(0)
            .render(svc_content, buf, services_state);

        if !self.model.has_services() {
            let text = Text::from(vec![
                Line::from("It seems you don't have any proto services available. ")
                    .style(theme.base.focused),
                Line::from(""),
                Line::from(" 1. Please configure them in your wireman.toml, or")
                    .style(theme.base.focused),
                Line::from(" 2. use server reflection (<C-r>), or").style(theme.base.focused),
                Line::from(" 3. use protos from the current folder via `wireman --local-protos`")
                    .style(theme.base.focused),
                Line::from(""),
                Line::from("For further information see: ").style(theme.base.focused),
                Line::from("https://preiter93.github.io/wireman/")
                    .underlined()
                    .style(theme.title.focused),
                Line::from("(Copy link with \"y\")").style(theme.base.unfocused),
            ]);
            text.render(inner_area, buf);
        }

        // Search line for services
        if show_services_search == 1 {
            let style = if self.tab == SelectionTab::SearchServices {
                theme.base.focused
            } else {
                theme.base.unfocused
            };
            SearchLine::new(
                self.model.services_filter.clone().unwrap_or_default(),
                style,
            )
            .render(svc_search, buf);
        }

        // Methods
        let is_selected = [SelectionTab::Methods].contains(&self.tab);

        let (border_style, title_style) = if is_selected {
            (theme.border.focused, theme.title.focused)
        } else {
            (theme.border.unfocused, theme.title.unfocused)
        };

        let methods = self.model.methods();
        let methods_state = &mut self.model.methods_state;
        let methods_block = block
            .clone()
            .title(" Methods ")
            .title_style(title_style)
            .border_style(border_style)
            .border_type(theme.border.border_type);

        let item_count = methods.len();
        let builder = ListBuilder::new(move |context| {
            let title = &methods[context.index];
            let mut widget = ListItem::new(title.to_string());

            let style = match (is_selected, context.is_selected) {
                (true, true) => theme.highlight.focused.reversed(),
                (true, false) => theme.base.focused,
                (false, true) => theme.highlight.unfocused.reversed(),
                (false, false) => theme.base.unfocused,
            };

            if context.is_selected {
                widget.prefix = Some(">>");
            }
            widget.style = style;

            (widget, 1)
        });

        ListView::new(builder, item_count)
            .block(methods_block)
            .scroll_padding(1)
            .render(mtd_content, buf, methods_state);

        // Search line for methods
        if show_methods_search == 1 {
            let style = if self.tab == SelectionTab::SearchMethods {
                theme.base.focused
            } else {
                theme.base.unfocused
            };
            SearchLine::new(self.model.methods_filter.clone().unwrap_or_default(), style)
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
    style: Style,
}

impl SearchLine {
    fn new<T: Into<String>>(text: T, style: Style) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }
}

impl Widget for SearchLine {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::from(format!("Search (/): {}", self.text))
            .style(self.style)
            .render(area, buf);
    }
}
