use super::headers::{Address, Authentication, ListElements};
use super::root::layout;
use crate::model::headers::{AuthSelection, HeadersTab};
use crate::model::reflection::ReflectionModel;
use ratatui::layout::Layout;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::{
    buffer::Buffer,
    layout::{Direction, Rect},
    widgets::{Block, Borders, Widget},
};
use theme::Theme;

pub struct ReflectionDialog {
    pub model: ReflectionModel,
}

impl ReflectionDialog {
    pub fn new(model: ReflectionModel) -> Self {
        Self { model }
    }
}

impl Widget for ReflectionDialog {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        use ratatui::layout::Constraint::{Length, Min};
        let theme = Theme::global();
        let style = theme.base.style;
        let area = {
            let block = Block::default()
                .borders(Borders::ALL)
                .style(style)
                .title_top(Line::from(" Reflection ").centered())
                .title_style(theme.border.text.1);
            let inner_area = block.inner(area);
            block.render(area, buf);
            inner_area
        };
        let max_height_footer = 3;
        let [m, mut f] = Layout::vertical([Min(0), Length(max_height_footer)]).areas(area);

        // Address
        let layout = layout(m, Direction::Vertical, &[1, 1, 3, 1, 1, 4]);
        let [_, addr_title, addr_content, _, auth_title, auth_content] = layout;

        if self.model.headers.borrow().tab == HeadersTab::Addr {
            ListElements::VDividerFocused(String::from(" Address ")).render(addr_title, buf);
        } else {
            ListElements::VDividerUnfocused(String::from(" Address ")).render(addr_title, buf);
        }
        Address {
            state: self.model.headers.borrow().addr.state.clone(),
            title: String::new(),
            selected: self.model.headers.borrow().tab == HeadersTab::Addr,
        }
        .render(addr_content, buf);

        // Authentication
        if self.model.headers.borrow().tab == HeadersTab::Auth {
            ListElements::VDividerFocused(String::from(" Authentication ")).render(auth_title, buf);
        } else {
            ListElements::VDividerUnfocused(String::from(" Authentication "))
                .render(auth_title, buf);
        }
        let body = match self.model.headers.borrow().auth.selected {
            AuthSelection::Bearer => Authentication {
                state: self.model.headers.borrow().auth.bearer.state.clone(),
                title: String::new(),
                selected: self.model.headers.borrow().tab == HeadersTab::Auth,
                selected_tag: 0,
            },
            AuthSelection::Basic => Authentication {
                state: self.model.headers.borrow().auth.basic.state.clone(),
                title: String::new(),
                selected: self.model.headers.borrow().tab == HeadersTab::Auth,
                selected_tag: 1,
            },
        };
        body.render(auth_content, buf);

        // Status line
        let line = if let Some(err) = self.model.error {
            Line::from(err).left_aligned().red()
        } else {
            Line::from("Press Enter ")
                .style(theme.footer.text)
                .right_aligned()
        };

        let paragraph = Paragraph::new(line).wrap(Wrap { trim: true });
        let line_count = paragraph.line_count(f.width) as u16;
        f.y += max_height_footer.saturating_sub(line_count);
        paragraph.render(f, buf);
    }
}
