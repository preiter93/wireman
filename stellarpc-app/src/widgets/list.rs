use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Span, Spans},
    widgets::{Paragraph, Widget},
};
use tui_widget_list::WidgetListItem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'a> {
    /// The items text
    pub text: Spans<'a>,

    /// The items style
    pub style: Style,

    /// The current prefix. Changes when the item is selected.
    pub prefix: Option<&'a str>,
}

impl<'a> ListItem<'a> {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<Spans<'a>>,
    {
        Self {
            text: text.into(),
            style: Style::default(),
            prefix: None,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn prefix(mut self, prefix: Option<&'a str>) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn width(&self) -> usize {
        self.text.width()
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn as_widget(self) -> Paragraph<'a> {
        let text = if let Some(prefix) = self.prefix {
            prefix_text(self.text, prefix)
        } else {
            self.text
        };
        Paragraph::new(text).style(self.style)
    }

    fn modify_selected(
        mut item: WidgetListItem<Self>,
        selected: Option<bool>,
    ) -> WidgetListItem<Self> {
        if let Some(selected) = selected {
            if selected {
                let highlight_style = Style::default()
                    .bg(crate::theme::COL_LIST_HIGHLIGHTED_METHOD_BG)
                    .fg(crate::theme::COL_LIST_HIGHLIGHTED_METHOD_FG);
                item.content.prefix = Some("  >>");
                item.content.style = highlight_style;
            } else {
                item.content.prefix = Some("    ");
            }
        }
        item
    }
}

impl<'a> From<ListItem<'a>> for WidgetListItem<ListItem<'a>> {
    fn from(val: ListItem<'a>) -> Self {
        Self::new(val, 1_u16).modify_fn(ListItem::modify_selected)
    }
}

impl<'a> Widget for ListItem<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.as_widget().render(area, buf);
    }
}

fn prefix_text<'a>(text: Spans<'a>, prefix: &'a str) -> Spans<'a> {
    let mut line = text;
    line.0.insert(0, Span::from(prefix));
    line
}
