use crate::{
    context::{AppContext, SelectionTab},
    model::selection::SelectionMode,
    widgets::editor::yank_to_clipboard,
};
use event_handler::{EventHandler, KeyCode, KeyEvent};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServicesSelectionEvents {
    Next,
    Prev,
    Select,
    Search,
    ClearSearch,
    GoToMethods,
    ToggleReflectionMode,
    UntoggleReflectionMode,
    EditConfig,
    YankWebsiteLink,
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
            ServicesSelectionEvents::ToggleReflectionMode => "Toggle Reflection Mode",
            ServicesSelectionEvents::UntoggleReflectionMode => "Untoggle Reflection Mode",
            ServicesSelectionEvents::EditConfig => "Edit Configuration",
            ServicesSelectionEvents::YankWebsiteLink => "Yank website link",
        };
        write!(f, "{display_str}")
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
            ServicesSelectionEvents::ToggleReflectionMode
            | ServicesSelectionEvents::UntoggleReflectionMode => {
                ctx.selection.borrow_mut().toggle_reflection_mode();
            }
            ServicesSelectionEvents::EditConfig => {
                ctx.configuration.borrow_mut().toggle();
            }
            ServicesSelectionEvents::YankWebsiteLink => {
                yank_to_clipboard("https://preiter93.github.io/wireman/");
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
        if ctx.selection.borrow().selection_mode == SelectionMode::File {
            map.extend([(
                KeyEvent::ctrl(KeyCode::Char('r')),
                ServicesSelectionEvents::ToggleReflectionMode,
            )]);
        } else {
            map.extend([(
                KeyEvent::ctrl(KeyCode::Char('r')),
                ServicesSelectionEvents::UntoggleReflectionMode,
            )]);
        }
        map.extend([(
            KeyEvent::ctrl(KeyCode::Char('e')),
            ServicesSelectionEvents::EditConfig,
        )]);
        if ctx.selection.borrow().services_filter.is_some() {
            map.extend([(
                KeyEvent::new(KeyCode::Esc),
                ServicesSelectionEvents::ClearSearch,
            )]);
        }
        if !ctx.selection.borrow().has_services() {
            map.extend([(
                KeyEvent::new(KeyCode::Char('y')),
                ServicesSelectionEvents::YankWebsiteLink,
            )]);
        }
        map
    }
}
