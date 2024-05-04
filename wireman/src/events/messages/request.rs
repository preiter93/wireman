use crate::context::{AppContext, MessagesTab};
use std::fmt;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestEvents {
    NextTab,
    PrevTab,
    MakeRequest,
    AbortRequest,
    GoToResponse,
    CopyAsGrpCurl,
    FormatMessage,
    ResetHistory,
    SaveHistory,
    LoadHistory1,
    LoadHistory2,
    LoadHistory3,
    LoadHistory4,
    LoadHistory5,
}

impl fmt::Display for RequestEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            RequestEvents::NextTab => "Next Tab",
            RequestEvents::PrevTab => "Previous Tab",
            RequestEvents::MakeRequest => "Make Request",
            RequestEvents::AbortRequest => "Abort Request",
            RequestEvents::GoToResponse => "Go to Response",
            RequestEvents::CopyAsGrpCurl => "Copy as cURL",
            RequestEvents::FormatMessage => "Format Message",
            RequestEvents::ResetHistory => "Reset Request",
            RequestEvents::SaveHistory => "Save Request",
            RequestEvents::LoadHistory1 => "Load History 1",
            RequestEvents::LoadHistory2 => "Load History 2",
            RequestEvents::LoadHistory3 => "Load History 3",
            RequestEvents::LoadHistory4 => "Load History 4",
            RequestEvents::LoadHistory5 => "Load History 5",
        };
        write!(f, "{}", display_str)
    }
}

impl RequestEventHandler {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct RequestEventHandler;

impl EventHandler for RequestEventHandler {
    type Context = AppContext;

    type Event = RequestEvents;

    fn handle_event(event: &RequestEvents, ctx: &mut Self::Context) {
        match event {
            RequestEvents::NextTab => {
                ctx.tab = ctx.tab.next();
            }
            RequestEvents::PrevTab => {
                ctx.tab = ctx.tab.prev();
            }
            RequestEvents::MakeRequest => {
                {
                    let history = &ctx.messages.borrow().history_model;
                    if history.autosave {
                        history.save(&ctx.messages.borrow());
                    }
                }
                ctx.messages.borrow_mut().start_request();
            }
            RequestEvents::AbortRequest => {
                ctx.messages.borrow_mut().abort_request();
            }
            RequestEvents::GoToResponse => {
                ctx.messages_tab = MessagesTab::Response;
            }
            RequestEvents::CopyAsGrpCurl => {
                ctx.messages.borrow_mut().yank_grpcurl();
            }
            RequestEvents::FormatMessage => {
                ctx.messages.borrow_mut().request.editor.format_json();
            }
            RequestEvents::SaveHistory => {
                let history = &ctx.messages.borrow().history_model;
                history.save(&ctx.messages.borrow());
            }
            RequestEvents::ResetHistory => {
                let method = ctx.messages.borrow().selected_method.clone();
                if let Some(method) = method {
                    ctx.messages.borrow().history_model.delete(&method);
                    ctx.messages.borrow_mut().request.load_template(&method);
                    ctx.messages.borrow_mut().headers_model.borrow_mut().clear();
                }
            }
            RequestEvents::LoadHistory1 => {
                ctx.messages.borrow_mut().handle_history_reload(1);
            }
            RequestEvents::LoadHistory2 => {
                ctx.messages.borrow_mut().handle_history_reload(2);
            }
            RequestEvents::LoadHistory3 => {
                ctx.messages.borrow_mut().handle_history_reload(3);
            }
            RequestEvents::LoadHistory4 => {
                ctx.messages.borrow_mut().handle_history_reload(4);
            }
            RequestEvents::LoadHistory5 => {
                ctx.messages.borrow_mut().handle_history_reload(5);
            }
        }
    }

    fn key_event_mappings(ctx: &Self::Context) -> Vec<(KeyEvent, RequestEvents)> {
        let disabled_root_events = ctx.disable_root_events;
        let mut map = Vec::new();
        if !disabled_root_events {
            map.extend([
                (KeyEvent::new(KeyCode::Tab), RequestEvents::NextTab),
                (KeyEvent::shift(KeyCode::BackTab), RequestEvents::PrevTab),
                (KeyEvent::new(KeyCode::Enter), RequestEvents::MakeRequest),
                (KeyEvent::new(KeyCode::Esc), RequestEvents::AbortRequest),
                (
                    KeyEvent::shift(KeyCode::Char('J')),
                    RequestEvents::GoToResponse,
                ),
                (
                    KeyEvent::ctrl(KeyCode::Char('y')),
                    RequestEvents::CopyAsGrpCurl,
                ),
                (
                    KeyEvent::ctrl(KeyCode::Char('f')),
                    RequestEvents::FormatMessage,
                ),
                (
                    KeyEvent::ctrl(KeyCode::Char('s')),
                    RequestEvents::SaveHistory,
                ),
                (
                    KeyEvent::ctrl(KeyCode::Char('d')),
                    RequestEvents::ResetHistory,
                ),
                (
                    KeyEvent::new(KeyCode::Char('1')),
                    RequestEvents::LoadHistory1,
                ),
                (
                    KeyEvent::new(KeyCode::Char('2')),
                    RequestEvents::LoadHistory2,
                ),
                (
                    KeyEvent::new(KeyCode::Char('3')),
                    RequestEvents::LoadHistory3,
                ),
                (
                    KeyEvent::new(KeyCode::Char('4')),
                    RequestEvents::LoadHistory4,
                ),
                (
                    KeyEvent::new(KeyCode::Char('5')),
                    RequestEvents::LoadHistory5,
                ),
            ]);
        }
        map
    }

    fn pass_through_key_events(key_event: &KeyEvent, ctx: &mut Self::Context) {
        let request = &mut ctx.messages.borrow_mut().request.editor;
        request.on_key(key_event.clone().into());
        ctx.disable_root_events = !request.normal_mode();
    }
}
