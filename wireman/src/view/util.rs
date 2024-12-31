use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::Span,
};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect_percentage(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let [_, popup, _] = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .areas(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup)[1]
}

/// helper function to create a centered rect using up certain percentage in x and a certain
/// height of the available rect `r`
#[allow(clippy::cast_sign_loss)]
pub fn centered_rect_length(percent_x: u16, height: u16, r: Rect) -> Rect {
    centered_rect_percentage(
        percent_x,
        ((f64::from(height) / f64::from(r.height)) * 100.) as u16,
        r,
    )
}

pub fn spans_from_keys(keys: &[(&'static str, &'static str)]) -> Vec<Span<'static>> {
    let theme = theme::Theme::global();
    keys.iter()
        .flat_map(|(key, desc)| {
            let key = Span::styled(format!("{key}: "), theme.title.unfocused);
            let desc = Span::styled(format!("{desc}  "), theme.base.unfocused);
            [key, desc]
        })
        .collect()
}
