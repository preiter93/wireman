#![allow(clippy::module_name_repetitions)]
use core::{MethodDescriptor, ServiceDescriptor};

use std::cell::RefCell;
use std::rc::Rc;

use tui_widget_list::ListState;

use super::core_client::CoreClient;

#[derive(Clone)]
pub struct SelectionModel {
    /// Core client retrieves proto descriptors
    core_client: Rc<RefCell<CoreClient>>,
    /// a list of services.
    pub services: Vec<String>,
    /// A list of methods.
    pub methods: Vec<String>,
    /// The selection state of the grpc services.
    pub services_state: ListState,
    /// The selection state of the grpc methods.
    pub methods_state: ListState,
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
        let services = list_services(&core_client.borrow());
        let mut methods: Vec<String> = Vec::new();

        // Preselect first service
        let mut services_state = ListState::default();
        if !services.is_empty() {
            services_state.select(Some(0));
            methods = list_methods(&core_client.borrow(), &services[0]);
        }

        Self {
            core_client,
            services,
            methods,
            services_state,
            methods_state: ListState::default(),
        }
    }

    /// Select the next service.
    pub fn next_service(&mut self) {
        if self.services.is_empty() {
            return;
        }
        let i = match self.services_state.selected() {
            Some(i) => {
                if i >= self.services.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.services_state.select(Some(i));
        self.load_methods();
    }

    /// Select the previous service.
    pub fn previous_service(&mut self) {
        if self.services.is_empty() {
            return;
        }
        let i = match self.services_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.services.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.services_state.select(Some(i));
        self.load_methods();
    }

    /// Load the methods after a services was selected
    pub fn load_methods(&mut self) {
        if let Some(service_index) = self.services_state.selected() {
            let service_name = &self.services[service_index];
            self.methods = list_methods(&self.core_client.borrow(), service_name);
        }
    }

    /// Select the next method.
    pub fn next_method(&mut self) {
        if self.methods.is_empty() {
            return;
        }
        let i = match self.methods_state.selected() {
            Some(i) => {
                if i >= self.methods.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.methods_state.select(Some(i));
    }

    /// Select the previous method.
    pub fn previous_method(&mut self) {
        if self.methods.is_empty() {
            return;
        }
        let i = match self.methods_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.services.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.methods_state.select(Some(i));
    }

    /// Return the description of the currently selected service
    pub fn selected_service(&self) -> Option<ServiceDescriptor> {
        if let Some(index) = self.services_state.selected() {
            let name = &self.services[index];
            return self.core_client.borrow().get_service_by_name(name);
        }
        return None;
    }

    /// Return the descrption of the currently selected method
    pub fn selected_method(&self) -> Option<MethodDescriptor> {
        if let (Some(service_index), Some(method_index)) = (
            self.services_state.selected(),
            self.methods_state.selected(),
        ) {
            let service_name = &self.services[service_index];
            let method_name = &self.methods[method_index];
            return self
                .core_client
                .borrow()
                .get_method_by_name(service_name, method_name);
        }
        return None;
    }

    /// Clears the method state
    pub fn clear_method(&mut self) {
        self.methods_state.select(None);
    }

    pub fn selected_service_index(&self) -> Option<usize> {
        self.services_state.selected()
    }

    pub fn selected_method_index(&self) -> Option<usize> {
        self.methods_state.selected()
    }
}

fn list_services(core_client: &CoreClient) -> Vec<String> {
    core_client
        .get_services()
        .iter()
        .map(|service| service.full_name().to_string())
        .collect()
}

fn list_methods(core_client: &CoreClient, service_name: &str) -> Vec<String> {
    if let Some(service) = core_client.get_service_by_name(service_name) {
        return core_client
            .get_methods(&service)
            .iter()
            .map(|method| method.name().to_string())
            .collect();
    }
    return Vec::new();
}
