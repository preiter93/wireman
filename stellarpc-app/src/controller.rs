#![allow(clippy::module_name_repetitions)]
use crate::{
    model::{
        headers::HeadersModel, history::HistoryModel, CoreClient, MessagesModel, SelectionModel,
    },
    AppConfig,
};
use std::{cell::RefCell, rc::Rc};

/// Translates key input to actions for the models
pub struct Controller {
    /// The model for the services and methods list
    pub selection: Rc<RefCell<SelectionModel>>,

    /// The model for the request and response messages
    pub messages: Rc<RefCell<MessagesModel>>,

    /// The model for the headers
    pub headers: Rc<RefCell<HeadersModel>>,

    ///  Whether to display the help
    pub show_help: bool,
}

impl Controller {
    /// Instantiate the homepage
    pub fn new(core_client: CoreClient, config: AppConfig) -> Self {
        // The core client is shared
        let core_client_rc = Rc::new(RefCell::new(core_client));

        // The selection model
        let selection = Rc::new(RefCell::new(SelectionModel::new(Rc::clone(
            &core_client_rc,
        ))));

        // The metadata model
        let server_address = &core_client_rc.borrow().get_default_address();
        let headers = Rc::new(RefCell::new(HeadersModel::new(server_address)));

        // The messages model
        let messages = Rc::new(RefCell::new(MessagesModel::new(
            core_client_rc,
            Rc::clone(&headers),
            HistoryModel::new(config.history),
        )));

        Self {
            selection,
            messages,
            headers,
            show_help: true,
        }
    }
}
