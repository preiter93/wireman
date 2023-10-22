use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};
use tui_widget_list::WidgetItem;

use crate::view::theme::THEME;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'a> {
    /// The items text
    pub text: Line<'a>,

    /// The items style
    pub style: Style,

    /// The current prefix. Changes when the item is selected.
    pub prefix: Option<&'a str>,
}

impl<'a> ListItem<'a> {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        Self {
            text: text.into(),
            style: Style::default(),
            prefix: None,
        }
    }

    // pub fn style(mut self, style: Style) -> Self {
    //     self.style = style;
    //     self
    // }

    // pub fn prefix(mut self, prefix: Option<&'a str>) -> Self {
    //     self.prefix = prefix;
    //     self
    // }

    // pub fn width(&self) -> usize {
    //     self.text.width()
    // }
}

impl<'a> WidgetItem for ListItem<'a> {
    fn highlighted(&self) -> Option<Self> {
        let mut item = self.clone();
        let highlight_style = THEME.list.selected;
        item.prefix = Some(">>");
        item.style = highlight_style;
        Some(item)
    }
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = if let Some(prefix) = self.prefix {
            prefix_text(self.text.clone(), prefix)
        } else {
            self.text.clone()
        };
        Paragraph::new(text).style(self.style).render(area, buf)
    }

    fn height(&self) -> usize {
        1
    }
}

fn prefix_text<'a>(text: Line<'a>, prefix: &'a str) -> Line<'a> {
    let mut line = text;
    line.spans.insert(0, Span::from(prefix));
    line
}
