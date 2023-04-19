use crate::model::selection::ItemWithChildren;
use crate::{
    commons::HelpActions,
    model::selection::SelectionModel,
    widgets::list_with_children::{ListWithChildrenState, SelectionLevel},
};
use core::MethodDescriptor;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub struct SelectionController {
    pub model: SelectionModel<String>,
}

impl SelectionController {
    pub fn new(model: SelectionModel<String>) -> Self {
        Self { model }
    }

    pub fn on_key(&mut self, key: KeyEvent) -> Option<MethodDescriptor> {
        let mut load_method = None;
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
            if self.is_focus_services() {
                self.on_key_services_focused(key, &mut load_method);
            } else {
                self.on_key_methods_focused(key, &mut load_method);
            }
        }
        load_method
    }

    /// Key bindings if services are focused
    fn on_key_services_focused(
        &mut self,
        key: KeyEvent,
        load_method: &mut Option<MethodDescriptor>,
    ) {
        match key.code {
            KeyCode::Enter => {
                self.model.expand_service();
                *load_method = self.model.get_selected_method();
            }
            _ => {}
        }
    }

    /// Key bindings if services are focused
    fn on_key_methods_focused(
        &mut self,
        key: KeyEvent,
        _load_method: &mut Option<MethodDescriptor>,
    ) {
        match key.code {
            KeyCode::Enter => self.model.collapse_methods(),
            _ => {}
        }
    }

    /// Returns whether the current selection focus are the Services
    fn is_focus_services(&self) -> bool {
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
    pub fn items(&self) -> &Vec<ItemWithChildren<String>> {
        &self.model.items
    }

    /// Return the state of the services list
    pub fn list_state(&mut self) -> &mut ListWithChildrenState {
        &mut self.model.state
    }
}
