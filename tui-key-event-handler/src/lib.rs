use std::collections::HashMap;

mod crossterm;
pub mod key_event;
pub use key_event::KeyCode;
pub use key_event::KeyEvent;
pub use key_event::{KeyModifier, KeyModifiers};

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

    /// Retrieves the key event mappings to their corresponding application events.
    ///
    /// Returns a map of key events to application events.
    fn key_event_mappings(ctx: &Self::Context) -> HashMap<KeyEvent, Self::Event>;

    /// Handles a key event by dispatching it to the corresponding application event handler.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context in which the key event is handled.
    /// * `key_event` - The key event to handle.
    fn handle_key_event<T: Into<KeyEvent>>(ctx: &mut Self::Context, key_event: T) {
        let mappings = Self::key_event_mappings(&ctx);
        let key_event = key_event.into();
        if let Some(event) = mappings.get(&key_event) {
            Self::handle_event(event, ctx);
        } else {
            Self::pass_through_key_events(&key_event, ctx);
        }
    }
}
