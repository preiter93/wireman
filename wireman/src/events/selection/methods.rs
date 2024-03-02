use crate::context::{AppContext, SelectionTab};
use std::collections::HashMap;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodsSelectionEvent {
    Next,
    Prev,
    Select,
    NextTab,
    PrevTab,
    Search,
    Unselect,
    GoToServices,
}

pub struct MethodsSelectionEventHandler {}

impl MethodsSelectionEventHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl EventHandler for MethodsSelectionEventHandler {
    type Context = AppContext;

    type Event = MethodsSelectionEvent;

    fn handle_event(event: &Self::Event, ctx: &mut Self::Context) {
        match event {
            MethodsSelectionEvent::Next => {
                ctx.selection.borrow_mut().next_method();
            }
            MethodsSelectionEvent::Prev => {
                ctx.selection.borrow_mut().previous_method();
            }
            MethodsSelectionEvent::Select => {
                if ctx.selection.borrow().selected_method().is_none() {
                    ctx.selection.borrow_mut().next_method();
                }
            }
            MethodsSelectionEvent::NextTab => {
                if let Some(method) = ctx.selection.borrow().selected_method() {
                    ctx.messages.borrow_mut().load_method(&method);
                } else {
                    ctx.messages.borrow_mut().set_no_method_error();
                }
                ctx.tab = ctx.tab.next();
            }
            MethodsSelectionEvent::PrevTab => {
                if let Some(method) = ctx.selection.borrow().selected_method() {
                    ctx.messages.borrow_mut().load_method(&method);
                } else {
                    ctx.messages.borrow_mut().set_no_method_error();
                }
                ctx.tab = ctx.tab.prev();
            }
            MethodsSelectionEvent::Search => {
                ctx.selection_tab = SelectionTab::SearchMethods;
                ctx.disable_root_events = true;
            }
            MethodsSelectionEvent::Unselect => {
                if ctx.selection.borrow_mut().methods_filter.is_some() {
                    ctx.selection.borrow_mut().clear_methods_filter();
                } else {
                    ctx.selection_tab = SelectionTab::Services;
                    ctx.selection.borrow_mut().clear_methods_selection();
                }
            }
            MethodsSelectionEvent::GoToServices => {
                ctx.selection_tab = SelectionTab::Services;
            }
        }
    }

    fn key_event_mappings(ctx: &Self::Context) -> HashMap<KeyEvent, Self::Event> {
        let method_selected = ctx.selection.borrow().selected_method().is_some();
        let mut map = HashMap::from([
            (KeyEvent::new(KeyCode::Down), Self::Event::Next),
            (KeyEvent::new(KeyCode::Char('j')), Self::Event::Next),
            (KeyEvent::new(KeyCode::Up), Self::Event::Prev),
            (KeyEvent::new(KeyCode::Char('k')), Self::Event::Prev),
            (KeyEvent::new(KeyCode::Char('/')), Self::Event::Search),
            (KeyEvent::new(KeyCode::Esc), Self::Event::Unselect),
            (KeyEvent::new(KeyCode::Char('K')), Self::Event::GoToServices),
        ]);
        if !method_selected {
            map.extend([(KeyEvent::new(KeyCode::Enter), Self::Event::Select)]);
        } else {
            map.extend([
                (KeyEvent::new(KeyCode::Enter), Self::Event::NextTab),
                (KeyEvent::new(KeyCode::Tab), Self::Event::NextTab),
                (KeyEvent::new(KeyCode::BackTab), Self::Event::PrevTab),
            ]);
        }
        map
    }
}
