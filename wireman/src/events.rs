pub(crate) mod configuration;
pub(crate) mod headers;
pub(crate) mod messages;
pub(crate) mod selection;
use core::ProtoDescriptor;
use std::fmt::Display;
use std::pin::Pin;

use crate::app::App;
use crate::context::{AppContext, HelpContext, MessagesTab, SelectionTab, Tab};
use crate::model::messages::{server_streaming, unary, RequestResult};
use crate::model::selection::SelectionMode;
use configuration::ConfigurationEventHandler;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use event_handler::EventHandler;
use futures::{Stream, StreamExt};
use logger::Logger;
pub(crate) use selection::methods::MethodsSelectionEventsHandler;
pub(crate) use selection::methods_search::MethodsSearchEventsHandler;
use selection::reflection::ReflectionDialogEventHandler;
pub(crate) use selection::services::ServicesSelectionEventsHandler;
pub(crate) use selection::services_search::ServicesSearchEventsHandler;
use tokio::sync::mpsc::{self, Receiver, Sender};

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
                    match event.code {
                        KeyCode::Char('e') if event.modifiers == KeyModifiers::CONTROL => {
                            self.ctx.configuration.borrow_mut().toggle();
                        }
                        _ => ConfigurationEventHandler::handle_key_event(&mut self.ctx, event),
                    }
                    return;
                }

                // Route specifc key event.
                match self.ctx.tab {
                    Tab::Selection => match self.ctx.selection_tab {
                        SelectionTab::Services | SelectionTab::Methods
                            if self.ctx.selection.borrow().selection_mode.clone()
                                == SelectionMode::ReflectionDialog =>
                        {
                            ReflectionDialogEventHandler::handle_key_event(&mut self.ctx, event);
                            if event.code == HELP_KEY && !self.ctx.disable_root_events {
                                Self::toggle_help(&mut self.ctx, ReflectionDialogEventHandler);
                            }
                        }
                        SelectionTab::Services => {
                            ServicesSelectionEventsHandler::handle_key_event(&mut self.ctx, event);
                            if event.code == HELP_KEY && !self.ctx.disable_root_events {
                                Self::toggle_help(&mut self.ctx, ServicesSelectionEventsHandler);
                            }
                        }
                        SelectionTab::Methods => {
                            MethodsSelectionEventsHandler::handle_key_event(&mut self.ctx, event);
                            if event.code == HELP_KEY && !self.ctx.disable_root_events {
                                Self::toggle_help(&mut self.ctx, MethodsSelectionEventsHandler);
                            }
                        }
                        SelectionTab::SearchServices => {
                            ServicesSearchEventsHandler::handle_key_event(&mut self.ctx, event);
                        }
                        SelectionTab::SearchMethods => {
                            MethodsSearchEventsHandler::handle_key_event(&mut self.ctx, event);
                        }
                    },
                    Tab::Headers => {
                        HeadersEventHandler::handle_key_event(&mut self.ctx, event);
                        if event.code == HELP_KEY && !self.ctx.disable_root_events {
                            Self::toggle_help(&mut self.ctx, HeadersEventHandler);
                        }
                    }
                    Tab::Messages => match self.ctx.messages_tab {
                        MessagesTab::Request => {
                            RequestEventHandler::handle_key_event(&mut self.ctx, event);
                            if event.code == HELP_KEY && !self.ctx.disable_root_events {
                                Self::toggle_help(&mut self.ctx, RequestEventHandler);
                            }
                        }
                        MessagesTab::Response => {
                            ResponseEventHandler::handle_key_event(&mut self.ctx, event);
                            if event.code == HELP_KEY && !self.ctx.disable_root_events {
                                Self::toggle_help(&mut self.ctx, ResponseEventHandler);
                            }
                        }
                    },
                }
            }
        }

        // Dispatch a grpc request event in a seperate thread.
        if self.ctx.messages.borrow().dispatch {
            let mut messages_model = self.ctx.messages.borrow_mut();
            let tls = messages_model.request.core_client.borrow().get_tls_config();
            messages_model.dispatch = false;
            match messages_model.get_request() {
                Ok(req) => {
                    let handler = tokio::spawn(async move {
                        let is_server_streaming = req.method_descriptor().is_server_streaming();
                        if is_server_streaming {
                            let mut stream: Pin<Box<dyn Stream<Item = RequestResult> + Send>> =
                                Box::pin(server_streaming(req, tls).await);
                            while let Some(resp) = stream.next().await {
                                let _ = sx1.send(InternalStreamData::Request(resp)).await;
                            }
                            let _ = sx1.send(InternalStreamData::Done).await;
                        } else {
                            let resp = unary(req, tls).await;
                            let _ = sx1.send(InternalStreamData::Request(resp)).await;
                            let _ = sx1.send(InternalStreamData::Done).await;
                        }
                    });

                    messages_model.handler = Some(handler);
                }
                Err(err) => {
                    messages_model.response.set_text(&err.string());
                    messages_model.response.set_error(err);
                }
            }
        }

        // Dispatch a server reflection event in a seperate thread.
        if self.ctx.reflection.borrow().dispatch_reflection {
            self.ctx.reflection.borrow_mut().handle_reflection(sx2);
        }
    }

    pub(crate) fn handle_crossterm_mouse_event(&mut self, event: MouseEvent) {
        if self.ctx.tab == Tab::Messages {
            match self.ctx.messages_tab {
                MessagesTab::Request => {
                    RequestEventHandler::handle_mouse_event(&mut self.ctx, event);
                }
                MessagesTab::Response => {
                    ResponseEventHandler::handle_mouse_event(&mut self.ctx, event);
                }
            };
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
                self.ctx.messages.borrow_mut().handler = None;
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
