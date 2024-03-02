pub(crate) mod headers;
pub(crate) mod messages;
pub(crate) mod selection;
use crate::app::App;
use crate::context::{MessagesTab, SelectionTab, Tab};
use crate::model::messages::{do_request, RequestResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
pub(crate) use selection::methods::MethodsSelectionEventHandler;
pub(crate) use selection::methods_search::MethodsSearchEventHandler;
pub(crate) use selection::services::ServicesSelectionEventHandler;
pub(crate) use selection::services_search::ServicesSearchEventHandler;
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

impl InternalStream {
    pub(crate) fn new() -> Self {
        let (sx, rx) = mpsc::channel::<RequestResult>(10);
        Self { sx, rx }
    }
}

impl App {
    pub(crate) fn handle_crossterm_event(&mut self, event: KeyEvent) {
        let sx = self.internal_stream.sx.clone();
        match event.code {
            KeyCode::Char('q') if !self.ctx.disable_root_events => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if event.modifiers == KeyModifiers::CONTROL => {
                self.should_quit = true;
            }
            _ => match self.ctx.tab {
                Tab::Selection => match self.ctx.selection_tab {
                    SelectionTab::Services => {
                        ServicesSelectionEventHandler::handle_key_event(&mut self.ctx, event);
                    }
                    SelectionTab::Methods => {
                        MethodsSelectionEventHandler::handle_key_event(&mut self.ctx, event);
                    }
                    SelectionTab::SearchServices => {
                        ServicesSearchEventHandler::handle_key_event(&mut self.ctx, event);
                    }
                    SelectionTab::SearchMethods => {
                        MethodsSearchEventHandler::handle_key_event(&mut self.ctx, event);
                    }
                },
                Tab::Headers => {
                    HeadersEventHandler::handle_key_event(&mut self.ctx, event);
                }
                Tab::Messages => match self.ctx.messages_tab {
                    MessagesTab::Request => {
                        RequestEventHandler::handle_key_event(&mut self.ctx, event);
                    }
                    MessagesTab::Response => {
                        ResponseEventHandler::handle_key_event(&mut self.ctx, event);
                    }
                },
            },
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

    pub(crate) fn handle_internal_event(&mut self, result: &RequestResult) {
        result.set(&mut self.ctx.messages.borrow_mut().response.editor);
        self.ctx.messages.borrow_mut().handler.take();
    }
}
