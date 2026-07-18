pub(crate) mod configuration;
pub(crate) mod headers;
pub(crate) mod messages;
pub(crate) mod selection;
use std::fmt::Display;
use std::pin::Pin;
use wireman_core::ProtoDescriptor;

use crate::app::App;
use crate::context::{AppContext, HelpContext, MessagesTab, SelectionTab, Tab};
use crate::model::messages::{
    bidi_streaming, client_streaming, server_streaming, unary, RequestResult,
};
use crate::model::selection::SelectionMode;
use configuration::ConfigurationEventHandler;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use futures::{stream::once, Stream, StreamExt};
pub(crate) use selection::methods::MethodsSelectionEventsHandler;
pub(crate) use selection::methods_search::MethodsSearchEventsHandler;
use selection::reflection::ReflectionDialogEventHandler;
pub(crate) use selection::services::ServicesSelectionEventsHandler;
pub(crate) use selection::services_search::ServicesSearchEventsHandler;
use tokio::sync::mpsc::{self, Receiver, Sender};
use wireman_event_handler::EventHandler;
use wireman_logger::Logger;

use self::headers::HeadersEventHandler;
use self::messages::request::RequestEventHandler;
use self::messages::response::ResponseEventHandler;

pub(crate) enum InternalStreamData {
    Request(RequestResult),
    Reflection(Result<ProtoDescriptor, String>),
    Done,
}

pub(crate) struct InternalStream {
    pub(crate) sx: Sender<InternalStreamData>,
    pub(crate) rx: Receiver<InternalStreamData>,
}

impl InternalStream {
    pub(crate) fn new() -> Self {
        let (sx, rx) = mpsc::channel::<InternalStreamData>(10);
        Self { sx, rx }
    }
}

const HELP_KEY: KeyCode = KeyCode::Char('?');

/// Forwards every item of a stream to the internal channel, then signals completion.
async fn drain_to(
    mut stream: Pin<Box<dyn Stream<Item = RequestResult> + Send>>,
    sx: &Sender<InternalStreamData>,
) {
    while let Some(resp) = stream.next().await {
        let _ = sx.send(InternalStreamData::Request(resp)).await;
    }
    let _ = sx.send(InternalStreamData::Done).await;
}

impl App {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn handle_crossterm_key_event(&mut self, event: KeyEvent) {
        let sx1 = self.internal_stream.sx.clone();
        let sx2 = self.internal_stream.sx.clone();
        match event.code {
            KeyCode::Char('c') if event.modifiers == KeyModifiers::CONTROL => {
                self.should_quit = true;
            }
            _ => {
                // Help modal dialog key events.
                if self.ctx.help.is_some() {
                    match event.code {
                        KeyCode::Esc | HELP_KEY => {
                            self.ctx.help = None;
                        }
                        _ => (),
                    }
                    return;
                }
                // Configuration dialog key events
                if self.ctx.configuration.borrow().toggled() {
                    let can_exit = self
                        .ctx
                        .configuration
                        .borrow()
                        .editor
                        .as_ref()
                        .map(|e| e.normal_mode())
                        .unwrap_or(true);

                    match event.code {
                        KeyCode::Esc if can_exit => {
                            self.ctx.configuration.borrow_mut().toggle();
                        }
                        _ => ConfigurationEventHandler::handle_key_event(
                            &mut self.ctx,
                            event,
                            &mut self.term,
                        ),
                    }
                    return;
                }

                // Route specific key event.
                match self.ctx.tab {
                    Tab::Selection => match self.ctx.selection_tab {
                        SelectionTab::Services | SelectionTab::Methods
                            if self.ctx.selection.borrow().selection_mode.clone()
                                == SelectionMode::ReflectionDialog =>
                        {
                            ReflectionDialogEventHandler::handle_key_event(
                                &mut self.ctx,
                                event,
                                &mut self.term,
                            );
                            if event.code == HELP_KEY && !self.ctx.disable_root_events {
                                Self::toggle_help(&mut self.ctx, ReflectionDialogEventHandler);
                            }
                        }
                        SelectionTab::Services => {
                            ServicesSelectionEventsHandler::handle_key_event(
                                &mut self.ctx,
                                event,
                                &mut self.term,
                            );
                            if event.code == HELP_KEY && !self.ctx.disable_root_events {
                                Self::toggle_help(&mut self.ctx, ServicesSelectionEventsHandler);
                            }
                        }
                        SelectionTab::Methods => {
                            MethodsSelectionEventsHandler::handle_key_event(
                                &mut self.ctx,
                                event,
                                &mut self.term,
                            );
                            if event.code == HELP_KEY && !self.ctx.disable_root_events {
                                Self::toggle_help(&mut self.ctx, MethodsSelectionEventsHandler);
                            }
                        }
                        SelectionTab::SearchServices => {
                            ServicesSearchEventsHandler::handle_key_event(
                                &mut self.ctx,
                                event,
                                &mut self.term,
                            );
                        }
                        SelectionTab::SearchMethods => {
                            MethodsSearchEventsHandler::handle_key_event(
                                &mut self.ctx,
                                event,
                                &mut self.term,
                            );
                        }
                    },
                    Tab::Headers => {
                        HeadersEventHandler::handle_key_event(&mut self.ctx, event, &mut self.term);
                        if event.code == HELP_KEY && !self.ctx.disable_root_events {
                            Self::toggle_help(&mut self.ctx, HeadersEventHandler);
                        }
                    }
                    Tab::Messages => match self.ctx.messages_tab {
                        MessagesTab::Request => {
                            RequestEventHandler::handle_key_event(
                                &mut self.ctx,
                                event,
                                &mut self.term,
                            );
                            if event.code == HELP_KEY && !self.ctx.disable_root_events {
                                Self::toggle_help(&mut self.ctx, RequestEventHandler);
                            }
                        }
                        MessagesTab::Response => {
                            ResponseEventHandler::handle_key_event(
                                &mut self.ctx,
                                event,
                                &mut self.term,
                            );
                            if event.code == HELP_KEY && !self.ctx.disable_root_events {
                                Self::toggle_help(&mut self.ctx, ResponseEventHandler);
                            }
                        }
                    },
                }
            }
        }

