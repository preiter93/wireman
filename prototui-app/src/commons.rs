use std::collections::{btree_map::Iter, BTreeMap};

use ratatui::{
    style::Style,
    text::Span,
    widgets::{Block, Borders},
};

use crate::theme;

/// Default border window. Highlighted borders can have
/// a different style.
pub fn window_border(title: &str, highlighted: bool) -> Block {
    let (border_style, border_type) = if highlighted {
        (
            Style::default()
                .fg(theme::COL_WINDOW_BORDER_HIGHLIGHTED_FG)
                .bg(theme::COL_WINDOW_BORDER_HIGHLIGHTED_BG),
            theme::TYP_BORDER_HIGHLIGHTED,
        )
    } else {
        (
            Style::default()
                .fg(theme::COL_WINDOW_BORDER_FG)
                .bg(theme::COL_WINDOW_BORDER_BG),
            theme::TYP_BORDER,
        )
    };

    Block::default()
        .title(Span::styled(
            title,
            Style::default()
                .fg(theme::COL_WINDOW_TITLE)
                .add_modifier(theme::MOD_WINDOW_TITLE),
        ))
        .borders(Borders::ALL)
        .style(
            Style::default()
                .fg(theme::COL_TEXT_NORMAL)
                .bg(theme::COL_BACKGROUND),
        )
        .border_type(border_type)
        .border_style(border_style)
}

/// A list of help actions. Only used for displaying.
pub struct HelpActions {
    items: BTreeMap<&'static str, &'static str>,
}

impl HelpActions {
    /// Returns empty map
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    /// Insert a action
    pub fn insert(&mut self, key: &'static str, action: &'static str) {
        self.items.insert(key, action);
    }

    /// Iterate over the actions
    pub fn iter(&self) -> Iter<'_, &str, &str> {
        self.items.iter()
    }

    /// Returns the number of actions
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl Default for HelpActions {
    /// The default helper actions, include q for quit, tab for switching tabs
    /// and h for displaying the help widget
    fn default() -> Self {
        let mut actions = Self::new();
        actions.insert("q", "Quit");
        actions.insert("H", "Toggle help");
        actions
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_iter() {
        // given
        let mut actions = HelpActions::new();
        actions.insert("key", "help");

        // then
        for (key, help) in actions.iter() {
            assert_eq!(*key, "key");
            assert_eq!(*help, "help");
        }
    }
}
