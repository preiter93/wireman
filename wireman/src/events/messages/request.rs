use crate::{
    context::{AppContext, MessagesTab},
    model::headers::HeadersTab,
};
use std::collections::HashMap;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent, KeyModifier};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestEvents {
    MakeRequest,
    AbortRequest,
    NextTab,
    PrevTab,
    GoToResponse,
    CopyAsGrpCurl,
    FormatMessage,
    ResetMessage,
    SaveMessage,
    LoadHistory1,
    LoadHistory2,
    LoadHistory3,
    LoadHistory4,
    LoadHistory5,
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

    fn handle_event(event: &Self::Event, ctx: &mut Self::Context) {
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
                if ctx.headers.borrow().tab == HeadersTab::None {
                    ctx.tab = ctx.tab.next();
                } else {
                    ctx.headers.borrow_mut().tab = HeadersTab::None;
                }
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
            RequestEvents::SaveMessage => {
                let history = &ctx.messages.borrow().history_model;
                history.save(&ctx.messages.borrow());
            }
            RequestEvents::ResetMessage => {
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

    fn key_event_mappings(ctx: &Self::Context) -> HashMap<KeyEvent, Self::Event> {
        let disabled_root_events = ctx.disable_root_events;
        let mut map = HashMap::new();
        if !disabled_root_events {
            map.extend([
                (KeyEvent::new(KeyCode::Tab), Self::Event::NextTab),
                (KeyEvent::new(KeyCode::BackTab), Self::Event::PrevTab),
                (KeyEvent::new(KeyCode::Enter), Self::Event::MakeRequest),
                (KeyEvent::new(KeyCode::Esc), Self::Event::AbortRequest),
                (KeyEvent::new(KeyCode::Char('J')), Self::Event::GoToResponse),
                (
                    KeyEvent::new(KeyCode::Char('y')).modifier(KeyModifier::Control),
                    Self::Event::CopyAsGrpCurl,
                ),
                (
                    KeyEvent::new(KeyCode::Char('f')).modifier(KeyModifier::Control),
                    Self::Event::FormatMessage,
                ),
                (
                    KeyEvent::new(KeyCode::Char('s')).modifier(KeyModifier::Control),
                    Self::Event::SaveMessage,
                ),
                (
                    KeyEvent::new(KeyCode::Char('d')).modifier(KeyModifier::Control),
                    Self::Event::ResetMessage,
                ),
                (KeyEvent::new(KeyCode::Char('1')), Self::Event::LoadHistory1),
                (KeyEvent::new(KeyCode::Char('2')), Self::Event::LoadHistory2),
                (KeyEvent::new(KeyCode::Char('3')), Self::Event::LoadHistory3),
                (KeyEvent::new(KeyCode::Char('4')), Self::Event::LoadHistory4),
                (KeyEvent::new(KeyCode::Char('5')), Self::Event::LoadHistory5),
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
