use crate::context::{AppContext, SelectionTab};
use std::collections::HashMap;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServicesSearchEvents {
    Finish,
    RemoveChar,
}

pub struct ServicesSearchEventsHandler {}

impl ServicesSearchEventsHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl EventHandler for ServicesSearchEventsHandler {
    type Context = AppContext;

    type Event = ServicesSearchEvents;

    fn handle_event(event: &ServicesSearchEvents, ctx: &mut Self::Context) {
        match event {
            ServicesSearchEvents::Finish => {
                ctx.selection_tab = SelectionTab::Services;
                ctx.disable_root_events = false;
            }
            ServicesSearchEvents::RemoveChar => {
                ctx.selection.borrow_mut().remove_char_services_filter();
            }
        }
    }

    fn pass_through_key_events(key_event: &KeyEvent, ctx: &mut Self::Context) {
        if let KeyCode::Char(ch) = key_event.code {
            ctx.selection.borrow_mut().push_char_services_filter(ch);
        }
    }

    fn key_event_mappings(_: &Self::Context) -> HashMap<KeyEvent, ServicesSearchEvents> {
        HashMap::from([
            (KeyEvent::new(KeyCode::Enter), ServicesSearchEvents::Finish),
            (KeyEvent::new(KeyCode::Esc), ServicesSearchEvents::Finish),
            (
                KeyEvent::new(KeyCode::Backspace),
                ServicesSearchEvents::RemoveChar,
            ),
        ])
    }
}
