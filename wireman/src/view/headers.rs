use super::root::layout;
use crate::{
    model::headers::{AuthSelection, HeadersModel, HeadersTab},
    widgets::editor::{view_single_selected, view_single_unselected},
    widgets::kv::KV,
};
use edtui::{EditorMode, EditorState, EditorStatusLine};
use ratatui::{
    prelude::*,
    widgets::{Block, StatefulWidget, Tabs, Widget},
};
use theme::Theme;
use tui_widget_list::{ListBuilder, ListState, ListView};

pub struct HeadersPage<'a> {
    model: &'a HeadersModel,
}

impl<'a> HeadersPage<'a> {
    pub fn new(model: &'a HeadersModel) -> Self {
        Self { model }
    }

    pub fn footer_keys(model: &'a HeadersModel) -> Vec<(&'static str, &'static str)> {
        let mut keys = vec![
            ("^c", "Quit"),
            ("Tab", "Next Page"),
            ("j/k/h/l", "Navigate"),
        ];
        if model.mode() == EditorMode::Insert {
            keys.push(("Esc", "Normal Mode"));
        } else {
            keys.push(("i", "Insert Mode"));
        }
        keys.push(("<C-a>", "Add Header"));
        keys.push(("?", "Help"));

        keys
    }
}

impl Widget for HeadersPage<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let theme = theme::Theme::global();
        let sl = u16::from(!theme.editor.hide_status_line);
        let [addr_title, addr_content, _, auth_title, auth_content, _, meta_title, meta_content, status] =
            layout(area, Direction::Vertical, &[1, 3, 1, 1, 4, 1, 1, 0, sl]);

        // Address
        ListElements::VDivider(String::from(" Address ")).render(addr_title, buf);
        Address {
            state: self.model.addr.state.clone(),
            title: String::new(),
            selected: self.model.tab == HeadersTab::Addr,
        }
        .render(addr_content, buf);

        // Authentication
        ListElements::VDivider(String::from(" Authentication ")).render(auth_title, buf);
        let body = match self.model.auth.selected {
            AuthSelection::Bearer => Authentication {
                state: self.model.auth.bearer.state.clone(),
                title: String::new(),
                selected: self.model.tab == HeadersTab::Auth,
                selected_tag: 0,
            },
            AuthSelection::Basic => Authentication {
                state: self.model.auth.basic.state.clone(),
                title: String::new(),
                selected: self.model.tab == HeadersTab::Auth,
                selected_tag: 1,
            },
        };
        body.render(auth_content, buf);

        // Metadata
        if !self.model.meta.is_hidden() {
            ListElements::VDivider(String::from(" Headers ")).render(meta_title, buf);
            let headers = &self.model.meta.headers;
            let index = self.model.meta.selected;
            Metadata {
                content: headers
                    .iter()
                    .enumerate()
                    .map(|(i, x)| KV {
                        key: x.0.state.clone(),
                        val: x.1.state.clone(),
                        key_selected: (self.model.tab == HeadersTab::Meta)
                            && index.is_some_and(|x| x.row == i && x.col == 0),
                        val_selected: (self.model.tab == HeadersTab::Meta)
                            && index.is_some_and(|x| x.row == i && x.col == 1),
                    })
                    .collect(),
                selected_row: index.map(|x| x.row),
            }
            .render(meta_content, buf);
        }

        // Show a combined status line for all editors
        if !theme.editor.hide_status_line {
            EditorStatusLine::default()
                .style_text(theme.editor.status_text)
                .style_line(theme.editor.status_line)
                .mode(self.model.mode().name())
                .render(status, buf);
        }
    }
}

#[allow(clippy::large_enum_variant)]
pub enum ListElements {
    VSpace(usize),
    VDivider(String),
}

impl Widget for ListElements {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = theme::Theme::global();
        match self {
            Self::VSpace(_) => {}
            Self::VDivider(title) => {
                Block::default()
                    .title(title)
                    .title_alignment(Alignment::Center)
                    .title_style(theme.headers.titles)
                    .render(area, buf);
            }
        };
    }
}

#[derive(Clone)]
pub struct Address {
    pub state: EditorState,
    pub title: String,
    pub selected: bool,
}

impl Widget for Address {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        if self.selected {
            view_single_selected(&mut self.state, &self.title).render(area, buf);
        } else {
            view_single_unselected(&mut self.state, &self.title).render(area, buf);
        }
    }
}

#[derive(Clone)]
pub struct Authentication {
    pub state: EditorState,
    pub title: String,
    pub selected: bool,
    pub selected_tag: usize,
}

impl Widget for Authentication {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::global();
        let [title, content] = layout(area, Direction::Vertical, &[1, 0]);

        let titles = vec![" Bearer ", " Basic "];
        Tabs::new(titles)
            .style(theme.headers.tabs.0)
            .highlight_style(theme.headers.tabs.1)
            .select(self.selected_tag)
            .divider("")
            .render(title.inner(Margin::new(0, 0)), buf);

        if self.selected {
            view_single_selected(&mut self.state, self.title).render(content, buf);
        } else {
            view_single_unselected(&mut self.state, self.title).render(content, buf);
        }
    }
}

#[derive(Clone)]
struct Metadata {
    content: Vec<KV>,
    selected_row: Option<usize>,
}

impl Widget for Metadata {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        state.selected = self.selected_row;
        let item_count = self.content.len();
        let list = ListView::new(
            ListBuilder::new(move |context| {
                let item = self.content[context.index].clone();
                (item, 3)
            }),
            item_count,
        );
        list.render(area, buf, &mut state);
    }
}
