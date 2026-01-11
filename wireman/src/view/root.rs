use super::{
    configuration::ConfigurationDialog, headers::HeadersPage, messages::MessagesPage,
    selection::SelectionPage, util::spans_from_keys,
};
use crate::{
    context::{AppContext, Tab},
    widgets::{help::HelpDialog, modal::centered_rect},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Paragraph, Tabs, Widget},
};
use std::rc::Rc;
use theme::{self, Theme};

pub struct Root<'a> {
    ctx: &'a mut AppContext,
}

impl<'a> Root<'a> {
    pub fn new(ctx: &'a mut AppContext) -> Self {
        Root { ctx }
    }
}
impl Root<'_> {
    fn render_navbar(&self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::global();
        let [title, tabs] = layout(area, Direction::Horizontal, &[0, 31]);
        Block::new().style(theme.base.focused).render(area, buf);

        let style = theme.base.unfocused;
        let highlight_style = theme.title.focused;

        Paragraph::new(Span::styled("WireMan", theme.highlight.focused).bold()).render(title, buf);
        let titles = vec!["Endpoints", "Headers", "Request"];
        Tabs::new(titles)
            .style(style)
            .highlight_style(highlight_style)
            .select(self.ctx.tab.index())
            .render(tabs, buf);

        // Capture tab areas for hit-testing
        let third = tabs.width / 3;
        let rem = tabs.width % 3;
        let r0 = Rect {
            x: tabs.x,
            y: tabs.y,
            width: third,
            height: tabs.height,
        };
        let r1 = Rect {
            x: tabs.x + third,
            y: tabs.y,
            width: third,
            height: tabs.height,
        };
        let r2 = Rect {
            x: tabs.x + third * 2,
            y: tabs.y,
            width: third + rem,
            height: tabs.height,
        };
        self.ctx.ui.borrow_mut().navbar_tabs = Some([r0, r1, r2]);
    }

    fn render_info(&self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::global();
        let selection = self.ctx.selection.borrow();

        let info = match (
            selection.selected_service().map(|s| s.name().to_string()),
            selection.selected_method().map(|m| m.name().to_string()),
        ) {
            (Some(service), Some(method)) => format!("{service} > {method}"),
            _ => String::new(),
        };

        Paragraph::new(Span::styled(info, theme.base.unfocused)).render(area, buf);
    }

    fn render_content(&self, area: Rect, buf: &mut Buffer) {
        match self.ctx.tab {
            Tab::Selection => SelectionPage {
                model: &mut self.ctx.selection.borrow_mut(),
                reflection_model: &mut self.ctx.reflection.borrow_mut(),
                tab: self.ctx.selection_tab,
            }
            .render(area, buf),
            Tab::Messages => {
                let mut history_tabs_area = self.ctx.ui.borrow().history_tabs;
                MessagesPage {
                    model: &mut self.ctx.messages.borrow_mut(),
                    tab: self.ctx.messages_tab,
                    history_tabs_area: Some(&mut history_tabs_area),
                }
                .render(area, buf);
                self.ctx.ui.borrow_mut().history_tabs = history_tabs_area;
            }
            Tab::Headers => {
                let headers_rc = Rc::clone(&self.ctx.headers);
                let mut history_tabs_area = self.ctx.ui.borrow().history_tabs;
                HeadersPage::new(headers_rc)
                    .with_history_tabs_area(&mut history_tabs_area)
                    .render(area, buf);
                self.ctx.ui.borrow_mut().history_tabs = history_tabs_area;
            }
        };
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let keys = match self.ctx.tab {
            Tab::Selection => SelectionPage::footer_keys(self.ctx.selection_tab),
            Tab::Messages => MessagesPage::footer_keys(
                self.ctx.messages_tab,
                self.ctx.messages.borrow().request.editor.insert_mode(),
            ),
            Tab::Headers => HeadersPage::footer_keys(&self.ctx.headers.borrow()),
        };
        let spans = spans_from_keys(&keys);
        Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}

impl Widget for Root<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::global();
        Block::new().style(theme.base.focused).render(area, buf);

        if theme.hide_footer {
            let [header, info, content] = layout(area, Direction::Vertical, &[1, 1, 0]);
            self.render_navbar(header, buf);
            self.render_info(info, buf);
            self.render_content(content, buf);
        } else {
            let [header, info, content, footer] = layout(area, Direction::Vertical, &[1, 1, 0, 1]);
            self.render_navbar(header, buf);
            self.render_info(info, buf);
            self.render_content(content, buf);
            self.render_footer(footer, buf);
        }

        if let Some(help_ctx) = &self.ctx.help {
            let popup_area = centered_rect(80, 70, area);
            Clear.render(popup_area, buf);
            HelpDialog::new(help_ctx).render(popup_area, buf);
        }

        if self.ctx.configuration.borrow().toggled() {
            let popup_area = centered_rect(80, 70, area);
            Clear.render(popup_area, buf);
            ConfigurationDialog {
                model: &mut self.ctx.configuration.borrow_mut(),
            }
            .render(popup_area, buf);
        }
    }
}

/// simple helper method to split an area into multiple sub-areas
pub fn layout<const N: usize>(area: Rect, direction: Direction, sizes: &[u16]) -> [Rect; N] {
    use ratatui::layout::Constraint::{Length, Min};
    let constraints = sizes
        .iter()
        .map(|&h| if h > 0 { Length(h) } else { Min(0) });
    if direction == Direction::Vertical {
        Layout::vertical(constraints).areas(area)
    } else {
        Layout::horizontal(constraints).areas(area)
    }
}
