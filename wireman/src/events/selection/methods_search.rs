use crate::context::{AppContext, SelectionTab};
use std::fmt;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodsSearchEvents {
    Finish,
    RemoveChar,
}

impl fmt::Display for MethodsSearchEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            MethodsSearchEvents::Finish => "Finish",
            MethodsSearchEvents::RemoveChar => "Remove Character",
        };
        write!(f, "{display_str}")
    }
}

pub struct MethodsSearchEventsHandler;

impl EventHandler for MethodsSearchEventsHandler {
    type Context = AppContext;

    type Event = MethodsSearchEvents;

    fn handle_event(event: &MethodsSearchEvents, ctx: &mut Self::Context) {
        match event {
            MethodsSearchEvents::Finish => {
                ctx.selection_tab = SelectionTab::Methods;
                ctx.disable_root_events = false;
            }
            MethodsSearchEvents::RemoveChar => {
                ctx.selection.borrow_mut().remove_char_methods_filter();
            }
        }
    }

    fn pass_through_key_events(key_event: &KeyEvent, ctx: &mut Self::Context) {
        if let KeyCode::Char(ch) = key_event.code {
            ctx.selection.borrow_mut().push_char_methods_filter(ch);
        }
    }

    fn key_event_mappings(_: &Self::Context) -> Vec<(KeyEvent, MethodsSearchEvents)> {
        Vec::from([
            (KeyEvent::new(KeyCode::Enter), MethodsSearchEvents::Finish),
            (KeyEvent::new(KeyCode::Esc), MethodsSearchEvents::Finish),
            (
                KeyEvent::new(KeyCode::Backspace),
                MethodsSearchEvents::RemoveChar,
            ),
        ])
    }
}
