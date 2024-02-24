use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};
use theme::Theme;
use tui_widget_list::{ListableWidget, ScrollAxis};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'a> {
    /// The items text
    pub text: Line<'a>,

    /// The items style
    pub style: Style,

    /// The highlight style
    pub highlight_style: Style,

    /// The current prefix. Changes when the item is selected.
    pub prefix: Option<&'a str>,
}

impl<'a> ListItem<'a> {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        let theme = Theme::global();
        Self {
            text: text.into(),
            style: theme.list.text,
            highlight_style: theme.list.focused,
            prefix: None,
        }
    }
}

impl ListableWidget for ListItem<'_> {
    fn size(&self, _: &ScrollAxis) -> usize {
        1
    }

    fn highlight(self) -> Self {
        let highlight_style = self.highlight_style;
        let mut item = self;
        item.prefix = Some(">>");
        item.style = highlight_style;
        item
    }
}

impl Widget for ListItem<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = if let Some(prefix) = self.prefix {
            prefix_text(self.text.clone(), prefix)
        } else {
            self.text.clone()
        };
        Paragraph::new(text).style(self.style).render(area, buf);
    }
}

fn prefix_text<'a>(text: Line<'a>, prefix: &'a str) -> Line<'a> {
    let mut line = text;
    line.spans.insert(0, Span::from(prefix));
    line
}
