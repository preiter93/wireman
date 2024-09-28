use super::headers::{Address, Authentication, ListElements};
use super::root::layout;
use crate::model::headers::{AuthSelection, HeadersTab};
use crate::model::reflection::ReflectionModel;
use ratatui::layout::Layout;
use ratatui::text::Line;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Direction, Rect},
    widgets::{block::Title, Block, Borders, Widget},
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
        let title_style = theme.navbar.title;
        let area = {
            let block = Block::default()
                .borders(Borders::ALL)
                .style(style)
                .title(Title::from("Reflection Settings").alignment(Alignment::Center))
                .title_style(title_style);
            let inner_area = block.inner(area);
            block.render(area, buf);
            inner_area
        };
        let [main, footer] = Layout::vertical([Min(0), Length(1)]).areas(area);

        // Address
        let layout = layout(main, Direction::Vertical, &[1, 1, 3, 1, 1, 4]);
        let [_, addr_title, addr_content, _, auth_title, auth_content] = layout;
        ListElements::VDivider(String::from(" Address ")).render(addr_title, buf);
        Address {
            state: self.model.headers.borrow().addr.state.clone(),
            title: String::new(),
            selected: self.model.headers.borrow().tab == HeadersTab::Addr,
        }
        .render(addr_content, buf);

        // Authentication
        ListElements::VDivider(String::from(" Authentication ")).render(auth_title, buf);
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

        Line::from("Press Enter ")
            .right_aligned()
            .render(footer, buf);
    }
}
