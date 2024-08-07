use crate::context::{AppContext, SelectionTab};
use std::fmt;
use tui_key_event_handler::{EventHandler, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServicesSelectionEvents {
    Next,
    Prev,
    Select,
    Search,
    ClearSearch,
    GoToMethods,
}

impl fmt::Display for ServicesSelectionEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            ServicesSelectionEvents::Next => "Next",
            ServicesSelectionEvents::Prev => "Prev",
            ServicesSelectionEvents::Select => "Select",
            ServicesSelectionEvents::Search => "Search",
            ServicesSelectionEvents::ClearSearch => "Clear Search",
            ServicesSelectionEvents::GoToMethods => "Go to Methods",
        };
        write!(f, "{}", display_str)
    }
}

pub struct ServicesSelectionEventsHandler;

impl EventHandler for ServicesSelectionEventsHandler {
    type Context = AppContext;

    type Event = ServicesSelectionEvents;

    fn handle_event(event: &ServicesSelectionEvents, ctx: &mut Self::Context) {
        match event {
            ServicesSelectionEvents::Next => {
                ctx.selection.borrow_mut().next_service();
                ctx.selection.borrow_mut().clear_methods_selection();
            }
            ServicesSelectionEvents::Prev => {
                ctx.selection.borrow_mut().previous_service();
                ctx.selection.borrow_mut().clear_methods_selection();
            }
            ServicesSelectionEvents::Select => {
                ctx.selection_tab = SelectionTab::Methods;
                if ctx.selection.borrow().selected_method().is_none() {
                    ctx.selection.borrow_mut().next_method();
                }
            }
            ServicesSelectionEvents::Search => {
                ctx.selection_tab = SelectionTab::SearchServices;
                ctx.disable_root_events = true;
            }
            ServicesSelectionEvents::ClearSearch => {
                if ctx.selection.borrow().services_filter.is_some() {
                    ctx.selection.borrow_mut().clear_services_filter();
                }
            }
            ServicesSelectionEvents::GoToMethods => {
                ctx.selection_tab = SelectionTab::Methods;
            }
        }
    }

    fn key_event_mappings(ctx: &Self::Context) -> Vec<(KeyEvent, ServicesSelectionEvents)> {
        let mut map = Vec::from([
            (KeyEvent::new(KeyCode::Down), ServicesSelectionEvents::Next),
            (
                KeyEvent::new(KeyCode::Char('j')),
                ServicesSelectionEvents::Next,
            ),
            (KeyEvent::new(KeyCode::Up), ServicesSelectionEvents::Prev),
            (
                KeyEvent::new(KeyCode::Char('k')),
                ServicesSelectionEvents::Prev,
            ),
            (KeyEvent::new(KeyCode::Tab), ServicesSelectionEvents::Select),
            (
                KeyEvent::new(KeyCode::Enter),
                ServicesSelectionEvents::Select,
            ),
            (
                KeyEvent::new(KeyCode::Char('/')),
                ServicesSelectionEvents::Search,
            ),
            (
                KeyEvent::shift(KeyCode::Char('J')),
                ServicesSelectionEvents::GoToMethods,
            ),
        ]);
        if ctx.selection.borrow().services_filter.is_some() {
            map.extend([(
                KeyEvent::new(KeyCode::Esc),
                ServicesSelectionEvents::ClearSearch,
            )])
        }
        map
    }
}
