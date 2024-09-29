use crate::{
    events::ServicesSelectionEventsHandler,
    model::{headers::HeadersTab, selection::SelectionMode},
    widgets::editor::TextEditor,
};
use event_handler::{EventHandler, KeyCode, KeyEvent};

use crate::{
    context::AppContext,
    events::headers::{HeadersEventHandler, HeadersEvents},
};

use super::services::ServicesSelectionEvents;

pub enum ReflectionDialogEvents {
    Headers(HeadersEvents),
    Selection(ServicesSelectionEvents),
    Reflection(ReflectionEvents),
}

impl std::fmt::Display for ReflectionDialogEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Headers(events) => events.fmt(f),
            Self::Selection(events) => events.fmt(f),
            Self::Reflection(events) => events.fmt(f),
        }
    }
}

pub enum ReflectionEvents {
    CloseDialog,
    ReflectServer,
}

impl std::fmt::Display for ReflectionEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            ReflectionEvents::ReflectServer => "Reflect server",
            ReflectionEvents::CloseDialog => "Close dialog",
        };
        write!(f, "{display_str}")
    }
}

pub struct ReflectionDialogEventHandler;

impl EventHandler for ReflectionDialogEventHandler {
    type Context = AppContext;

    type Event = ReflectionDialogEvents;

    fn handle_event(event: &ReflectionDialogEvents, ctx: &mut Self::Context) {
        match event {
            ReflectionDialogEvents::Headers(event) => HeadersEventHandler::handle_event(event, ctx),
            ReflectionDialogEvents::Selection(event) => {
                ServicesSelectionEventsHandler::handle_event(event, ctx);
            }
            ReflectionDialogEvents::Reflection(events) => match events {
                ReflectionEvents::ReflectServer => {
                    ctx.reflection.borrow_mut().dispatch_reflection();
                }
                ReflectionEvents::CloseDialog => {
                    ctx.reflection
                        .borrow()
                        .selection
                        .borrow_mut()
                        .selection_mode = SelectionMode::File;
                }
            },
        }
    }

    fn key_event_mappings(ctx: &Self::Context) -> Vec<(KeyEvent, ReflectionDialogEvents)> {
        let headers = ctx.headers.borrow();
        let disabled_root_events = headers.disabled_root_events();
        let selected_editor = headers.selected_editor();
        let selected_tab = ctx.headers.borrow().tab.clone();
        let enable_switch_auth_tab =
            selected_tab == HeadersTab::Auth && selected_editor.map_or(true, TextEditor::is_empty);
        let mut map = Vec::new();
        if !disabled_root_events {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Esc),
                    ReflectionDialogEvents::Reflection(ReflectionEvents::CloseDialog),
                ),
                (
                    KeyEvent::new(KeyCode::Enter),
                    ReflectionDialogEvents::Reflection(ReflectionEvents::ReflectServer),
                ),
                (
                    KeyEvent::ctrl(KeyCode::Char('r')),
                    ReflectionDialogEvents::Selection(ServicesSelectionEvents::FileMode),
                ),
            ]);
        }
        if !disabled_root_events && selected_tab == HeadersTab::Addr
            || selected_tab == HeadersTab::None
        {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Down),
                    ReflectionDialogEvents::Headers(HeadersEvents::NextRow),
                ),
                (
                    KeyEvent::new(KeyCode::Char('j')),
                    ReflectionDialogEvents::Headers(HeadersEvents::NextRow),
                ),
            ]);
        }
        if !disabled_root_events && selected_tab == HeadersTab::Auth {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Up),
                    ReflectionDialogEvents::Headers(HeadersEvents::PrevRow),
                ),
                (
                    KeyEvent::new(KeyCode::Char('k')),
                    ReflectionDialogEvents::Headers(HeadersEvents::PrevRow),
                ),
            ]);
        }

        if !disabled_root_events && enable_switch_auth_tab {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Right),
                    ReflectionDialogEvents::Headers(HeadersEvents::NextAuth),
                ),
                (
                    KeyEvent::new(KeyCode::Char('l')),
                    ReflectionDialogEvents::Headers(HeadersEvents::NextAuth),
                ),
            ]);
        }
        if !disabled_root_events && enable_switch_auth_tab {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Left),
                    ReflectionDialogEvents::Headers(HeadersEvents::PrevAuth),
                ),
                (
                    KeyEvent::new(KeyCode::Char('h')),
                    ReflectionDialogEvents::Headers(HeadersEvents::PrevAuth),
                ),
            ]);
        }

        map
    }

    fn pass_through_key_events(key_event: &KeyEvent, ctx: &mut Self::Context) {
        let tab = ctx.headers.borrow().tab.clone();
        match tab {
            HeadersTab::Addr => {
                let input = &mut ctx.headers.borrow_mut().addr;
                input.on_key(key_event.clone().into());
                ctx.disable_root_events = !(input.normal_mode());
            }
            HeadersTab::Auth => {
                let mut headers = ctx.headers.borrow_mut();
                let input = headers.auth.selected_editor_mut();
                input.on_key(key_event.clone().into());
                ctx.disable_root_events = !(input.normal_mode());
            }
            HeadersTab::Meta | HeadersTab::None => (),
        }
    }
}
