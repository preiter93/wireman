use core::{MethodDescriptor, ServiceDescriptor};

use crate::widgets::list_with_children::{ListWithChildrenState, SelectionLevel};
use std::cell::RefCell;
use std::rc::Rc;

use super::core_client::CoreClient;

#[derive(Clone)]
pub struct SelectionModel<T> {
    analyzer: Rc<RefCell<CoreClient>>,
    pub state: ListWithChildrenState,
    pub items: Vec<ItemWithChildren<T>>,
}

#[derive(Clone)]
pub struct ItemWithChildren<T> {
    pub parent: T,
    pub children: Vec<T>,
}

impl SelectionModel<String> {
    /// Returns a selection model. Requires the core client
    /// which retrieves the proto services and methods.
    pub fn new(analyzer: Rc<RefCell<CoreClient>>) -> Self {
        let services = analyzer.borrow_mut().get_services();
        let items = services
            .iter()
            .map(|service| ItemWithChildren::<String> {
                parent: service.full_name().to_string(),
                children: analyzer
                    .borrow_mut()
                    .get_methods(service)
                    .iter()
                    .map(|method| method.name().to_string())
                    .collect(),
            })
            .collect();

        // Preselect first service
        let mut state = ListWithChildrenState::default();
        if !services.is_empty() {
            state.select(Some(0));
        }

        Self {
            analyzer,
            state,
            items,
        }
    }

    /// Select the next service or method, depending on the current
    /// selection level.
    pub fn next(&mut self) {
        match self.state.selection_level() {
            SelectionLevel::Parent => self.next_service(),
            SelectionLevel::Children => self.next_method(),
        }
    }

    /// Select the previous service or method, depending on the current
    /// selection level.
    pub fn previous(&mut self) {
        match self.state.selection_level() {
            SelectionLevel::Parent => self.previous_service(),
            SelectionLevel::Children => self.previous_method(),
        }
    }

    pub fn next_service(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous_service(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn next_method(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let k = match (self.state.selected(), self.state.selected_sub()) {
            (Some(i), Some(j)) => {
                let items = &self.items[i].children;
                if j >= items.len() - 1 {
                    0
                } else {
                    j + 1
                }
            }
            _ => 0,
        };
        self.state.select_sub(Some(k));
    }

    pub fn previous_method(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let k = match (self.state.selected(), self.state.selected_sub()) {
            (Some(i), Some(j)) => {
                let items = &self.items[i].children;
                if items.is_empty() {
                    0
                } else if j == 0 {
                    items.len() - 1
                } else {
                    j - 1
                }
            }
            _ => 0,
        };

        self.state.select_sub(Some(k));
    }

    /// Return the description of the currently selected service
    fn get_selected_service(&self) -> Option<ServiceDescriptor> {
        if let Some(i) = self.state.selected() {
            Some(self.analyzer.borrow().get_services()[i].clone())
        } else {
            None
        }
    }

    /// Return the descrption of the currently selected method
    pub fn get_selected_method(&self) -> Option<MethodDescriptor> {
        if let (Some(service), Some(i)) = (self.get_selected_service(), self.state.selected_sub()) {
            self.analyzer.borrow().get_methods(&service).get(i).cloned()
        } else {
            None
        }
    }

    /// Expands a service to show its methods. This is handled in the lists
    /// local state.
    pub fn expand_service(&mut self) {
        if let Some(service) = self.get_selected_service() {
            // do not expand if the service has no methods
            if self.analyzer.borrow().get_methods(&service).is_empty() {
                return;
            }
            self.state.expand_selected();
            self.next_method();
        }
    }

    pub fn collapse_methods(&mut self) {
        self.state.collapse();
    }
}
