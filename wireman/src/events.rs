pub(crate) mod headers;
pub(crate) mod messages;
pub(crate) mod selection;
use std::fmt::Display;

use crate::app::App;
use crate::context::{AppContext, HelpContext, MessagesTab, SelectionTab, Tab};
use crate::model::messages::{do_request, RequestResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
pub(crate) use selection::methods::MethodsSelectionEventsHandler;
pub(crate) use selection::methods_search::MethodsSearchEventsHandler;
pub(crate) use selection::services::ServicesSelectionEventsHandler;
pub(crate) use selection::services_search::ServicesSearchEventsHandler;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tui_key_event_handler::EventHandler;

use self::headers::HeadersEventHandler;
use self::messages::request::RequestEventHandler;
use self::messages::response::ResponseEventHandler;

type InternalStreamData = RequestResult;
pub(crate) struct InternalStream {
    pub(crate) sx: Sender<InternalStreamData>,
    pub(crate) rx: Receiver<InternalStreamData>,
}

const HELP_KEY: KeyCode = KeyCode::Char('?');

impl InternalStream {
    pub(crate) fn new() -> Self {
        let (sx, rx) = mpsc::channel::<RequestResult>(10);
        Self { sx, rx }
    }
}

impl App {
    pub(crate) fn handle_crossterm_key_event(&mut self, event: KeyEvent) {
        let sx = self.internal_stream.sx.clone();
        match event.code {
            KeyCode::Char('c') if event.modifiers == KeyModifiers::CONTROL => {
                self.should_quit = true;
            }
            _ => {
                //  Help modal dialog key events.
                if self.ctx.help.is_some() {
                    match event.code {
                        KeyCode::Esc | HELP_KEY => {
                            self.ctx.help = None;
                        }
                        _ => (),
                    }
                    return;
                }
                // Route specifc key event.
                match self.ctx.tab {
                    Tab::Selection => match self.ctx.selection_tab {
                        SelectionTab::Services => {
                            ServicesSelectionEventsHandler::handle_key_event(&mut self.ctx, event);
                            if event.code == HELP_KEY {
                                Self::toggle_help(&mut self.ctx, ServicesSelectionEventsHandler);
                            }
                        }
                        SelectionTab::Methods => {
                            MethodsSelectionEventsHandler::handle_key_event(&mut self.ctx, event);
                            if event.code == HELP_KEY {
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
                        if event.code == HELP_KEY {
                            Self::toggle_help(&mut self.ctx, HeadersEventHandler);
                        }
                    }
                    Tab::Messages => match self.ctx.messages_tab {
                        MessagesTab::Request => {
                            RequestEventHandler::handle_key_event(&mut self.ctx, event);
                            if event.code == HELP_KEY {
                                Self::toggle_help(&mut self.ctx, RequestEventHandler);
                            }
                        }
                        MessagesTab::Response => {
                            ResponseEventHandler::handle_key_event(&mut self.ctx, event);
                            if event.code == HELP_KEY {
                                Self::toggle_help(&mut self.ctx, ResponseEventHandler);
                            }
                        }
                    },
                }
            }
        }

        // Dispatch the grpc request in a seperate thread.
        if self.ctx.messages.borrow().dispatch {
            let mut messages_model = self.ctx.messages.borrow_mut();
            messages_model.dispatch = false;
            match messages_model.collect_request() {
                Ok(req) => {
                    let handler = tokio::spawn(async move {
                        let resp = do_request(req).await;
                        let _ = sx.send(resp).await;
                    });
                    messages_model.handler = Some(handler);
                }
                Err(err) => {
                    messages_model.response.set_text(&err.string());
                    messages_model.response.set_error(err);
                }
            }
        }
    }

    pub(crate) fn handle_crossterm_mouse_event(&mut self, event: MouseEvent) {
        if self.ctx.tab == Tab::Messages {
            match self.ctx.messages_tab {
                MessagesTab::Request => {
                    RequestEventHandler::handle_mouse_event(&mut self.ctx, event)
                }
                MessagesTab::Response => {
                    ResponseEventHandler::handle_mouse_event(&mut self.ctx, event)
                }
            };
        }
    }

    pub(crate) fn handle_internal_event(&mut self, result: &RequestResult) {
        result.set(&mut self.ctx.messages.borrow_mut().response.editor);
        self.ctx.messages.borrow_mut().handler.take();
    }

    fn toggle_help<E>(ctx: &mut AppContext, _: E)
    where
        E: EventHandler<Context = AppContext>,
        E::Event: Display,
    {
        let key_mappings = E::format_event_mappings_as_strings(ctx);
        ctx.help = Some(HelpContext::new(key_mappings))
    }
}
