use crate::context::AppContext;
use crossterm::event::MouseEvent;
use event_handler::{EventHandler, KeyCode, KeyEvent};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::fmt;

pub enum ConfigurationEvents {
    SaveFile,
}

impl fmt::Display for ConfigurationEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            Self::SaveFile => "Save File",
        };
        write!(f, "{display_str}")
    }
}

pub struct ConfigurationEventHandler;

impl EventHandler for ConfigurationEventHandler {
    type Context = AppContext;

    type Event = ConfigurationEvents;

    fn handle_event(_: &ConfigurationEvents, ctx: &mut Self::Context) {
        let Some(config) = ctx.configuration.borrow_mut().save_to_file() else {
            return;
        };
        ctx.reload(&config);
    }

    fn key_event_mappings(ctx: &Self::Context) -> Vec<(KeyEvent, ConfigurationEvents)> {
        let disabled_root_events = ctx.disable_root_events;
        let mut map = Vec::new();
        if !disabled_root_events {
            map.extend([(
                KeyEvent::ctrl(KeyCode::Char('s')),
                ConfigurationEvents::SaveFile,
            )]);
        }
        map
    }

    fn pass_through_key_events<B: Backend>(
        key_event: &KeyEvent,
        ctx: &mut Self::Context,
        terminal: &mut Terminal<B>,
    ) {
        if let Some(editor) = &mut ctx.configuration.borrow_mut().editor {
            editor.on_key(key_event.clone().into(), terminal);
            ctx.disable_root_events = !editor.normal_mode();
        }
    }

    fn pass_through_mouse_events(event: &MouseEvent, ctx: &mut Self::Context) {
        if let Some(editor) = &mut ctx.configuration.borrow_mut().editor {
            editor.on_mouse(*event);
            ctx.disable_root_events = !editor.normal_mode();
        }
    }

    fn pass_through_paste_events(text: String, ctx: &mut Self::Context) {
        if let Some(editor) = &mut ctx.configuration.borrow_mut().editor {
            editor.on_paste(text)
        }
    }
}
