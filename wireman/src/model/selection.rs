#![allow(clippy::module_name_repetitions)]
use core::{MethodDescriptor, ProtoDescriptor, ServiceDescriptor};
use std::cell::RefCell;
use std::rc::Rc;
use tui_widget_list::ListState;

use super::core_client::CoreClient;

#[derive(Clone)]
pub struct SelectionModel {
    /// Core client retrieves proto descriptors
    core_client: Rc<RefCell<CoreClient>>,
    /// a list of services.
    services: Vec<String>,
    /// A list of methods.
    methods: Vec<String>,
    /// The selection state of the grpc services.
    pub services_state: ListState,
    /// The selection state of the grpc methods.
    pub methods_state: ListState,
    /// Filters the services
    pub services_filter: Option<String>,
    /// Filters the methods
    pub methods_filter: Option<String>,
    /// The selection mode
    pub selection_mode: SelectionMode,
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SelectionMode {
    #[default]
    File,
    ReflectionDialog,
    Reflection,
}

/// Each service can hold a list of methods
#[derive(Clone)]
pub struct ServiceWithMethods {
    pub service: String,
    pub methods: Vec<String>,
}

impl SelectionModel {
    /// Instantiates a [`SelectionModel`]. Requires the core client to
    /// retrieve the proto services and methods.
    pub fn new(core_client: Rc<RefCell<CoreClient>>) -> Self {
        let mut model = Self {
            core_client,
            services: Vec::new(),
            methods: Vec::new(),
            services_state: ListState::default(),
            methods_state: ListState::default(),
            services_filter: None,
            methods_filter: None,
            selection_mode: SelectionMode::File,
        };
        model.load_core_services_and_methods_from_files();

        model
    }

    /// Loads the services and methods from the core client
    pub fn load_core_services_and_methods_from_files(&mut self) {
        self.services = list_services(&self.core_client.borrow());
        self.services_state.select(None);

        self.methods = Vec::new();
        self.methods_state.select(None);

        if !self.services.is_empty() {
            self.services_state.select(Some(0));
            self.methods = list_methods(&self.core_client.borrow(), &self.services[0]);
        }
    }

    /// Toggles the reflection mode.
    pub fn toggle_reflection_mode(&mut self) {
        match self.selection_mode {
            SelectionMode::File => self.selection_mode = SelectionMode::ReflectionDialog,
            SelectionMode::ReflectionDialog => self.selection_mode = SelectionMode::File,
            SelectionMode::Reflection => {
                let _ = self.core_client.borrow_mut().reset();
                self.load_core_services_and_methods_from_files();
                self.selection_mode = SelectionMode::File
            }
        }
    }

    /// Update the proto descriptor.
    pub fn update_proto_descriptor(&mut self, desc: ProtoDescriptor) {
        self.core_client.borrow_mut().update_proto_descriptor(desc);
        self.load_core_services_and_methods_from_files();
    }

    /// Select the next service.
    pub fn next_service(&mut self) {
        if self.services().is_empty() {
            return;
        }
        let i = match self.services_state.selected {
            Some(i) => {
                if i >= self.services().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.services_state.select(Some(i));
        self.load_methods();
        self.methods_filter = None;
    }

    /// Select the previous service.
    pub fn previous_service(&mut self) {
        if self.services().is_empty() {
            return;
        }
        let i = match self.services_state.selected {
            Some(i) => {
                if i == 0 {
                    self.services().len() - 1
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
        if let Some(service_index) = self.services_state.selected {
            let service_name = &self.services()[service_index];
            self.methods = list_methods(&self.core_client.borrow(), service_name);
        }
    }

    /// Select the next method.
    pub fn next_method(&mut self) {
        if self.methods().is_empty() {
            return;
        }
        let i = match self.methods_state.selected {
            Some(i) => {
                if i >= self.methods().len() - 1 {
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
        if self.methods().is_empty() {
            return;
        }
        let i = match self.methods_state.selected {
            Some(i) => {
                if i == 0 {
                    self.methods().len() - 1
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
        if let Some(index) = self.services_state.selected {
            let name = &self.services[index];
            return self.core_client.borrow().get_service_by_name(name);
        }
        None
    }

    /// Return the descrption of the currently selected method
    pub fn selected_method(&self) -> Option<MethodDescriptor> {
        if let (Some(service_index), Some(method_index)) =
            (self.services_state.selected, self.methods_state.selected)
        {
            let service_name = &self.services()[service_index];
            let method_name = &self.methods()[method_index];
            return self
                .core_client
                .borrow()
                .get_method_by_name(service_name, method_name);
        }
        None
    }

    /// Clears the method state
    pub fn clear_methods_selection(&mut self) {
        self.methods_state.select(None);
    }

    pub fn services(&self) -> Vec<String> {
        let services = self.services.clone();
        if let Some(filter) = &self.services_filter {
            return services
                .into_iter()
                .filter(|service| service.to_lowercase().contains(&filter.to_lowercase()))
                .collect();
        }
        services
    }

    pub fn methods(&self) -> Vec<String> {
        let methods = self.methods.clone();
        if let Some(filter) = &self.methods_filter {
            return methods
                .into_iter()
                .filter(|method| method.to_lowercase().contains(&filter.to_lowercase()))
                .collect();
        }
        methods
    }

    fn set_services_filter(&mut self, filter: Option<String>) {
        self.services_filter = filter;
        if self.services().is_empty() {
            self.services_state.select(None);
        } else {
            self.services_state.select(Some(0));
        }
        self.load_methods();
        self.methods_state.select(None);
    }

    fn set_methods_filter(&mut self, filter: Option<String>) {
        self.methods_filter = filter;
        if self.methods().is_empty() {
            self.methods_state.select(None);
        } else {
            self.methods_state.select(Some(0));
        }
    }

    pub fn clear_services_filter(&mut self) {
        self.set_services_filter(None);
    }

    pub fn clear_methods_filter(&mut self) {
        self.set_methods_filter(None);
    }

    pub fn push_char_services_filter(&mut self, ch: char) {
        let new_filter = self
            .services_filter
            .take()
            .map_or(Some(String::from(ch)), |prev| {
                Some(prev + &String::from(ch))
            });
        self.set_services_filter(new_filter);
    }

    pub fn push_char_methods_filter(&mut self, ch: char) {
        let new_filter = self
            .methods_filter
            .take()
            .map_or(Some(String::from(ch)), |prev| {
                Some(prev + &String::from(ch))
            });
        self.set_methods_filter(new_filter);
    }

    pub fn remove_char_services_filter(&mut self) {
        if let Some(mut filter) = self.services_filter.take() {
            let _ = filter.pop();
            self.set_services_filter(Some(filter));
        }
    }

    pub fn remove_char_methods_filter(&mut self) {
        if let Some(mut filter) = self.methods_filter.take() {
            let _ = filter.pop();
            self.set_methods_filter(Some(filter));
        }
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
    Vec::new()
}
