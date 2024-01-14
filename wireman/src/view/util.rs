use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect_percentage(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

/// helper function to create a centered rect using up certain percentage in x and a certain
/// height of the available rect `r`
#[allow(clippy::cast_sign_loss)]
pub fn centered_rect_length(percent_x: u16, height: u16, r: Rect) -> Rect {
    let full_height = r.height;
    let percent_y = ((f64::from(height) / f64::from(full_height)) * 100.) as u16;
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub(super) fn crop_top(area: Rect, size: u16) -> Rect {
    Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: size,
    }
}
pub(super) fn crop_bot(area: Rect, size: u16) -> Rect {
    Rect {
        x: area.x,
        y: area.y + size,
        width: area.width,
        height: size,
    }
}
