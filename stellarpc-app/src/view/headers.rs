use crate::model::headers::{HeadersModel, HeadersSelection};
use ratatui::{
    prelude::*,
    style::Stylize,
    widgets::{Block, BorderType, Borders, Padding, Widget},
};
use tui_vim_editor::{Buffer as EditorBuffer, Editor};
use tui_widget_list::{WidgetItem, WidgetList};

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
        let block = Block::new()
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .style(THEME.content)
            .padding(Padding::new(1, 1, 1, 1));
        let items: Vec<ListElements> = vec![
            ListElements::SingleInput(SingleInput {
                buffer: self.model.address.buffer.clone(),
                title: "Address".to_string(),
                selected: self.model.selected == HeadersSelection::Address,
            }),
            ListElements::VSpace(2),
            ListElements::SingleInput(SingleInput {
                buffer: self.model.bearer.buffer.clone(),
                title: "Bearer".to_string(),
                selected: self.model.selected == HeadersSelection::Bearer,
            }),
        ];
        let mut list = WidgetList::new(items);
        list = list.block(block);
        list.render(area, buf);
    }
}

enum ListElements {
    VSpace(usize),
    SingleInput(SingleInput),
}

impl WidgetItem for ListElements {
    fn height(&self) -> usize {
        match &self {
            Self::VSpace(inner) => *inner,
            Self::SingleInput(inner) => inner.height(),
        }
    }
    fn render(&self, area: Rect, buf: &mut Buffer) {
        match &self {
            Self::VSpace(_) => {}
            Self::SingleInput(inner) => inner.render(area, buf),
        };
    }
}

#[derive(Clone)]
struct SingleInput {
    buffer: EditorBuffer,
    title: String,
    selected: bool,
}

impl WidgetItem for SingleInput {
    fn height(&self) -> usize {
        3
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Left)
            .style(THEME.content)
            .padding(Padding::new(1, 1, 0, 1));
        let mut input = Editor::new(&self.buffer);
        if self.selected {
            block = block.border_type(BorderType::Double);
        }
        if !self.selected {
            input.set_cursor_style(Style::default());
        }
        input.set_block(block.title(self.title.clone()).bold().white());
        input.render(area, buf);
    }
}
