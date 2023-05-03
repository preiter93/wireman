#![allow(clippy::module_name_repetitions, clippy::cast_possible_truncation)]
use crate::commons::editor::ErrorKind;
use crate::commons::window_border;
use crate::controller::MessagesController;
use crate::theme;
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

// Draw a text editor widget
pub fn draw_request<'a, B>(
    f: &mut Frame<B>,
    area: Rect,
    controller: &mut MessagesController<'a>,
    block: Block<'a>,
) where
    B: Backend,
{
    // Set the request editors ui
    let mut request = controller.get_editor_request().clone();
    request.set_style_default();
    request.set_block(block);

    // Get the error text from the model
    let error = controller.get_error();

    // Get response text from model
    let response = controller.response_string();

    // Determine size of error and response widget
    let resp_length = if response.is_empty() {
        0
    } else {
        response.lines().count() as u16 + 2
    };
    let err_length = error.as_ref().map_or(0, |_| 3);
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Length(err_length),
                Constraint::Length(resp_length),
            ]
            .as_ref(),
        )
        .split(area);

    // Render request window
    f.render_widget(request.widget(), chunks[0]);

    // Render error window
    if let Some(error) = &error {
        f.render_widget(error_widget(error.clone()), chunks[1]);
    }

    // Render response window
    if resp_length > 0 {
        f.render_widget(response_widget(&response), chunks[2]);
    }
}

/// Renders the grpc response
fn response_widget(text: &str) -> Paragraph {
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
