#![allow(clippy::module_name_repetitions)]
use crate::{
    model::{
        headers::HeadersModel, history::HistoryModel, CoreClient, MessagesModel, SelectionModel,
    },
    AppConfig,
};
use config::Config;
use std::{
    cell::RefCell,
    error::Error,
    rc::Rc,
    sync::mpsc::{self, Receiver},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

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

    /// An event receiver for app internal events.
    pub event_recv: Receiver<String>,
}

impl Controller {
    /// Instantiate the homepage
    pub fn new(env: &Config) -> Result<Self> {
        let core_client = CoreClient::new(env)?;
        let app_config = AppConfig::new(env)?;

        // The core client
        let core_client_rc = Rc::new(RefCell::new(core_client));

        // The selection model
        let selection = Rc::new(RefCell::new(SelectionModel::new(Rc::clone(
            &core_client_rc,
        ))));

        // The metadata model
        let server_address = &core_client_rc.borrow().get_default_address();
        let headers = Rc::new(RefCell::new(HeadersModel::new(server_address)));

        // An event hanlder for sending and receiving app events
        let (tx, rx) = mpsc::channel::<String>();

        // The messages model
        let messages = Rc::new(RefCell::new(MessagesModel::new(
            core_client_rc,
            Rc::clone(&headers),
            HistoryModel::new(app_config.history),
            tx,
        )));

        Ok(Self {
            selection,
            messages,
            headers,
            show_help: true,
            event_recv: rx,
        })
    }
}
