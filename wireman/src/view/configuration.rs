use edtui::{EditorMode, EditorStatusLine, EditorTheme, EditorView};
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
    pub fn footer_keys(normal_mode: bool) -> Vec<(&'static str, &'static str)> {
        let mut keys = vec![("Esc", "Quit"), ("<C-s>", "Save")];
        if normal_mode {
            keys.push(("<C-e>", "Open in Editor"));
        }
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
            .style(theme.base.focused)
            .title_style(theme.title.focused)
            .border_style(theme.border.focused)
            .border_type(theme.border.border_type)
            .title(" Configuration ")
            .title_alignment(Alignment::Center);
        let inner_area = block.inner(area);
        block.render(area, buf);

        let [main, status, info_line] =
            Layout::vertical([Min(0), Length(1), Length(1)]).areas(inner_area);

        let normal_mode = self
            .model
            .editor
            .as_ref()
            .is_some_and(|e| e.state.mode == EditorMode::Normal);

        if let Some(editor) = &mut self.model.editor {
            let view = EditorView::new(&mut editor.state).theme(
                EditorTheme::default()
                    .base(theme.base.focused)
                    .cursor_style(theme.base.focused.reversed())
                    .selection_style(theme.highlight.focused.reversed())
                    .hide_status_line(),
            );
            view.render(main, buf);

            let status_line = EditorStatusLine::default()
                .style_text(theme.highlight.unfocused.reversed())
                .style_line(theme.base.unfocused)
                .mode(editor.state.mode.name());

            status_line.render(status, buf);
        }

        if let Some(ref message) = self.model.message {
            let line = match message {
                Message::Info(s) => Line::from(s.as_str()),
                Message::Success(s) => Line::from(s.as_str()).green(),
                Message::Error(s) => Line::from(s.as_str()).red(),
            };
            line.style(theme.base.unfocused).render(info_line, buf);
        };

        let keys = Self::footer_keys(normal_mode);
        let spans = spans_from_keys(&keys);
        Paragraph::new(Line::from(spans))
            .style(theme.base.focused)
            .alignment(Alignment::Center)
            .render(footer, buf);
    }
}
