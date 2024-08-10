use crate::{context::AppContext, model::headers::HeadersTab, widgets::editor::TextEditor};
use std::fmt;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadersEvents {
    NextTab,
    PrevTab,
    NextRow,
    PrevRow,
    NextCol,
    PrevCol,
    NextColForce,
    PrevColForce,
    NextAuth,
    PrevAuth,
    Unselect,
    AddHeaders,
    DelHeaders,
    SaveHistory,
    LoadHistory1,
    LoadHistory2,
    LoadHistory3,
    LoadHistory4,
    LoadHistory5,
}

impl fmt::Display for HeadersEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            HeadersEvents::NextTab => "Next Tab",
            HeadersEvents::PrevTab => "Prev Tab",
            HeadersEvents::NextRow => "Next Row",
            HeadersEvents::PrevRow => "Prev Row",
            HeadersEvents::NextCol => "Next Column",
            HeadersEvents::PrevCol => "Prev Column",
            HeadersEvents::NextAuth => "Next auth mode",
            HeadersEvents::PrevAuth => "Prev auth mode",
            HeadersEvents::NextColForce => "Next Column (Force)",
            HeadersEvents::PrevColForce => "Prev Column (Force)",
            HeadersEvents::Unselect => "Unselect",
            HeadersEvents::AddHeaders => "Add Headers",
            HeadersEvents::DelHeaders => "Del Headers",
            HeadersEvents::SaveHistory => "Save Request",
            HeadersEvents::LoadHistory1 => "Load History 1",
            HeadersEvents::LoadHistory2 => "Load History 2",
            HeadersEvents::LoadHistory3 => "Load History 3",
            HeadersEvents::LoadHistory4 => "Load History 4",
            HeadersEvents::LoadHistory5 => "Load History 5",
        };
        write!(f, "{display_str}")
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
            HeadersEvents::NextCol | HeadersEvents::NextAuth | HeadersEvents::NextColForce => {
                ctx.headers.borrow_mut().next_col();
            }
            HeadersEvents::PrevCol | HeadersEvents::PrevAuth | HeadersEvents::PrevColForce => {
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
            HeadersEvents::LoadHistory1 => {
                ctx.messages.borrow_mut().handle_history_reload(1);
            }
            HeadersEvents::LoadHistory2 => {
                ctx.messages.borrow_mut().handle_history_reload(2);
            }
            HeadersEvents::LoadHistory3 => {
                ctx.messages.borrow_mut().handle_history_reload(3);
            }
            HeadersEvents::LoadHistory4 => {
                ctx.messages.borrow_mut().handle_history_reload(4);
            }
            HeadersEvents::LoadHistory5 => {
                ctx.messages.borrow_mut().handle_history_reload(5);
            }
            HeadersEvents::SaveHistory => {
                let history = &ctx.messages.borrow().history_model;
                history.save(&ctx.messages.borrow());
            }
        }
    }

    fn key_event_mappings(ctx: &Self::Context) -> Vec<(KeyEvent, HeadersEvents)> {
        let headers = ctx.headers.borrow();
        let disabled_root_events = headers.disabled_root_events();
        let selected_editor = headers.selected_editor();
        let (is_first_col, is_last_col) = match selected_editor {
            Some(e) => (e.is_first_col(), e.is_last_col()),
            None => (true, true),
        };
        let enable_switch_auth_tab = ctx.headers.borrow().tab == HeadersTab::Auth
            && selected_editor.map_or(true, TextEditor::is_empty);
        let enable_switch_col_force = ctx.headers.borrow().tab == HeadersTab::Meta;
        let enable_next_col = enable_switch_col_force && is_last_col;
        let enable_prev_col = enable_switch_col_force && is_first_col;
        let mut map = Vec::new();
        map.extend([(KeyEvent::new(KeyCode::Enter), HeadersEvents::NextTab)]);
        if !disabled_root_events {
            map.extend([
                (KeyEvent::new(KeyCode::Tab), HeadersEvents::NextTab),
                (KeyEvent::shift(KeyCode::BackTab), HeadersEvents::PrevTab),
                (KeyEvent::new(KeyCode::Down), HeadersEvents::NextRow),
                (KeyEvent::new(KeyCode::Char('j')), HeadersEvents::NextRow),
                (KeyEvent::new(KeyCode::Up), HeadersEvents::PrevRow),
                (KeyEvent::new(KeyCode::Char('k')), HeadersEvents::PrevRow),
                (
                    KeyEvent::new(KeyCode::Char('1')),
                    HeadersEvents::LoadHistory1,
                ),
                (
                    KeyEvent::new(KeyCode::Char('2')),
                    HeadersEvents::LoadHistory2,
                ),
                (
                    KeyEvent::new(KeyCode::Char('3')),
                    HeadersEvents::LoadHistory3,
                ),
                (
                    KeyEvent::new(KeyCode::Char('4')),
                    HeadersEvents::LoadHistory4,
                ),
                (
                    KeyEvent::new(KeyCode::Char('5')),
                    HeadersEvents::LoadHistory5,
                ),
                (
                    KeyEvent::ctrl(KeyCode::Char('s')),
                    HeadersEvents::SaveHistory,
                ),
            ]);
        }
        if !disabled_root_events && enable_switch_auth_tab {
            map.extend([
                (KeyEvent::new(KeyCode::Right), HeadersEvents::NextAuth),
                (KeyEvent::new(KeyCode::Char('l')), HeadersEvents::NextAuth),
            ]);
        }
        if !disabled_root_events && enable_switch_auth_tab {
            map.extend([
                (KeyEvent::new(KeyCode::Left), HeadersEvents::PrevAuth),
                (KeyEvent::new(KeyCode::Char('h')), HeadersEvents::PrevAuth),
            ]);
        }
        if !disabled_root_events && enable_next_col {
            map.extend([
                (KeyEvent::new(KeyCode::Right), HeadersEvents::NextCol),
                (KeyEvent::new(KeyCode::Char('l')), HeadersEvents::NextCol),
            ]);
        }
        if !disabled_root_events && enable_prev_col {
            map.extend([
                (KeyEvent::new(KeyCode::Left), HeadersEvents::PrevCol),
                (KeyEvent::new(KeyCode::Char('h')), HeadersEvents::PrevCol),
            ]);
        }
        if enable_switch_col_force {
            map.extend([
                (
                    KeyEvent::shift(KeyCode::Char('L')),
                    HeadersEvents::NextColForce,
                ),
                (
                    KeyEvent::shift(KeyCode::Char('H')),
                    HeadersEvents::PrevColForce,
                ),
            ]);
        }
        map.extend([(
            KeyEvent::ctrl(KeyCode::Char('a')),
            HeadersEvents::AddHeaders,
        )]);
        if enable_switch_col_force {
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
            HeadersTab::None => (),
        }
    }
}
