use crate::{
    context::{AppContext, SelectionTab},
    model::selection::SelectionMode,
};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use event_handler::{EventHandler, KeyCode, KeyEvent};
use std::fmt;
use tui_widget_list::hit_test::Hit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodsSelectionEvents {
    Next,
    Prev,
    Select,
    NextTab,
    PrevTab,
    Search,
    Unselect,
    ClearSearch,
    GoToServices,
    ToggleReflectionMode,
    UntoggleReflectionMode,
    EditConfig,
}

impl fmt::Display for MethodsSelectionEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            MethodsSelectionEvents::Next => "Next",
            MethodsSelectionEvents::Prev => "Prev",
            MethodsSelectionEvents::NextTab => "Next Page",
            MethodsSelectionEvents::PrevTab => "Prev Page",
            MethodsSelectionEvents::Select => "Select",
            MethodsSelectionEvents::Search => "Search",
            MethodsSelectionEvents::Unselect => "Unselect",
            MethodsSelectionEvents::ClearSearch => "Clear Search",
            MethodsSelectionEvents::GoToServices => "Go to Services",
            MethodsSelectionEvents::ToggleReflectionMode => "Toggle Reflection Mode",
            MethodsSelectionEvents::UntoggleReflectionMode => "Untoggle Reflection Mode",
            MethodsSelectionEvents::EditConfig => "Edit Configuration",
        };
        write!(f, "{display_str}")
    }
}

pub struct MethodsSelectionEventsHandler;

impl EventHandler for MethodsSelectionEventsHandler {
    type Context = AppContext;

    type Event = MethodsSelectionEvents;

    fn handle_event(event: &MethodsSelectionEvents, ctx: &mut Self::Context) {
        match event {
            MethodsSelectionEvents::Next => {
                ctx.selection.borrow_mut().next_method();
            }
            MethodsSelectionEvents::Prev => {
                ctx.selection.borrow_mut().previous_method();
            }
            MethodsSelectionEvents::Select => {
                if ctx.selection.borrow().selected_method().is_none() {
                    ctx.selection.borrow_mut().next_method();
                }
            }
            MethodsSelectionEvents::NextTab => {
                if let Some(method) = ctx.selection.borrow().selected_method() {
                    ctx.messages.borrow_mut().load_method(&method);
                    ctx.headers.borrow_mut().set_method(&method);
                } else {
                    ctx.messages.borrow_mut().set_no_method_error();
                }
                ctx.tab = ctx.tab.next();
            }
            MethodsSelectionEvents::PrevTab => {
                if let Some(method) = ctx.selection.borrow().selected_method() {
                    ctx.messages.borrow_mut().load_method(&method);
                    ctx.headers.borrow_mut().set_method(&method);
                } else {
                    ctx.messages.borrow_mut().set_no_method_error();
                }
                ctx.tab = ctx.tab.prev();
            }
            MethodsSelectionEvents::Search => {
                ctx.selection_tab = SelectionTab::SearchMethods;
                ctx.disable_root_events = true;
            }
            MethodsSelectionEvents::Unselect => {
                if ctx.selection.borrow_mut().methods_filter.is_some() {
                    ctx.selection.borrow_mut().clear_methods_filter();
                } else {
                    ctx.selection_tab = SelectionTab::Services;
                    ctx.selection.borrow_mut().clear_methods_selection();
                }
            }
            MethodsSelectionEvents::ClearSearch => {
                ctx.selection.borrow_mut().clear_methods_filter();
            }
            MethodsSelectionEvents::GoToServices => {
                ctx.selection_tab = SelectionTab::Services;
            }
            MethodsSelectionEvents::ToggleReflectionMode
            | MethodsSelectionEvents::UntoggleReflectionMode => {
                ctx.selection.borrow_mut().toggle_reflection_mode();
            }
            MethodsSelectionEvents::EditConfig => {
                ctx.configuration.borrow_mut().toggle();
            }
        }
    }

    fn key_event_mappings(ctx: &Self::Context) -> Vec<(KeyEvent, MethodsSelectionEvents)> {
        let method_selected = ctx.selection.borrow().selected_method().is_some();
        let filter_active = ctx.selection.borrow_mut().methods_filter.is_some();
        let mut map = vec![];
        if method_selected {
            map.extend([
                (
                    KeyEvent::new(KeyCode::Enter),
                    MethodsSelectionEvents::NextTab,
                ),
                (KeyEvent::new(KeyCode::Tab), MethodsSelectionEvents::NextTab),
                (
                    KeyEvent::shift(KeyCode::BackTab),
                    MethodsSelectionEvents::PrevTab,
                ),
            ]);
        } else {
            map.extend([(
                KeyEvent::new(KeyCode::Enter),
                MethodsSelectionEvents::Select,
            )]);
        }

        if ctx.selection.borrow().selection_mode == SelectionMode::File {
            map.extend([(
                KeyEvent::ctrl(KeyCode::Char('r')),
                MethodsSelectionEvents::ToggleReflectionMode,
            )]);
        } else {
            map.extend([(
                KeyEvent::ctrl(KeyCode::Char('r')),
                MethodsSelectionEvents::UntoggleReflectionMode,
            )]);
        }

        if filter_active {
            map.extend([(
                KeyEvent::new(KeyCode::Esc),
                MethodsSelectionEvents::ClearSearch,
            )]);
        } else {
            map.extend([(
                KeyEvent::new(KeyCode::Esc),
                MethodsSelectionEvents::Unselect,
            )]);
        }
        map.extend(vec![
            (KeyEvent::new(KeyCode::Down), MethodsSelectionEvents::Next),
            (
                KeyEvent::new(KeyCode::Char('j')),
                MethodsSelectionEvents::Next,
            ),
            (KeyEvent::new(KeyCode::Up), MethodsSelectionEvents::Prev),
            (
                KeyEvent::new(KeyCode::Char('k')),
                MethodsSelectionEvents::Prev,
            ),
            (
                KeyEvent::new(KeyCode::Char('/')),
                MethodsSelectionEvents::Search,
            ),
            (
                KeyEvent::shift(KeyCode::Char('K')),
                MethodsSelectionEvents::GoToServices,
            ),
        ]);
        map.extend([(
            KeyEvent::ctrl(KeyCode::Char('e')),
            MethodsSelectionEvents::EditConfig,
        )]);
        map
    }

    fn pass_through_mouse_events(event: &MouseEvent, ctx: &mut Self::Context) {
        if let MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column,
            row,
            ..
        } = *event
        {
            let hit = {
                let selection = ctx.selection.borrow();
                selection.methods_state.hit_test(column, row)
            };

            match hit {
                Some(Hit::Item(index)) => {
                    if ctx.selection_tab == SelectionTab::Methods {
                        {
                            let state = &mut ctx.selection.borrow_mut().methods_state;
                            state.select(Some(index));
                        }
                        if let Some(method) = ctx.selection.borrow().selected_method() {
                            ctx.messages.borrow_mut().load_method(&method);
                            ctx.headers.borrow_mut().set_method(&method);
                        } else {
                            ctx.messages.borrow_mut().set_no_method_error();
                        }
                        ctx.tab = ctx.tab.next();
                    } else {
                        ctx.selection_tab = SelectionTab::Methods;
                    }
                }
                Some(Hit::Area) => {
                    ctx.selection_tab = SelectionTab::Methods;
                }
                None => {}
            }
        }
    }
}
