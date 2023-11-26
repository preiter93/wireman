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
    /// The state of the services and methods list. Holds info about currently
    /// selected service and method and whether a service should be expanded.
    pub selection: SelectionState,
    /// A list of proto services. Each service can hold a list of methods.
    pub items: Vec<ServiceWithMethods>,
    /// The selection state of the grpc services.
    pub svc_state: ListState,
    /// The selection state of the grpc methods.
    pub mth_state: ListState,
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
        let mut state = SelectionState::default();
        if !services.is_empty() {
            state.select_parent(Some(0));
        }

        Self {
            core_client,
            selection: state,
            items,
            svc_state: ListState::default(),
            mth_state: ListState::default(),
        }
    }

    /// Whether the parent level is selected
    pub fn is_parent_selected(&self) -> bool {
        self.selection.selection_level() == SelectionLevel::Parent
    }

    /// Select the next service or method, depending on the current
    /// selection level.
    pub fn next(&mut self) {
        match self.selection.selection_level() {
            SelectionLevel::Parent => self.next_service(),
            SelectionLevel::Children => self.next_method(),
        }
    }

    /// Select the previous service or method, depending on the current
    /// selection level.
    pub fn previous(&mut self) {
        match self.selection.selection_level() {
            SelectionLevel::Parent => self.previous_service(),
            SelectionLevel::Children => self.previous_method(),
        }
    }

    /// Select the next service.
    pub fn next_service(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.selection.selected_parent() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selection.select_parent(Some(i));
    }

    /// Select the previous service.
    pub fn previous_service(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.selection.selected_parent() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selection.select_parent(Some(i));
    }

    /// Select the next method.
    pub fn next_method(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let k = match (
            self.selection.selected_parent(),
            self.selection.selected_child(),
        ) {
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
        self.selection.select_child(Some(k));
    }

    /// Select the previous method.
    pub fn previous_method(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let k = match (
            self.selection.selected_parent(),
            self.selection.selected_child(),
        ) {
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
        self.selection.select_child(Some(k));
    }

    /// Return the description of the currently selected service
    pub fn selected_service(&self) -> Option<ServiceDescriptor> {
        self.selection
            .selected_parent()
            .map(|i| self.core_client.borrow().get_services()[i].clone())
    }

    /// Return the descrption of the currently selected method
    pub fn selected_method(&self) -> Option<MethodDescriptor> {
        if let (Some(service), Some(i)) = (self.selected_service(), self.selection.selected_child())
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

    /// Clears the method state
    pub fn clear_method(&mut self) {
        self.selection.select_child(None);
    }

    pub fn selected_service_index(&self) -> Option<usize> {
        self.selection.selected_parent
    }

    pub fn selected_method_index(&self) -> Option<usize> {
        self.selection.selected_child
    }
}

/// The local state for the services and methods
/// Holds the index of selected service and method. And
/// whether the child list of a service should be
/// expanded.
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    offset: usize,
    selected_parent: Option<usize>,
    selected_child: Option<usize>,
    selection_level: SelectionLevel,
}

/// Whether we are currently selecting in the parent
/// list or in the children list.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum SelectionLevel {
    #[default]
    Parent,
    Children,
}

impl SelectionState {
    pub fn selected_parent(&self) -> Option<usize> {
        self.selected_parent
    }

    pub fn selected_child(&self) -> Option<usize> {
        self.selected_child
    }

    pub fn select_parent(&mut self, index: Option<usize>) {
        self.selected_parent = index;
        if index.is_none() {
            self.offset = 0;
        }
    }

    pub fn select_child(&mut self, index: Option<usize>) {
        self.selected_child = index;
    }

    pub fn selection_level(&self) -> SelectionLevel {
        self.selection_level.clone()
    }
}
