#![allow(clippy::module_name_repetitions, clippy::cast_possible_truncation)]
use crate::model::MessagesModel;
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Widget;
use tui_vim_editor::Editor;
use tui_vim_editor::StatusLine;

use super::theme::THEME;

/// The request and response tab
pub struct MessagesTab<'a, 'b> {
    pub model: &'a MessagesModel<'b>,
    pub sub: usize,
}

impl<'a, 'b> MessagesTab<'a, 'b> {
    pub fn footer_keys() -> Vec<(&'static str, &'static str)> {
        vec![
            ("q", "Quit"),
            ("Tab", "Next Tab"),
            ("↑", "Up"),
            ("↓", "Down"),
            ("Enter", "Send"),
        ]
    }
}

impl Widget for MessagesTab<'_, '_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        // Layout
        let area = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        // Block
        let block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .style(THEME.content);

        // Request
        let buffer = &self.model.request.editor.buffer;
        let mut editor = Editor::new(buffer);
        let block = block.title("Request").bold().white();
        if self.sub == 0 {
            editor = editor
                .status_line(
                    StatusLine::default()
                        .style_text(THEME.status_line.0)
                        .style_line(THEME.status_line.1),
                )
                .block(block.clone().border_type(BorderType::Double));
        } else {
            editor = editor
                .cursor_style(Style::default())
                .block(block.clone().border_type(BorderType::Plain));
        }
        editor.render(area[0], buf);

        // Response
        let buffer = &self.model.response.editor.buffer;
        editor = Editor::new(buffer);
        let block = block.title("Response").bold().white();
        if self.sub == 1 {
            editor = editor
                .status_line(
                    StatusLine::default()
                        .style_text(THEME.status_line.0)
                        .style_line(THEME.status_line.1),
                )
                .block(block.border_type(BorderType::Double));
        } else {
            editor = editor
                .cursor_style(Style::default())
                .block(block.border_type(BorderType::Plain));
        }
        editor.render(area[1], buf);
    }
}

// /// TODO: Split into request/error/response
// pub fn render_messages<'a, B>(
//     f: &mut Frame<B>,
//     area: Rect,
//     controller: &mut Controller<'a>,
//     block: Block<'a>,
// ) where
//     B: Backend,
// {
//     let model = &controller.messages.request;
//
//     // Get the request text
//     let mut request = model.editor.clone();
//     request.update_style();
//     request.set_block(block);
//
//     // Get the error text from the model
//     let error = model.editor.get_error();
//
//     // Get response text from model
//     let response = controller.messages.response.editor.get_text_raw();
//
//     // Determine size of error and response widget
//     let resp_length = if response.is_empty() {
//         0
//     } else {
//         response.lines().count() as u16 + 2
//     };
//     let err_length = error.as_ref().map_or(0, |_| 3);
//     let chunks = Layout::default()
//         .constraints(
//             [
//                 // TODO: Add proper sizing
//                 Constraint::Min(10),
//                 Constraint::Length(err_length),
//                 Constraint::Length(resp_length),
//             ]
//             .as_ref(),
//         )
//         .split(area);
//
//     // Render request window
//     f.render_widget(&request, area);
//
//     // Render error window
//     if let Some(error) = &error {
//         f.render_widget(error_widget(error.clone()), chunks[1]);
//     }
//
//     // Render response window
//     if resp_length > 0 {
//         f.render_widget(response_widget(&response), chunks[2]);
//     }
// }
//
// /// Renders the grpc response
// fn response_widget(text: &str) -> Paragraph {
//     Paragraph::new(Text::from(text))
//         .block(window_border("Response", false))
//         .wrap(Wrap { trim: false })
// }
//
// /// Renders any error in a separate box
// fn error_widget<'a>(err: ErrorKind) -> Paragraph<'a> {
//     let text = vec![Line::from(Span::styled(
//         err.msg,
//         Style::default().fg(theme::COL_TEXT_ERROR),
//     ))];
//     let title = Span::styled(
//         err.kind,
//         Style::default()
//             .fg(theme::COL_TEXT_ERROR)
//             .add_modifier(theme::MOD_WINDOW_TITLE),
//     );
//     Paragraph::new(text)
//         .block(Block::default().title(title).borders(Borders::ALL))
//         .alignment(Alignment::Center)
//         .wrap(Wrap { trim: true })
// }
