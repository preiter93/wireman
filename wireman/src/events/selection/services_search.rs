use crate::context::{AppContext, SelectionTab};
use std::collections::HashMap;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServicesSearchEvent {
    Finish,
    RemoveChar,
}

pub struct ServicesSearchEventHandler {}

impl ServicesSearchEventHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl EventHandler for ServicesSearchEventHandler {
    type Context = AppContext;

    type Event = ServicesSearchEvent;

    fn handle_event(event: &Self::Event, ctx: &mut Self::Context) {
        match event {
            ServicesSearchEvent::Finish => {
                ctx.selection_tab = SelectionTab::Services;
                ctx.disable_root_events = false;
            }
            ServicesSearchEvent::RemoveChar => {
                ctx.selection.borrow_mut().remove_char_services_filter();
            }
        }
    }

    fn pass_through_key_events(key_event: &KeyEvent, ctx: &mut Self::Context) {
        if let KeyCode::Char(ch) = key_event.code {
            ctx.selection.borrow_mut().push_char_services_filter(ch);
        }
    }

    fn key_event_mappings(_: &Self::Context) -> HashMap<KeyEvent, Self::Event> {
        HashMap::from([
            (KeyEvent::new(KeyCode::Enter), Self::Event::Finish),
            (KeyEvent::new(KeyCode::Esc), Self::Event::Finish),
            (KeyEvent::new(KeyCode::Backspace), Self::Event::RemoveChar),
        ])
    }
}
