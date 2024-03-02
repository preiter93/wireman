use crate::{context::AppContext, model::headers::HeadersTab};
use std::collections::HashMap;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent, KeyModifier};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadersEvents {
    Confirm,
    NextTab,
    PrevTab,
    PrevRow,
    NextRow,
    NextCol,
    PrevCol,
    NextColForce,
    PrevColForce,
    Unselect,
    AddHeaders,
    DelHeaders,
}

impl HeadersEventHandler {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct HeadersEventHandler;

impl EventHandler for HeadersEventHandler {
    type Context = AppContext;

    type Event = HeadersEvents;

    fn handle_event(event: &Self::Event, ctx: &mut Self::Context) {
        match event {
            HeadersEvents::Confirm => {
                if ctx.headers.borrow().tab == HeadersTab::None {
                    ctx.tab = ctx.tab.next();
                } else {
                    ctx.headers.borrow_mut().tab = HeadersTab::None;
                }
            }
            HeadersEvents::NextTab => {
                ctx.tab = ctx.tab.next();
            }
            HeadersEvents::PrevTab => {
                ctx.tab = ctx.tab.prev();
            }
            HeadersEvents::NextRow => {
                ctx.headers.borrow_mut().next_row();
            }
            HeadersEvents::PrevRow => {
                ctx.headers.borrow_mut().prev_row();
            }
            HeadersEvents::NextCol => {
                ctx.headers.borrow_mut().next_col();
            }
            HeadersEvents::PrevCol => {
                ctx.headers.borrow_mut().prev_col();
            }
            HeadersEvents::NextColForce => {
                ctx.headers.borrow_mut().next_col();
            }
            HeadersEvents::PrevColForce => {
                ctx.headers.borrow_mut().prev_col();
            }
            HeadersEvents::Unselect => {
                ctx.headers.borrow_mut().tab = HeadersTab::None;
            }
            HeadersEvents::AddHeaders => {
                ctx.headers.borrow_mut().meta.add();
            }
            HeadersEvents::DelHeaders => {
                let selected = ctx.headers.borrow().meta.selected;
                if let Some(index) = selected {
                    ctx.headers.borrow_mut().meta.remove(index.row);
                    if ctx.headers.borrow().meta.is_empty() {
                        ctx.headers.borrow_mut().tab = HeadersTab::None;
                    }
                }
            }
        }
    }

    fn key_event_mappings(ctx: &Self::Context) -> HashMap<KeyEvent, Self::Event> {
        let disabled_root_events = ctx.headers.borrow().disabled_root_events();
        let (is_first_col, is_last_col) = match ctx.headers.borrow().selected_editor() {
            Some(e) => (e.is_first_col(), e.is_last_col()),
            None => (true, true),
        };
        let is_meta_tab = ctx.headers.borrow().tab == HeadersTab::Meta;
        let mut map = HashMap::new();
        if !disabled_root_events {
            map.extend([
                (KeyEvent::new(KeyCode::Esc), Self::Event::Unselect),
                (KeyEvent::new(KeyCode::Enter), Self::Event::Confirm),
                (KeyEvent::new(KeyCode::Tab), Self::Event::NextTab),
                (KeyEvent::new(KeyCode::BackTab), Self::Event::PrevTab),
                (KeyEvent::new(KeyCode::Down), Self::Event::NextRow),
                (KeyEvent::new(KeyCode::Char('j')), Self::Event::NextRow),
                (KeyEvent::new(KeyCode::Up), Self::Event::PrevRow),
                (KeyEvent::new(KeyCode::Char('k')), Self::Event::PrevRow),
                (KeyEvent::new(KeyCode::Char('L')), Self::Event::NextColForce),
                (KeyEvent::new(KeyCode::Char('H')), Self::Event::PrevColForce),
                (
                    KeyEvent::new(KeyCode::Char('a')).modifier(KeyModifier::Control),
                    Self::Event::AddHeaders,
                ),
            ]);
        }
        if !disabled_root_events && is_first_col {
            map.extend([
                (KeyEvent::new(KeyCode::Left), Self::Event::PrevCol),
                (KeyEvent::new(KeyCode::Char('h')), Self::Event::PrevCol),
            ]);
        }
        if !disabled_root_events && is_last_col {
            map.extend([
                (KeyEvent::new(KeyCode::Char('l')), Self::Event::NextCol),
                (KeyEvent::new(KeyCode::Right), Self::Event::NextCol),
            ]);
        }
        if is_meta_tab {
            map.extend([(
                KeyEvent::new(KeyCode::Char('d')).modifier(KeyModifier::Control),
                Self::Event::DelHeaders,
            )]);
        }
        map
    }

    fn pass_through_key_events(key_event: &KeyEvent, ctx: &mut Self::Context) {
        let tab = ctx.headers.borrow().tab.clone();
        match tab {
            HeadersTab::Meta => {
                if let Some(input) = ctx.headers.borrow_mut().selected_editor_mut() {
                    input.on_key(key_event.clone().into());
                }
            }
            HeadersTab::Addr => {
                ctx.headers
                    .borrow_mut()
                    .addr
                    .on_key(key_event.clone().into());
            }
            HeadersTab::Auth => {
                ctx.headers
                    .borrow_mut()
                    .auth
                    .selected_editor_mut()
                    .on_key(key_event.clone().into());
            }
            _ => (),
        }
    }
}
