#![allow(clippy::module_name_repetitions)]
use crate::model::selection::ServiceWithMethods;
use crate::{
    commons::HelpActions,
    model::selection::SelectionModel,
    widgets::list_with_children::{ListWithChildrenState, SelectionLevel},
};
use core::MethodDescriptor;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

/// Selection controller manipulates the services and methods
/// list state on user input.
pub struct SelectionController {
    model: SelectionModel,
}

impl SelectionController {
    /// Instantiate a Controller from the model.
    pub fn new(model: SelectionModel) -> Self {
        Self { model }
    }

    /// Handle user input.
    pub fn on_key(&mut self, key: KeyEvent) -> (Option<MethodDescriptor>, bool) {
        let mut load_method = None;
        let mut clear_method = false;
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Down | KeyCode::Char('j') => {
                    self.model.next();
                    load_method = self.model.get_selected_method();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.model.previous();
                    load_method = self.model.get_selected_method();
                }
                _ => {}
            }
            // Handle key input depending if services or methods
            // are currently in the selection focus.
            if self.services_selected() {
                self.on_key_services_focused(key, &mut load_method);
            } else {
                self.on_key_methods_focused(key, &mut clear_method);
            }
        }
        (load_method, clear_method)
    }

    /// Key bindings if services are focused
    fn on_key_services_focused(
        &mut self,
        key: KeyEvent,
        load_method: &mut Option<MethodDescriptor>,
    ) {
        if key.code == KeyCode::Enter {
            self.model.expand_service();
            *load_method = self.model.get_selected_method();
        }
    }

    /// Key bindings if services are focused
    fn on_key_methods_focused(&mut self, key: KeyEvent, clear_method: &mut bool) {
        if key.code == KeyCode::Enter {
            self.model.collapse_methods();
            self.model.unselect_method();
            *clear_method = true;
        }
    }

    /// Returns whether the current selection focus are the Services
    fn services_selected(&self) -> bool {
        self.model.state.selection_level() == SelectionLevel::Parent
    }

    /// Return a map of help actions. This is displayed in the
    /// helper wndow.
    pub fn help(&self) -> HelpActions {
        let mut actions = HelpActions::default();
        actions.insert("Tab", "Go to Request");
        match self.model.state.selection_level() {
            SelectionLevel::Parent => actions.insert("Enter", "Select service"),
            SelectionLevel::Children => actions.insert("Enter", "Collapse"),
        }
        actions.insert("j/↓", "down");
        actions.insert("k/↑", "up");
        actions
    }

    /// Return the istems of the services list
    pub fn items(&self) -> &Vec<ServiceWithMethods> {
        &self.model.items
    }

    /// Return the state of the services list
    pub fn list_state(&mut self) -> &mut ListWithChildrenState {
        &mut self.model.state
    }
}
