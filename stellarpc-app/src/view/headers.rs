use super::root::layout;
use crate::model::headers::{AuthSelection, HeadersModel, HeadersSelection};
use edtui::{EditorState, EditorTheme, EditorView, StatusLine};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, StatefulWidget, Tabs, Widget},
};
use tui_widget_list::{List, ListState, Listable};

use super::theme::THEME;

/// The request and response tab
pub struct HeadersTab<'a> {
    model: &'a HeadersModel,
}

impl<'a> HeadersTab<'a> {
    pub fn new(model: &'a HeadersModel) -> Self {
        Self { model }
    }

    pub fn footer_keys() -> Vec<(&'static str, &'static str)> {
        vec![
            ("q", "Quit"),
            ("Tab", "Next Tab"),
            ("↑/k", "Up"),
            ("↓/j", "Down"),
            ("Enter", "Select"),
        ]
    }
}

impl Widget for HeadersTab<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let area = layout(area, Direction::Vertical, &[0, 1]);
        let mut items: Vec<ListElements> = Vec::new();

        // Address
        items.push(ListElements::SingleInput(SingleInput {
            state: self.model.address.state.clone(),
            title: String::from("Address"),
            selected: self.model.selected == HeadersSelection::Address,
        }));

        items.push(ListElements::VDivider(String::from("Authentication")));
        match self.model.auth.selected {
            AuthSelection::Bearer => items.push(ListElements::Authentication(Authentication {
                state: self.model.auth.bearer.state.clone(),
                title: String::from("Bearer"),
                selected: self.model.selected == HeadersSelection::Auth,
                selected_tag: 0,
            })),
            AuthSelection::Basic => items.push(ListElements::Authentication(Authentication {
                state: self.model.auth.basic.state.clone(),
                title: String::from("Basic"),
                selected: self.model.selected == HeadersSelection::Auth,
                selected_tag: 1,
            })),
        }

        let mut state = ListState::default();
        let list = List::new(items);
        list.render(area[0], buf, &mut state);

        // Show a combined status line for all editors
        StatusLine::default()
            .style_text(THEME.status_line.0)
            .style_line(THEME.status_line.1)
            .content(self.model.mode().name())
            .render(area[1], buf);
    }
}

#[allow(clippy::large_enum_variant)]
enum ListElements {
    VSpace(usize),
    VDivider(String),
    SingleInput(SingleInput),
    Authentication(Authentication),
}

impl Listable for ListElements {
    fn height(&self) -> usize {
        match &self {
            Self::VSpace(height) => *height,
            Self::VDivider(_) => 1,
            Self::SingleInput(inner) => inner.height(),
            Self::Authentication(inner) => inner.height(),
        }
    }
}
impl Widget for ListElements {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::VSpace(_) => {}
            Self::VDivider(title) => {
                Block::default()
                    .title(title)
                    .title_alignment(Alignment::Center)
                    .borders(Borders::TOP)
                    .render(area, buf);
            }
            Self::SingleInput(inner) => {
                let inner_area = area.inner(&Margin::new(1, 1));
                inner.render(inner_area, buf);
            }
            Self::Authentication(inner) => {
                let inner_area = area.inner(&Margin::new(1, 0));
                inner.render(inner_area, buf);
            }
        };
    }
}

#[derive(Clone)]
struct SingleInput {
    state: EditorState,
    title: String,
    selected: bool,
}

impl Listable for SingleInput {
    fn height(&self) -> usize {
        5
    }
}
impl Widget for SingleInput {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Left)
            .style(THEME.content)
            .title(self.title)
            .bold()
            .white();

        let mut theme = EditorTheme::default().status_line(None);
        if self.selected {
            block = block.border_type(BorderType::Double);
        }
        if !self.selected {
            theme = theme.cursor_style(EditorTheme::default().base_style());
        }
        theme = theme.block(block);

        EditorView::new(&mut self.state)
            .theme(theme)
            .render(area, buf);
    }
}

#[derive(Clone)]
struct Authentication {
    state: EditorState,
    title: String,
    selected: bool,
    selected_tag: usize,
}

impl Listable for Authentication {
    fn height(&self) -> usize {
        5
    }
}
impl Widget for Authentication {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = layout(area, Direction::Vertical, &[2, 0]);

        let titles = vec![" Bearer ", " Basic "];
        Tabs::new(titles)
            .style(THEME.tabs)
            .highlight_style(THEME.tabs_selected)
            .select(self.selected_tag)
            .divider("")
            .render(area[0], buf);

        SingleInput {
            state: self.state,
            title: self.title,
            selected: self.selected,
        }
        .render(area[1], buf);
    }
}
