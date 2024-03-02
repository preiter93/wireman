use crate::context::{AppContext, SelectionTab};
use std::collections::HashMap;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodsSearchEvent {
    Finish,
    RemoveChar,
}

pub struct MethodsSearchEventHandler {}

impl MethodsSearchEventHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl EventHandler for MethodsSearchEventHandler {
    type Context = AppContext;

    type Event = MethodsSearchEvent;

    fn handle_event(event: &Self::Event, ctx: &mut Self::Context) {
        match event {
            MethodsSearchEvent::Finish => {
                ctx.selection_tab = SelectionTab::Methods;
                ctx.disable_root_events = false;
            }
            MethodsSearchEvent::RemoveChar => {
                ctx.selection.borrow_mut().remove_char_methods_filter();
            }
        }
    }

    fn pass_through_key_events(key_event: &KeyEvent, ctx: &mut Self::Context) {
        if let KeyCode::Char(ch) = key_event.code {
            ctx.selection.borrow_mut().push_char_methods_filter(ch);
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
