use edtui::{EditorStatusLine, EditorTheme, EditorView};
use ratatui::{
    layout::{Alignment, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{
    model::configuration::{ConfigurationModel, Message},
    view::util::spans_from_keys,
};

pub struct ConfigurationDialog<'a> {
    pub model: &'a mut ConfigurationModel,
}

impl ConfigurationDialog<'_> {
    pub fn footer_keys() -> Vec<(&'static str, &'static str)> {
        let keys = vec![("<C-e>", "Quit"), ("<C-s>", "Save")];
        keys
    }
}

impl Widget for ConfigurationDialog<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        use ratatui::layout::Constraint::{Length, Min};
        let theme = theme::Theme::global();

        let [area, footer] = Layout::vertical([Min(0), Length(1)]).areas(area);

        let block = Block::new()
            .borders(Borders::ALL)
            .style(theme.base.style)
            .title_style(theme.border.text.1)
            .border_style(theme.border.border.1)
            .border_type(theme.border.border_type.1)
            .title("|Configuration|")
            .title_alignment(Alignment::Center);
        let inner_area = block.inner(area);
        block.render(area, buf);

        let [main, status, info_line] =
            Layout::vertical([Min(0), Length(1), Length(1)]).areas(inner_area);

        if let Some(editor) = &mut self.model.editor {
            let view = EditorView::new(&mut editor.state).theme(
                EditorTheme::default()
                    .base(theme.editor.text)
                    .cursor_style(theme.editor.cursor)
                    .selection_style(theme.editor.selection)
                    .hide_status_line(),
            );
            view.render(main, buf);

            let status_line = EditorStatusLine::default()
                .style_text(theme.editor.status_text)
                .style_line(theme.editor.status_line)
                .mode(editor.state.mode.name());

            status_line.render(status, buf);
        }

        if let Some(ref message) = self.model.message {
            let line = match message {
                Message::Info(s) => Line::from(s.as_str()),
                Message::Success(s) => Line::from(s.as_str()).green(),
                Message::Error(s) => Line::from(s.as_str()).red(),
            };
            line.render(info_line, buf);
        };

        let keys = Self::footer_keys();
        let spans = spans_from_keys(&keys);
        Paragraph::new(Line::from(spans))
            .style(theme.base.style)
            .alignment(Alignment::Center)
            .render(footer, buf);
    }
}
