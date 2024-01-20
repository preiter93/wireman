pub mod debug;
pub mod editor;

use std::slice::Iter;

#[macro_export]
macro_rules! control_with_key {
    ($code:expr) => {
        (KeyModifiers::CONTROL, KeyCode::Char($code))
    };
}

type HelpAction = (&'static str, &'static str);

/// A list of help actions. Only used for displaying.
pub struct HelpActions {
    items: Vec<(&'static str, &'static str)>,
}

impl HelpActions {
    /// Returns empty map
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Returns actions from items
    pub fn from_items(items: Vec<HelpAction>) -> Self {
        Self { items }
    }

    /// Insert a action
    pub fn insert(&mut self, key: &'static str, action: &'static str) {
        self.items.push((key, action));
    }

    /// Iterate over the actions
    pub fn iter(&self) -> Iter<'_, (&str, &str)> {
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
        actions.insert("A", "Toggle address");
        actions.insert("M", "Toggle metadata");
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
