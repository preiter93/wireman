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
        let mut keys = vec![("^c", "Quit"), ("Tab", "Proceed"), ("j/k/h/l", "Navigate")];
        if model.mode() == EditorMode::Insert {
            keys.push(("Esc", "Normal"));
        } else {
            keys.push(("i", "Insert"));
        }
        keys.push(("?", "Help"));

        keys
    }
}

impl Widget for HeadersPage<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let theme = theme::Theme::global();
        let sl = u16::from(!theme.hide_status);
        let is_auth_selected = self.model.tab == HeadersTab::Auth;
        let [addr_title, addr_content, _, auth_title, auth_content, _, meta_title, meta_content, status] =
            layout(area, Direction::Vertical, &[1, 3, 1, 1, 4, 1, 1, 0, sl]);

        // Address
        let style = if self.model.tab == HeadersTab::Addr {
            theme.title.focused
        } else {
            theme.title.unfocused
        };
        Block::default()
            .title(String::from(" Address "))
            .title_alignment(Alignment::Center)
            .title_style(style)
            .render(addr_title, buf);

        Address {
            state: self.model.addr.state.clone(),
            title: String::new(),
            selected: self.model.tab == HeadersTab::Addr,
        }
        .render(addr_content, buf);

        // Authentication
        let style = if self.model.tab == HeadersTab::Auth {
            theme.title.focused
        } else {
            theme.title.unfocused
        };
        Block::default()
            .title(String::from(" Authentication "))
            .title_alignment(Alignment::Center)
            .title_style(style)
            .render(auth_title, buf);

        let body = match self.model.auth.selected {
            AuthSelection::Bearer => Authentication {
                state: self.model.auth.bearer.state.clone(),
                title: String::new(),
                selected: is_auth_selected,
                selected_tag: 0,
            },
            AuthSelection::Basic => Authentication {
                state: self.model.auth.basic.state.clone(),
                title: String::new(),
                selected: is_auth_selected,
                selected_tag: 1,
            },
        };
        body.render(auth_content, buf);

        // Metadata
        let style = if self.model.tab == HeadersTab::Meta {
            theme.title.focused
        } else {
            theme.title.unfocused
        };
        Block::default()
            .title(String::from(" Headers "))
            .title_alignment(Alignment::Center)
            .title_style(style)
            .render(meta_title, buf);

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
                    show_border_title: (self.model.tab == HeadersTab::Meta)
                        && index.is_some_and(|x| x.row == i),
                })
                .collect(),
            selected_row: index.map(|x| x.row),
            focused: self.model.tab == HeadersTab::Meta,
        }
        .render(meta_content, buf);

        // Show a combined status line for all editors
        if !theme.hide_status {
            EditorStatusLine::default()
                .style_text(theme.highlight.unfocused.reversed())
                .style_line(theme.base.unfocused)
                .mode(self.model.mode().name())
                .render(status, buf);
        }
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
        let [title, content] = layout(area, Direction::Vertical, &[1, 3]);

        let (highlight_style, style) = if self.selected {
            (theme.title.focused, theme.base.unfocused)
        } else {
            (theme.title.unfocused, theme.base.unfocused)
        };

        let mut info_text = "";
        if self.selected {
            if self.selected_tag == 0 {
                info_text = "Without \"Bearer\".";
            } else {
                info_text = "Base64 encoded username:password. Without \"Basic\".";
            };
        }
        if info_text.len() as u16 > area.width - 28 {
            info_text = "";
        }

        let [left, right] = layout(title, Direction::Horizontal, &[0, info_text.len() as u16]);

        if self.selected {
            Line::from(info_text)
                .style(theme.base.unfocused)
                .render(right, buf);
        }

        let titles = if self.selected {
            vec![" Bearer (H) ", " Basic (L) "]
        } else {
            vec![" Bearer ", " Basic "]
        };
        Tabs::new(titles)
            .style(style)
            .highlight_style(highlight_style)
            .select(self.selected_tag)
            .divider("")
            .render(left.inner(Margin::new(0, 0)), buf);

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
    focused: bool,
}

impl Widget for Metadata {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::global();

        let list_height = self.content.len() as u16 * 3;
        let [main, text] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(list_height),
                Constraint::Length(u16::from(self.focused) * 2),
            ])
            .areas(area);

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
        list.render(main, buf, &mut state);
        if self.focused {
            let [add, del] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)])
                .areas(text);
            Line::from(vec![
                Span::from("<C-a>: ").style(theme.title.unfocused),
                Span::from("Add header").style(theme.base.unfocused),
            ])
            .render(add, buf);
            Line::from(vec![
                Span::from("<C-d>: ").style(theme.title.unfocused),
                Span::from("Delete header").style(theme.base.unfocused),
            ])
            .render(del, buf);
        }
    }
}
