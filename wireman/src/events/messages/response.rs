use crate::context::{AppContext, MessagesTab};
use std::collections::HashMap;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent, KeyModifier};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseEvents {
    NextTab,
    PrevTab,
    GoToRequest,
    CopyAsGrpCurl,
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

    fn handle_event(event: &Self::Event, ctx: &mut Self::Context) {
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

    fn key_event_mappings(ctx: &Self::Context) -> HashMap<KeyEvent, Self::Event> {
        let disabled_root_events = ctx.disable_root_events;
        let mut map = HashMap::new();
        if !disabled_root_events {
            map.extend([
                (KeyEvent::new(KeyCode::Tab), Self::Event::NextTab),
                (KeyEvent::new(KeyCode::BackTab), Self::Event::PrevTab),
                (KeyEvent::new(KeyCode::Char('K')), Self::Event::GoToRequest),
                (
                    KeyEvent::new(KeyCode::Char('y')).modifier(KeyModifier::Control),
                    Self::Event::CopyAsGrpCurl,
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
