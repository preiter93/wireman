use super::root::layout;
use crate::model::headers::{AuthSelection, HeadersModel, HeadersSelection};
use edtui::{EditorState, StatusLine};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, StatefulWidget, Tabs, Widget},
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

    pub fn footer_keys(&self) -> Vec<(&'static str, &'static str)> {
        let mut footer = vec![
            ("q", "Quit"),
            ("Tab", "Next Tab"),
            ("↑/k", "Up"),
            ("↓/j", "Down"),
        ];
        if self.model.meta.headers().is_empty() || self.model.selected == HeadersSelection::Meta {
            footer.push(("^h", "Add Header"));
        }
        if self.model.selected == HeadersSelection::Meta {
            footer.push(("^d", "Remove Header"));
        }
        footer
    }
}

impl Widget for HeadersTab<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let area = layout(area, Direction::Vertical, &[5, 1, 5, 3, 0, 1]);

        // Address
        SingleInput {
            state: self.model.addr.state.clone(),
            title: String::from("Address"),
            selected: self.model.selected == HeadersSelection::Addr,
        }
        .render(area[0].inner(&Margin::new(0, 1)), buf);

        // Authentication
        ListElements::VDivider(String::from("Authentication")).render(area[1], buf);
        let body = match self.model.auth.selected {
            AuthSelection::Bearer => Authentication {
                state: self.model.auth.bearer.state.clone(),
                title: String::new(),
                selected: self.model.selected == HeadersSelection::Auth,
                selected_tag: 0,
            },
            AuthSelection::Basic => Authentication {
                state: self.model.auth.basic.state.clone(),
                title: String::new(),
                selected: self.model.selected == HeadersSelection::Auth,
                selected_tag: 1,
            },
        };
        body.render(area[2], buf);

        // Metadata
        let meta = self.model.meta.headers();
        if !meta.is_empty() {
            ListElements::VDivider(String::from("Headers"))
                .render(area[3].inner(&Margin::new(0, 1)), buf);
            let index = self.model.meta.selected_index();
            Metadata {
                content: meta
                    .iter()
                    .enumerate()
                    .map(|(i, x)| KV {
                        key: x.0.state.clone(),
                        val: x.1.state.clone(),
                        key_selected: index.map_or(false, |x| x.row == i && x.col == 0),
                        val_selected: index.map_or(false, |x| x.row == i && x.col == 1),
                    })
                    .collect(),
            }
            .render(area[4], buf);
        }

        // Show a combined status line for all editors
        StatusLine::default()
            .style_text(THEME.status_line.0)
            .style_line(THEME.status_line.1)
            .content(self.model.mode().name())
            .render(area[5], buf);
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
        if self.selected {
            super::editor::view_single_selected(&mut self.state, &self.title).render(area, buf);
        } else {
            super::editor::view_single_unselected(&mut self.state, &self.title).render(area, buf);
        }
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

#[derive(Clone)]
struct Metadata {
    content: Vec<KV>,
    // selected: Index2,
}

impl Widget for Metadata {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        let list = List::new(self.content);
        list.render(area, buf, &mut state);
    }
}

#[derive(Clone)]
struct KV {
    key: EditorState,
    val: EditorState,
    key_selected: bool,
    val_selected: bool,
}

impl KV {
    fn new(key: &EditorState, val: &EditorState) -> Self {
        Self {
            key: key.clone(),
            val: val.clone(),
            key_selected: false,
            val_selected: false,
        }
    }
}

impl Listable for KV {
    fn height(&self) -> usize {
        3
    }
}
impl Widget for KV {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        SingleInput {
            state: self.key,
            title: String::new(),
            selected: self.key_selected,
        }
        .render(area[0], buf);

        SingleInput {
            state: self.val,
            title: String::new(),
            selected: self.val_selected,
        }
        .render(area[1], buf);
        // let titles = vec![" Bearer ", " Basic "];
        // Tabs::new(titles)
        //     .style(THEME.tabs)
        //     .highlight_style(THEME.tabs_selected)
        //     .select(self.selected_tag)
        //     .divider("")
        //     .render(area[0], buf);
        //
        // SingleInput {
        //     state: self.state,
        //     title: self.title,
        //     selected: self.selected,
        // }
        // .render(area[1], buf);
    }
}
