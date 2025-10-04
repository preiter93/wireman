mod crossterm;
pub mod key_event;
use std::collections::HashMap;
use std::fmt::Display;

use ::crossterm::event::MouseEvent;
pub use key_event::KeyCode;
pub use key_event::KeyEvent;
pub use key_event::{KeyModifier, KeyModifiers};
use std::fmt::Write as _;

/// A trait for handling events with associated contexts.
pub trait EventHandler {
    /// The context in which events are handled.
    type Context;

    /// The type of events to handle.
    type Event;

    /// Handles a specific app event with the given context.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to handle.
    /// * `ctx` - The context in which the event is handled.
    fn handle_event(event: &Self::Event, ctx: &mut Self::Context);

    /// Passes through key events without any specific handling.
    ///
    /// This method is optional and can be used to simply pass through character events without
    /// registering any specific handling logic.
    fn pass_through_key_events(_: &KeyEvent, _: &mut Self::Context) {}

    /// Passes through mouse events without any specific handling.
    ///
    /// This method is optional and can be used to simply pass through character events without
    /// registering any specific handling logic.
    fn pass_through_mouse_events(_: &MouseEvent, _: &mut Self::Context) {}

    /// Passes through paste events without any specific handling.
    ///
    /// This method is optional and can be used to simply pass through character events without
    /// registering any specific handling logic.
    fn pass_through_paste_events(_: String, _: &mut Self::Context) {}

    /// Retrieves the key event mappings to their corresponding application events.
    ///
    /// Returns a map of key events to application events.
    fn key_event_mappings(ctx: &Self::Context) -> Vec<(KeyEvent, Self::Event)>;

    /// Handles a key event by dispatching it to the corresponding application event handler.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context in which the key event is handled.
    /// * `event` - The key event to handle.
    fn handle_key_event<T: Into<KeyEvent>>(ctx: &mut Self::Context, event: T) {
        let mappings = Self::key_event_mappings(ctx);
        let event = event.into();
        if let Some(item) = mappings.iter().find(|item| item.0 == event) {
            Self::handle_event(&item.1, ctx);
        } else {
            Self::pass_through_key_events(&event, ctx);
        }
    }

    /// Handles a mouse event by dispatching it to the corresponding application event handler.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context in which the mouse event is handled.
    /// * `event` - The mouse event to handle.
    fn handle_mouse_event<T: Into<MouseEvent>>(ctx: &mut Self::Context, event: T) {
        let event = event.into();
        Self::pass_through_mouse_events(&event, ctx);
    }

    /// Handles a paste event by dispatching it to the corresponding application event handler.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context in which the paste event is handled.
    /// * `text` - The text to paste.
    fn handle_paste_event(ctx: &mut Self::Context, text: String) {
        Self::pass_through_paste_events(text, ctx);
    }

    /// Converts the key event mappings into a vector of string representations, i.e.
    /// (app event as string, key event as string).
    ///
    /// This method formats each key event and corresponding app event
    /// as strings and returns them as a vector of tuples. If multiple key events
    /// map to the same app event, they are merged into a single entry where
    /// the keys are concatenated together.
    fn format_event_mappings_as_strings(ctx: &Self::Context) -> Vec<(String, String)>
    where
        Self::Event: Display,
    {
        let mappings = Self::key_event_mappings(ctx);
        let mut formatted_events: Vec<(String, String)> = Vec::new();
        let mut event_indices: HashMap<String, usize> = HashMap::new();

        for (key_event, event) in mappings {
            let key_event_str = key_event.to_string();
            let event_str = event.to_string();

            if let Some(&index) = event_indices.get(&event_str) {
                let mut s = String::new();
                let _ = write!(s, ", {key_event_str}");
                formatted_events[index].0.push_str(&s);
            } else {
                formatted_events.push((key_event_str, event_str.clone()));
                event_indices.insert(event_str, formatted_events.len() - 1);
            }
        }

        formatted_events
    }
}
