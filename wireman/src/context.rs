use std::{cell::RefCell, error::Error, rc::Rc};

use config::Config;

use crate::model::{
    headers::HeadersModel, history::HistoryModel, reflection::ReflectionModel, CoreClient,
    MessagesModel, SelectionModel,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct AppContext {
    /// The main tab.
    pub tab: Tab,

    /// The index of the selection sub window.
    pub selection_tab: SelectionTab,

    /// The index of the messages sub window.
    pub messages_tab: MessagesTab,

    /// Holds the data for the help modal dialog. Only non-None
    /// if the help modal dialog is open.
    pub help: Option<HelpContext>,

    /// Disable root key events. Disables keys such as
    /// quit when an editor is in insert mode.
    pub disable_root_events: bool,

    /// The model for the services and methods list
    pub selection: Rc<RefCell<SelectionModel>>,

    /// The model for server reflection
    pub reflection: Rc<RefCell<ReflectionModel>>,

    /// The model for the request and response messages
    pub messages: Rc<RefCell<MessagesModel>>,

    /// The model for the headers
    pub headers: Rc<RefCell<HeadersModel>>,
}

pub struct HelpContext {
    pub(crate) key_mappings: Vec<(String, String)>,
}

impl HelpContext {
    pub fn new(key_mappings: Vec<(String, String)>) -> Self {
        Self { key_mappings }
    }
}

impl AppContext {
    pub fn new(env: &Config) -> Result<Self> {
        // The core client
        let core_client_rc = Rc::new(RefCell::new(CoreClient::new(env)?));

        // The metadata model
        let server_address = &core_client_rc.borrow().get_default_address();
        let headers = Rc::new(RefCell::new(HeadersModel::new(server_address)));

        // The selection model
        let selection = Rc::new(RefCell::new(SelectionModel::new(Rc::clone(
            &core_client_rc,
        ))));

        // The reflection model
        let reflection = Rc::new(RefCell::new(ReflectionModel::new(
            Rc::clone(&core_client_rc),
            Rc::clone(&headers),
            Rc::clone(&selection),
        )));

        // The history model
        let history_model = HistoryModel::new(env)?;

        // The messages model
        let messages = Rc::new(RefCell::new(MessagesModel::new(
            core_client_rc,
            Rc::clone(&headers),
            history_model,
        )));

        Ok(Self {
            tab: Tab::default(),
            selection_tab: SelectionTab::default(),
            messages_tab: MessagesTab::default(),
            disable_root_events: false,
            help: None,
            selection,
            messages,
            headers,
            reflection,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Tab {
    #[default]
    Selection,
    Messages,
    Headers,
}
impl Tab {
    pub fn next(self) -> Self {
        match &self {
            Self::Selection => Self::Headers,
            Self::Headers => Self::Messages,
            Self::Messages => Self::Selection,
        }
    }
    pub fn prev(self) -> Self {
        match &self {
            Self::Selection => Self::Messages,
            Self::Headers => Self::Selection,
            Self::Messages => Self::Headers,
        }
    }
    pub fn index(self) -> usize {
        match &self {
            Self::Selection => 0,
            Self::Messages => 2,
            Self::Headers => 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum SelectionTab {
    #[default]
    Services,
    Methods,
    SearchServices,
    SearchMethods,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum MessagesTab {
    #[default]
    Request,
    Response,
}
