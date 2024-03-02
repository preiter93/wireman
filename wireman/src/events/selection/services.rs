use crate::context::{AppContext, SelectionTab};
use std::collections::HashMap;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServicesSelectionEvent {
    Next,
    Prev,
    Select,
    Search,
    ClearSearch,
    GoToMethods,
}

pub struct ServicesSelectionEventHandler {}

impl ServicesSelectionEventHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl EventHandler for ServicesSelectionEventHandler {
    type Context = AppContext;

    type Event = ServicesSelectionEvent;

    fn handle_event(event: &Self::Event, ctx: &mut Self::Context) {
        match event {
            ServicesSelectionEvent::Next => {
                ctx.selection.borrow_mut().next_service();
                ctx.selection.borrow_mut().clear_methods_selection();
            }
            ServicesSelectionEvent::Prev => {
                ctx.selection.borrow_mut().previous_service();
                ctx.selection.borrow_mut().clear_methods_selection();
            }
            ServicesSelectionEvent::Select => {
                ctx.selection_tab = SelectionTab::Methods;
                if ctx.selection.borrow().selected_method().is_none() {
                    ctx.selection.borrow_mut().next_method();
                }
            }
            ServicesSelectionEvent::Search => {
                ctx.selection_tab = SelectionTab::SearchServices;
                ctx.disable_root_events = true;
            }
            ServicesSelectionEvent::ClearSearch => {
                if ctx.selection.borrow().services_filter.is_some() {
                    ctx.selection.borrow_mut().clear_services_filter();
                }
            }
            ServicesSelectionEvent::GoToMethods => {
                ctx.selection_tab = SelectionTab::Methods;
            }
        }
    }

    fn key_event_mappings(_: &Self::Context) -> HashMap<KeyEvent, Self::Event> {
        HashMap::from([
            (KeyEvent::new(KeyCode::Down), Self::Event::Next),
            (KeyEvent::new(KeyCode::Char('j')), Self::Event::Next),
            (KeyEvent::new(KeyCode::Up), Self::Event::Prev),
            (KeyEvent::new(KeyCode::Char('k')), Self::Event::Prev),
            (KeyEvent::new(KeyCode::Tab), Self::Event::Select),
            (KeyEvent::new(KeyCode::Enter), Self::Event::Select),
            (KeyEvent::new(KeyCode::Char('/')), Self::Event::Search),
            (KeyEvent::new(KeyCode::Esc), Self::Event::ClearSearch),
            (KeyEvent::new(KeyCode::Char('J')), Self::Event::GoToMethods),
        ])
    }
}
