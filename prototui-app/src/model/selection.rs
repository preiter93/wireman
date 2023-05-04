#![allow(clippy::module_name_repetitions)]
use core::{MethodDescriptor, ServiceDescriptor};

use crate::widgets::list_with_children::{ListWithChildrenState, SelectionLevel};
use std::cell::RefCell;
use std::rc::Rc;

use super::core_client::CoreClient;

#[derive(Clone)]
pub struct SelectionModel {
    /// Core client retrieves proto descriptors
    core_client: Rc<RefCell<CoreClient>>,
    /// The state of the services and methods list. Holds info about currently
    /// selected service and method and whether a service should be expanded.
    pub state: ListWithChildrenState,
    /// A list of proto services. Each service can hold a list of methods.
    pub items: Vec<ServiceWithMethods>,
}

/// Each service can hold a list of methods
#[derive(Clone)]
pub struct ServiceWithMethods {
    pub service: String,
    pub methods: Vec<String>,
}

impl SelectionModel {
    /// Returns a selection model. Requires the core client
    /// which retrieves the proto services and methods.
    pub fn new(core_client: Rc<RefCell<CoreClient>>) -> Self {
        let services = core_client.borrow_mut().get_services();
        let items = services
            .iter()
            .map(|service| ServiceWithMethods {
                service: service.full_name().to_string(),
                methods: core_client
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
            state.select_parent(Some(0));
        }

        Self {
            core_client,
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

    /// Select the next service.
    fn next_service(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected_parent() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select_parent(Some(i));
    }

    /// Select the previous service.
    fn previous_service(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected_parent() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select_parent(Some(i));
    }

    /// Select the next method.
    fn next_method(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let k = match (self.state.selected_parent(), self.state.selected_child()) {
            (Some(i), Some(j)) => {
                let items = &self.items[i].methods;
                if j >= items.len() - 1 {
                    0
                } else {
                    j + 1
                }
            }
            _ => 0,
        };
        self.state.select_child(Some(k));
    }

    /// Select the previous method.
    fn previous_method(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let k = match (self.state.selected_parent(), self.state.selected_child()) {
            (Some(i), Some(j)) => {
                let items = &self.items[i].methods;
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
        self.state.select_child(Some(k));
    }

    /// Return the description of the currently selected service
    fn get_selected_service(&self) -> Option<ServiceDescriptor> {
        self.state
            .selected_parent()
            .map(|i| self.core_client.borrow().get_services()[i].clone())
    }

    /// Return the descrption of the currently selected method
    pub fn get_selected_method(&self) -> Option<MethodDescriptor> {
        if let (Some(service), Some(i)) = (self.get_selected_service(), self.state.selected_child())
        {
            self.core_client
                .borrow()
                .get_methods(&service)
                .get(i)
                .cloned()
        } else {
            None
        }
    }

    /// Expands a service to show its methods. This is handled in the lists
    /// local state.
    pub fn expand_service(&mut self) {
        if let Some(service) = self.get_selected_service() {
            // do not expand if the service has no methods
            if self.core_client.borrow().get_methods(&service).is_empty() {
                return;
            }
            self.state.expand_parent();
            self.next_method();
        }
    }

    pub fn collapse_methods(&mut self) {
        self.state.collapse_children();
    }

    pub fn clear_method(&mut self) {
        self.state.select_child(None);
    }
}
