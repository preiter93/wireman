use crate::context::{AppContext, MessagesTab};
use crossterm::event::MouseEvent;
use event_handler::{EventHandler, KeyCode, KeyEvent};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseEvents {
    NextTab,
    PrevTab,
    GoToRequest,
    CopyAsGrpCurl,
    IncreaseSize,
    DecreaseSize,
}

impl fmt::Display for ResponseEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            ResponseEvents::NextTab => "Next Page",
            ResponseEvents::PrevTab => "Prev Page",
            ResponseEvents::GoToRequest => "Go to Request",
            ResponseEvents::CopyAsGrpCurl => "Copy as cURL",
            ResponseEvents::IncreaseSize => "Increase Size",
            ResponseEvents::DecreaseSize => "Decrease Size",
        };
        write!(f, "{display_str}")
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
            ResponseEvents::IncreaseSize => {
                ctx.messages.borrow_mut().request.decrease_window_size();
            }
            ResponseEvents::DecreaseSize => {
                ctx.messages.borrow_mut().request.increase_window_size();
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
                (
                    KeyEvent::new(KeyCode::Char('+')),
                    ResponseEvents::IncreaseSize,
                ),
                (
                    KeyEvent::new(KeyCode::Char('-')),
                    ResponseEvents::DecreaseSize,
                ),
            ]);
        }
        map
    }

    fn pass_through_key_events(event: &KeyEvent, ctx: &mut Self::Context) {
        let response = &mut ctx.messages.borrow_mut().response.editor;
        response.on_key(event.clone().into());
        ctx.disable_root_events = !response.normal_mode();
    }

    fn pass_through_mouse_events(event: &MouseEvent, ctx: &mut Self::Context) {
        let editor = &mut ctx.messages.borrow_mut().response.editor;
        editor.on_mouse(*event);
    }
}
