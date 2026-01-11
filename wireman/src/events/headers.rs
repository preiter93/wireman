use crate::{context::AppContext, model::headers::HeadersTab, widgets::editor::TextEditor};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use edtui::EditorMode;
use event_handler::{EventHandler, KeyCode, KeyEvent};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::fmt;

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
    AddHeaders,
    DelHeaders,
    SaveHistory,
    ResetHistory,
    LoadHistory1,
    LoadHistory2,
    LoadHistory3,
    LoadHistory4,
    LoadHistory5,
}

impl fmt::Display for HeadersEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            HeadersEvents::NextTab => "Next Page",
            HeadersEvents::PrevTab => "Prev Page",
            HeadersEvents::NextRow => "Next Row",
            HeadersEvents::PrevRow => "Prev Row",
            HeadersEvents::NextCol => "Next Column",
            HeadersEvents::PrevCol => "Prev Column",
            HeadersEvents::NextAuth => "Next auth mode",
            HeadersEvents::PrevAuth => "Prev auth mode",
            HeadersEvents::NextColForce => "Next Column (Force)",
            HeadersEvents::PrevColForce => "Prev Column (Force)",
            HeadersEvents::AddHeaders => "Add Headers",
            HeadersEvents::DelHeaders => "Del Headers",
            HeadersEvents::SaveHistory => "Save Request",
            HeadersEvents::ResetHistory => "Reset Request",
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
            HeadersEvents::AddHeaders => {
                ctx.headers.borrow_mut().meta.add();
            }
            HeadersEvents::DelHeaders => {
                let selected = ctx.headers.borrow().meta.selected;
                if let Some(index) = selected {
                    if ctx.headers.borrow_mut().meta.len() == 1 {
                        ctx.headers.borrow_mut().meta.clear();
                        ctx.headers.borrow_mut().meta.add();
                        ctx.headers.borrow_mut().tab = HeadersTab::None;
                    } else {
                        ctx.headers.borrow_mut().meta.remove(index.row);
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
                let history = &ctx.messages.borrow().history;
                history.borrow_mut().save(&ctx.messages.borrow());
            }
            HeadersEvents::ResetHistory => {
                let method = ctx.messages.borrow().selected_method.clone();
                if let Some(method) = method {
                    ctx.history.borrow_mut().delete(&method);
                    ctx.messages.borrow_mut().request.load_template(&method);
                    ctx.messages.borrow_mut().headers.borrow_mut().clear();
                }
            }
        }
    }

    #[allow(clippy::too_many_lines)]
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
        let enable_switch_col_force = ctx.headers.borrow().tab == HeadersTab::Meta
            || ctx.headers.borrow().tab == HeadersTab::Auth;
        let enable_next_col = enable_switch_col_force
            && (is_last_col && ctx.headers.borrow().tab == HeadersTab::Meta);
        let enable_prev_col = enable_switch_col_force
            && (is_first_col && ctx.headers.borrow().tab == HeadersTab::Meta);
        let mut map = Vec::new();
        map.extend([(KeyEvent::new(KeyCode::Enter), HeadersEvents::NextTab)]);
        if !disabled_root_events {
            map.extend([
                (KeyEvent::new(KeyCode::Tab), HeadersEvents::NextTab),
                (KeyEvent::shift(KeyCode::BackTab), HeadersEvents::PrevTab),
                (KeyEvent::new(KeyCode::Down), HeadersEvents::NextRow),
                (KeyEvent::new(KeyCode::Char('j')), HeadersEvents::NextRow),
                (KeyEvent::shift(KeyCode::Char('J')), HeadersEvents::NextRow),
                (KeyEvent::new(KeyCode::Up), HeadersEvents::PrevRow),
                (KeyEvent::new(KeyCode::Char('k')), HeadersEvents::PrevRow),
                (KeyEvent::shift(KeyCode::Char('K')), HeadersEvents::PrevRow),
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
                (
                    KeyEvent::ctrl(KeyCode::Char('q')),
                    HeadersEvents::ResetHistory,
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

    fn pass_through_key_events<B: Backend>(
        key_event: &KeyEvent,
        ctx: &mut Self::Context,
        terminal: &mut Terminal<B>,
    ) {
        let tab = ctx.headers.borrow().tab.clone();
        match tab {
            HeadersTab::Meta => {
                if let Some(input) = ctx.headers.borrow_mut().selected_editor_mut() {
                    input.on_key(key_event.clone().into(), terminal);
                    ctx.disable_root_events = !(input.normal_mode());
                }
            }
            HeadersTab::Addr => {
                let input = &mut ctx.headers.borrow_mut().addr;
                input.on_key(key_event.clone().into(), terminal);
                ctx.disable_root_events = !(input.normal_mode());
            }
            HeadersTab::Auth => {
                let mut headers = ctx.headers.borrow_mut();
                let input = headers.auth.selected_editor_mut();
                input.on_key(key_event.clone().into(), terminal);
                ctx.disable_root_events = !(input.normal_mode());
            }
            HeadersTab::None => (),
        }
    }

    fn pass_through_paste_events(text: String, ctx: &mut Self::Context) {
        let tab = ctx.headers.borrow().tab.clone();
        match tab {
            HeadersTab::Meta => {
                if let Some(input) = ctx.headers.borrow_mut().selected_editor_mut() {
                    input.on_paste(text);
                }
            }
            HeadersTab::Addr => {
                let input = &mut ctx.headers.borrow_mut().addr;
                input.on_paste(text);
            }
            HeadersTab::Auth => {
                let mut headers = ctx.headers.borrow_mut();
                let input = headers.auth.selected_editor_mut();
                input.on_paste(text);
            }
            HeadersTab::None => (),
        }
    }

    fn pass_through_mouse_events(event: &MouseEvent, ctx: &mut Self::Context) {
        if let MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column,
            row,
            ..
        } = *event
        {
            let disabled = ctx.headers.borrow().disabled_root_events();
            if disabled {
                let current_tab = {
                    let h = ctx.headers.borrow();
                    h.tab.clone()
                };
                match current_tab {
                    HeadersTab::Meta => {
                        if let Some(input) = ctx.headers.borrow_mut().selected_editor_mut() {
                            input.state.mode = EditorMode::Normal;
                        }
                    }
                    HeadersTab::Addr => {
                        let input = &mut ctx.headers.borrow_mut().addr;
                        input.state.mode = EditorMode::Normal;
                    }
                    HeadersTab::Auth => {
                        let mut headers = ctx.headers.borrow_mut();
                        let input = headers.auth.selected_editor_mut();
                        input.state.mode = EditorMode::Normal;
                    }
                    HeadersTab::None => {}
                }
                ctx.disable_root_events = false;
                return;
            }
            let pos = ratatui::prelude::Position { x: column, y: row };

            let headers_ref = ctx.headers.borrow();
            let current = headers_ref.tab.clone();
            let mut target = None;
            let mut meta_key_clicked = false;
            let mut meta_value_clicked = false;

            if headers_ref.addr_title_area.is_some_and(|r| r.contains(pos))
                || headers_ref
                    .addr_content_area
                    .is_some_and(|r| r.contains(pos))
            {
                target = Some(HeadersTab::Addr);
            } else if headers_ref.auth_title_area.is_some_and(|r| r.contains(pos))
                || headers_ref
                    .auth_content_area
                    .is_some_and(|r| r.contains(pos))
            {
                target = Some(HeadersTab::Auth);
            } else if headers_ref.meta_title_area.is_some_and(|r| r.contains(pos)) {
                target = Some(HeadersTab::Meta);
                meta_key_clicked = true;
            } else if let Some(r) = headers_ref.meta_content_area {
                // TODO: Handle more then one row
                if r.contains(pos) {
                    target = Some(HeadersTab::Meta);
                    let mid_x = r.x + r.width / 2;
                    if column < mid_x {
                        meta_key_clicked = true;
                    } else {
                        meta_value_clicked = true;
                    }
                }
            }
            drop(headers_ref);

            if let Some(t) = target {
                if t != current {
                    // First click focuses section; if meta is selected select appropriate column.
                    let mut headers = ctx.headers.borrow_mut();
                    let is_meta = t == HeadersTab::Meta;
                    headers.tab = t;
                    if is_meta {
                        headers.meta.select();
                        if meta_value_clicked {
                            headers.meta.next_col();
                        }
                    }
                    return;
                }
            }

            let tab = ctx.headers.borrow().tab.clone();
            if tab == HeadersTab::Meta {
                let mut headers = ctx.headers.borrow_mut();
                if let Some(sel) = headers.meta.selected {
                    if meta_value_clicked && sel.col == 0 {
                        headers.meta.next_col();
                    } else if meta_key_clicked && sel.col == 1 {
                        headers.meta.prev_col();
                    }
                }
            }
            // Do not forward mouse to editor here
        }
    }
}
