use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};
use theme::Theme;
use tui_widget_list::{ListWidget, RenderContext};

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
impl ListWidget for ListItem<'_> {
    fn pre_render(mut self, context: &RenderContext) -> (Self, u16)
    where
        Self: Sized,
    {
        let main_axis_size = 1;

        if context.is_selected {
            self.prefix = Some(">>");
            self.style = self.highlight_style;
        }

        (self, main_axis_size)
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
