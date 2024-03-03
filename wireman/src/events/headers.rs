use crate::{context::AppContext, model::headers::HeadersTab};
use std::fmt;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadersEvents {
    Confirm,
    NextTab,
    PrevTab,
    NextRow,
    PrevRow,
    NextCol,
    PrevCol,
    NextColForce,
    PrevColForce,
    Unselect,
    AddHeaders,
    DelHeaders,
}

impl fmt::Display for HeadersEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            HeadersEvents::Confirm => "Confirm",
            HeadersEvents::NextTab => "Next Tab",
            HeadersEvents::PrevTab => "Previous Tab",
            HeadersEvents::NextRow => "Next Row",
            HeadersEvents::PrevRow => "Previous Row",
            HeadersEvents::NextCol => "Next Column",
            HeadersEvents::PrevCol => "Previous Column",
            HeadersEvents::NextColForce => "Next Column (Force)",
            HeadersEvents::PrevColForce => "Previous Column (Force)",
            HeadersEvents::Unselect => "Unselect",
            HeadersEvents::AddHeaders => "Add Headers",
            HeadersEvents::DelHeaders => "Delete Headers",
        };
        write!(f, "{}", display_str)
    }
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

    fn handle_event(event: &HeadersEvents, ctx: &mut Self::Context) {
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

    fn key_event_mappings(ctx: &Self::Context) -> Vec<(KeyEvent, HeadersEvents)> {
        let disabled_root_events = ctx.headers.borrow().disabled_root_events();
        let (is_first_col, is_last_col) = match ctx.headers.borrow().selected_editor() {
            Some(e) => (e.is_first_col(), e.is_last_col()),
            None => (true, true),
        };
        let is_meta_tab = ctx.headers.borrow().tab == HeadersTab::Meta;
        let mut map = Vec::new();
        if !disabled_root_events {
            map.extend([
                (KeyEvent::new(KeyCode::Esc), HeadersEvents::Unselect),
                (KeyEvent::new(KeyCode::Enter), HeadersEvents::Confirm),
                (KeyEvent::new(KeyCode::Tab), HeadersEvents::NextTab),
                (KeyEvent::new(KeyCode::BackTab), HeadersEvents::PrevTab),
                (KeyEvent::new(KeyCode::Down), HeadersEvents::NextRow),
                (KeyEvent::new(KeyCode::Char('j')), HeadersEvents::NextRow),
                (KeyEvent::new(KeyCode::Up), HeadersEvents::PrevRow),
                (KeyEvent::new(KeyCode::Char('k')), HeadersEvents::PrevRow),
                (
                    KeyEvent::shift(KeyCode::Char('L')),
                    HeadersEvents::NextColForce,
                ),
                (
                    KeyEvent::shift(KeyCode::Char('H')),
                    HeadersEvents::PrevColForce,
                ),
                (
                    KeyEvent::ctrl(KeyCode::Char('a')),
                    HeadersEvents::AddHeaders,
                ),
            ]);
        }
        if !disabled_root_events && is_first_col {
            map.extend([
                (KeyEvent::new(KeyCode::Left), HeadersEvents::PrevCol),
                (KeyEvent::new(KeyCode::Char('h')), HeadersEvents::PrevCol),
            ]);
        }
        if !disabled_root_events && is_last_col {
            map.extend([
                (KeyEvent::new(KeyCode::Char('l')), HeadersEvents::NextCol),
                (KeyEvent::new(KeyCode::Right), HeadersEvents::NextCol),
            ]);
        }
        if is_meta_tab {
            map.extend([(
                KeyEvent::ctrl(KeyCode::Char('d')),
                HeadersEvents::DelHeaders,
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
