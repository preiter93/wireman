#![allow(clippy::module_name_repetitions)]
use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::text::Spans;
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;
use ratatui::Frame;

use crate::commons::window_border;
use crate::model::editor::ErrorKind;
use crate::model::editor::Mode;
use crate::model::EditorModel;
use crate::theme;

// Draw a text editor widget
pub fn draw_text_editor<'a, B>(
    f: &mut Frame<B>,
    area: Rect,
    model: &mut EditorModel<'a>,
    block: Block<'a>,
) where
    B: Backend,
{
    // Removes the cursor line
    model.editor.set_cursor_line_style(Style::default());
    model.editor.set_block(block);

    // // Set the cursor style depending on the mode
    if model.mode == Mode::Normal {
        model.editor.set_cursor_style(
            Style::default()
                .fg(theme::COL_CURSOR_NORMAL_MODE)
                .add_modifier(theme::MOD_CURSOR_NORMAL_MODE),
        );
    } else {
        model.editor.set_cursor_style(
            Style::default()
                .fg(theme::COL_CURSOR_INSERT_MODE)
                .add_modifier(theme::MOD_CURSOR_INSERT_MODE),
        );
    }

    let resp_length = model
        .response
        .as_ref()
        .map(|r| r.lines().count())
        .unwrap_or(0);
    let (chunk1, chunk2, chunk3) = match (&model.error, &model.response) {
        (Some(_), Some(_)) => (
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(resp_length as u16 + 2),
        ),
        (None, Some(_)) => (
            Constraint::Min(0),
            Constraint::Length(0),
            Constraint::Length(resp_length as u16 + 2),
        ),
        (Some(_), None) => (
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(0),
        ),
        _ => (
            Constraint::Min(0),
            Constraint::Length(0),
            Constraint::Length(0),
        ),
    };

    let chunks = Layout::default()
        .constraints([chunk1, chunk2, chunk3].as_ref())
        .split(area);
    f.render_widget(model.editor.widget(), chunks[0]);
    if let Some(error) = &model.error {
        f.render_widget(error_widget(error.to_owned()), chunks[1]);
    }
    if let Some(response) = &model.response {
        f.render_widget(response_widget(&response.to_owned()), chunks[2]);
    }
}

/// Renders the gRPC response
fn response_widget<'a>(text: &'a str) -> Paragraph<'a> {
    Paragraph::new(Text::from(text))
        .block(window_border("Response", false))
        .wrap(Wrap { trim: false })
}

/// Renders any error in a separate box
fn error_widget<'a>(err: ErrorKind) -> Paragraph<'a> {
    let text = vec![Spans::from(Span::styled(
        err.msg,
        Style::default().fg(theme::COL_TEXT_ERROR),
    ))];
    let title = Span::styled(
        err.kind,
        Style::default()
            .fg(theme::COL_TEXT_ERROR)
            .add_modifier(theme::MOD_WINDOW_TITLE),
    );
    Paragraph::new(text)
        .block(Block::default().title(title).borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}
