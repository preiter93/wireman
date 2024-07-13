use crate::context::{AppContext, MessagesTab};
use std::fmt;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseEvents {
    NextTab,
    PrevTab,
    GoToRequest,
    CopyAsGrpCurl,
}

impl fmt::Display for ResponseEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            ResponseEvents::NextTab => "Next Tab",
            ResponseEvents::PrevTab => "Prev Tab",
            ResponseEvents::GoToRequest => "Go to Request",
            ResponseEvents::CopyAsGrpCurl => "Copy as cURL",
        };
        write!(f, "{}", display_str)
    }
}

impl ResponseEventHandler {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct ResponseEventHandler;

impl EventHandler for ResponseEventHandler {
    type Context = AppContext;

    type Event = ResponseEvents;

    fn handle_event(event: &ResponseEvents, ctx: &mut Self::Context) {
        match event {
            ResponseEvents::NextTab => {
                ctx.tab = ctx.tab.next();
            }
            ResponseEvents::PrevTab => {
                ctx.tab = ctx.tab.prev();
            }
            ResponseEvents::GoToRequest => {
                ctx.messages_tab = MessagesTab::Request;
            }
            ResponseEvents::CopyAsGrpCurl => {
                ctx.messages.borrow_mut().yank_grpcurl();
            }
        }
    }

    fn key_event_mappings(ctx: &Self::Context) -> Vec<(KeyEvent, ResponseEvents)> {
        let disabled_root_events = ctx.disable_root_events;
        let mut map = Vec::new();
        if !disabled_root_events {
            map.extend([
                (KeyEvent::new(KeyCode::Tab), ResponseEvents::NextTab),
                (KeyEvent::shift(KeyCode::BackTab), ResponseEvents::PrevTab),
                (
                    KeyEvent::shift(KeyCode::Char('K')),
                    ResponseEvents::GoToRequest,
                ),
                (
                    KeyEvent::ctrl(KeyCode::Char('y')),
                    ResponseEvents::CopyAsGrpCurl,
                ),
            ]);
        }
        map
    }

    fn pass_through_key_events(key_event: &KeyEvent, ctx: &mut Self::Context) {
        // read only
        if key_event.code == KeyCode::Char('i') {
            return;
        }
        let response = &mut ctx.messages.borrow_mut().response.editor;
        response.on_key(key_event.clone().into());
        ctx.disable_root_events = !response.normal_mode();
    }
}
