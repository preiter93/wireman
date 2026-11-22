use std::{cell::RefCell, error::Error, rc::Rc};

use ratatui::{layout::Direction, prelude::Rect};
use wireman_config::Config;
use wireman_core::ProtoDescriptor;

use crate::model::{
    configuration::ConfigurationModel, headers::HeadersModel, history::HistoryModel,
    reflection::ReflectionModel, MessagesModel, SelectionModel,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Default)]
pub struct UiState {
    pub navbar_tabs: Option<[Rect; 3]>,
    pub history_tabs: Option<[Rect; 5]>,
    pub main_split: Direction,
}

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

    /// The model for the history
    pub history: Rc<RefCell<HistoryModel>>,

    /// Common UI state (e.g., navbar tabs hit-test areas)
    pub ui: Rc<RefCell<UiState>>,

    /// The model for the configuration dialog
    pub configuration: Rc<RefCell<ConfigurationModel>>,
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
    pub fn new(env: &Config, config_file: Option<String>) -> Result<Self> {
        // The proto descriptor
        let descriptor = Rc::new(RefCell::new(ProtoDescriptor::new(
            env.includes(),
            env.files(),
        )?));

        // The history model
        let history = Rc::new(RefCell::new(HistoryModel::new(env)));

        // The metadata model
        let server_address = env.server.default_address.clone().unwrap_or_default();
        let server_auth_header = env.server.default_auth_header.clone().unwrap_or_default();
        let headers = Rc::new(RefCell::new(HeadersModel::new(
            &server_address,
            &server_auth_header,
            Rc::clone(&history),
        )));

        // The selection model
        let selection = Rc::new(RefCell::new(SelectionModel::new(
            Rc::clone(&descriptor),
            env,
        )));

        // The configuration model.
        let configuration = Rc::new(RefCell::new(ConfigurationModel::new(config_file)));

        // The reflection model
        let reflection = Rc::new(RefCell::new(ReflectionModel::new(
            Rc::clone(&headers),
            Rc::clone(&selection),
        )));

        // The messages model
        let messages = Rc::new(RefCell::new(MessagesModel::new(
            descriptor,
            env,
            Rc::clone(&headers),
            Rc::clone(&history),
        )));

        // The ui model
        let mut ui = UiState::default();
        let theme = wireman_theme::Theme::global();
        ui.main_split = theme.layout.main_split;

        Ok(Self {
            tab: Tab::default(),
            selection_tab: SelectionTab::default(),
            messages_tab: MessagesTab::default(),
            disable_root_events: false,
            help: None,
            selection,
            messages,
            headers,
            history,
            reflection,
            ui: Rc::new(RefCell::new(ui)),
            configuration,
        })
    }

    pub fn reload(&mut self, env: &Config) {
        let Ok(descriptor) = ProtoDescriptor::new(env.includes(), env.files()) else {
            return;
        };

        let selection = Rc::new(RefCell::new(SelectionModel::new(
            Rc::new(RefCell::new(descriptor)),
            env,
        )));

        self.selection = selection;
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