        // Dispatch a grpc request event in a separate thread.
        if self.ctx.messages.borrow().dispatch {
            let mut messages_model = self.ctx.messages.borrow_mut();
            let tls = messages_model.request.core_client.borrow().get_tls_config();
            messages_model.dispatch = false;
            match messages_model.get_request() {
                Ok(head) => {
                    let method = head.method_descriptor();
                    let is_client = method.is_client_streaming();
                    let is_server = method.is_server_streaming();

                    // Client- and bidi-streaming open an interactive session:
                    // the head is the first message, the rest are fed via the
                    // returned receiver as the user sends them.
                    let rx = if is_client {
                        let rx = messages_model.open_stream();
                        messages_model.push_stream_message(head.clone());
                        Some(rx)
                    } else {
                        None
                    };

                    let handler = tokio::spawn(async move {
                        let stream: Pin<Box<dyn Stream<Item = RequestResult> + Send>> =
                            match (rx, is_server) {
                                (Some(rx), true) => bidi_streaming(head, rx, tls).await,
                                (Some(rx), false) => once(client_streaming(head, rx, tls)).boxed(),
                                (None, true) => server_streaming(head, tls).await,
                                (None, false) => once(unary(head, tls)).boxed(),
                            };
                        drain_to(stream, &sx1).await;
                    });
                    messages_model.handler = Some(handler);
                }
                Err(err) => {
                    messages_model.response.set_text(&err.string());
                    messages_model.response.set_error(err);
                }
            }
        }

        // Dispatch a server reflection event in a separate thread.
        if self.ctx.reflection.borrow().dispatch_reflection {
            self.ctx.reflection.borrow_mut().handle_reflection(sx2);
        }
    }

    pub(crate) fn handle_crossterm_mouse_event(&mut self, event: MouseEvent) {
        if self.ctx.configuration.borrow().toggled() {
            ConfigurationEventHandler::handle_mouse_event(&mut self.ctx, event);
            return;
        }

        // Handle navbar tab area click to switch pages (only on left mouse down)
        if let MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column,
            row,
            ..
        } = event
        {
            let pos = ratatui::prelude::Position { x: column, y: row };
            if let Some(areas) = self.ctx.ui.borrow().navbar_tabs {
                if areas[0].contains(pos) {
                    self.ctx.tab = Tab::Selection;
                } else if areas[1].contains(pos) {
                    self.ctx.tab = Tab::Headers;
                } else if areas[2].contains(pos) {
                    self.ctx.tab = Tab::Messages;
                }
            }

            if let Some(areas) = self.ctx.ui.borrow().history_tabs {
                for (i, area) in areas.iter().enumerate() {
                    if area.contains(pos) {
                        let save_spot = i + 1; // Save spots are 1-indexed
                        self.ctx.history.borrow_mut().select(save_spot);
                        // Load the history for the selected save spot
                        self.ctx
                            .history
                            .borrow()
                            .load(&mut self.ctx.messages.borrow_mut());
                        return;
                    }
                }
            }
        }

