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
    HeadersEvents(HeadersEvents),
    SelectionEvents(ServicesSelectionEvents),
    ReflectionEvents(ReflectionEvents),
}

pub enum ReflectionEvents {
    CloseDialog,
    TriggerReflection,
}

pub struct ReflectionDialogEventHandler;

impl EventHandler for ReflectionDialogEventHandler {
    type Context = AppContext;

    type Event = ReflectionDialogEvents;

    fn handle_event(event: &ReflectionDialogEvents, ctx: &mut Self::Context) {
        match event {
            ReflectionDialogEvents::HeadersEvents(event) => {
                HeadersEventHandler::handle_event(event, ctx)
            }
            ReflectionDialogEvents::SelectionEvents(event) => {
                ServicesSelectionEventsHandler::handle_event(event, ctx)
            }
            ReflectionDialogEvents::ReflectionEvents(events) => match events {
                ReflectionEvents::TriggerReflection => {
                    ctx.reflection.borrow_mut().dispatch_reflection()
                }
                ReflectionEvents::CloseDialog => {
                    ctx.reflection
                        .borrow()
                        .selection
                        .borrow_mut()
                        .selection_mode = SelectionMode::File
                }
            },
        }
    }

    fn key_event_mappings(ctx: &Self::Context) -> Vec<(KeyEvent, ReflectionDialogEvents)> {
        let headers = ctx.headers.borrow();
        let disabled_root_events = headers.disabled_root_events();
        let selected_editor = headers.selected_editor();
        let (is_first_col, is_last_col) = match selected_editor {
            Some(e) => (e.is_first_col(), e.is_last_col()),
            None => (true, true),
        };
        let selected_tab = ctx.headers.borrow().tab.clone();
        let enable_switch_auth_tab =
            selected_tab == HeadersTab::Auth && selected_editor.map_or(true, TextEditor::is_empty);
        let enable_next_col = is_last_col;
        let enable_prev_col = is_first_col;
        let mut map = Vec::new();
        map.extend([
            (
                KeyEvent::new(KeyCode::Esc),
                ReflectionDialogEvents::ReflectionEvents(ReflectionEvents::CloseDialog),
            ),
            (
                KeyEvent::new(KeyCode::Enter),
                ReflectionDialogEvents::ReflectionEvents(ReflectionEvents::TriggerReflection),
            ),
            (
                KeyEvent::ctrl(KeyCode::Char('r')),
                ReflectionDialogEvents::SelectionEvents(ServicesSelectionEvents::FileMode),
            ),
        ]);
        if !disabled_root_events {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Tab),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::NextTab),
                ),
                (
                    KeyEvent::shift(KeyCode::BackTab),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::PrevTab),
                ),
            ]);
        }
        if selected_tab == HeadersTab::Addr || selected_tab == HeadersTab::None {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Down),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::NextRow),
                ),
                (
                    KeyEvent::new(KeyCode::Char('j')),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::NextRow),
                ),
            ]);
        }
        if selected_tab == HeadersTab::Auth {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Up),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::PrevRow),
                ),
                (
                    KeyEvent::new(KeyCode::Char('k')),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::PrevRow),
                ),
            ]);
        }

        if !disabled_root_events && enable_switch_auth_tab {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Right),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::NextAuth),
                ),
                (
                    KeyEvent::new(KeyCode::Char('l')),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::NextAuth),
                ),
            ]);
        }
        if !disabled_root_events && enable_switch_auth_tab {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Left),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::PrevAuth),
                ),
                (
                    KeyEvent::new(KeyCode::Char('h')),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::PrevAuth),
                ),
            ]);
        }
        if !disabled_root_events && enable_next_col {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Right),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::NextCol),
                ),
                (
                    KeyEvent::new(KeyCode::Char('l')),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::NextCol),
                ),
            ]);
        }
        if !disabled_root_events && enable_prev_col {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Left),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::PrevCol),
                ),
                (
                    KeyEvent::new(KeyCode::Char('h')),
                    ReflectionDialogEvents::HeadersEvents(HeadersEvents::PrevCol),
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
