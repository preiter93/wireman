use ratatui::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'a> {
    /// The items text
    pub text: Line<'a>,

    /// The style of the item
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
}

impl Widget for ListItem<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = if let Some(prefix) = self.prefix {
            prefix_text(self.text.clone(), prefix)
        } else {
            self.text.clone()
        };
        text.style(self.style).render(area, buf);
    }
}

fn prefix_text<'a>(text: Line<'a>, prefix: &'a str) -> Line<'a> {
    let mut line = text;
    line.spans.insert(0, Span::from(prefix));
    line
}