        match self.ctx.tab {
            Tab::Selection => {
                ServicesSelectionEventsHandler::handle_mouse_event(&mut self.ctx, event);
                MethodsSelectionEventsHandler::handle_mouse_event(&mut self.ctx, event);
            }
            Tab::Headers => {
                if let MouseEvent {
                    kind: MouseEventKind::Down(MouseButton::Left),
                    ..
                } = event
                {
                    HeadersEventHandler::handle_mouse_event(&mut self.ctx, event);
                }
            }
            Tab::Messages => {
                // Hit-test: switch tabs on click in request/response areas
                if let MouseEvent {
                    kind: MouseEventKind::Down(MouseButton::Left),
                    column,
                    row,
                    ..
                } = event
                {
                    let pos = ratatui::prelude::Position { x: column, y: row };
                    let model_ref = self.ctx.messages.borrow();
                    if let Some(area) = model_ref.request.content_area {
                        if area.contains(pos) && self.ctx.messages_tab != MessagesTab::Request {
                            drop(model_ref);
                            self.ctx.messages_tab = MessagesTab::Request;
                            return;
                        }
                    }
                    if let Some(area) = model_ref.response.content_area {
                        if area.contains(pos) && self.ctx.messages_tab != MessagesTab::Response {
                            drop(model_ref);
                            self.ctx.messages_tab = MessagesTab::Response;
                            return;
                        }
                    }
                    drop(model_ref);
                }

                match self.ctx.messages_tab {
                    MessagesTab::Request => {
                        RequestEventHandler::handle_mouse_event(&mut self.ctx, event);
                    }
                    MessagesTab::Response => {
                        ResponseEventHandler::handle_mouse_event(&mut self.ctx, event);
                    }
                };
            }
        };
    }

    pub(crate) fn handle_crossterm_paste_event(&mut self, text: String) {
        if self.ctx.configuration.borrow().toggled() {
            ConfigurationEventHandler::handle_paste_event(&mut self.ctx, text);
            return;
        }

        match self.ctx.tab {
            Tab::Messages => {
                match self.ctx.messages_tab {
                    MessagesTab::Request => {
                        RequestEventHandler::handle_paste_event(&mut self.ctx, text);
                    }
                    MessagesTab::Response => {
                        ResponseEventHandler::handle_paste_event(&mut self.ctx, text);
                    }
                };
            }
            Tab::Headers => {
                HeadersEventHandler::handle_paste_event(&mut self.ctx, text);
            }
            Tab::Selection => {}
        }
    }

    pub(crate) fn handle_internal_event(&mut self, data: &InternalStreamData) {
        match data {
            InternalStreamData::Request(resp) => {
                resp.set(&mut self.ctx.messages.borrow_mut().response.editor);
            }
            InternalStreamData::Reflection(desc) => match desc {
                Ok(desc) => {
                    let d = desc.clone();
                    self.ctx.selection.borrow_mut().update_descriptor(d);
                    self.ctx.reflection.borrow_mut().error = None;
                    self.ctx.selection.borrow_mut().selection_mode = SelectionMode::Reflection;
                }
                Err(err) => {
                    self.ctx.reflection.borrow_mut().error = Some(err.clone());
                    Logger::critical(err);
                }
            },
            InternalStreamData::Done => {
                let mut messages = self.ctx.messages.borrow_mut();
                messages.handler = None;
                messages.clear_stream_session();
            }
        }
    }

    fn toggle_help<E>(ctx: &mut AppContext, _: E)
    where
        E: EventHandler<Context = AppContext>,
        E::Event: Display,
    {
        let key_mappings = E::format_event_mappings_as_strings(ctx);
        ctx.help = Some(HelpContext::new(key_mappings));
    }
}
