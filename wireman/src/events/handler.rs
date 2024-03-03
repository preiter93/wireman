use super::{
    headers::HeadersEventHandler,
    messages::{request::RequestEventHandler, response::ResponseEventHandler},
    selection::{
        methods::MethodsSelectionEventsHandler, methods_search::MethodsSearchEventsHandler,
        services::ServicesSelectionEventsHandler, services_search::ServicesSearchEventsHandler,
    },
};
use crate::context::{AppContext, MessagesTab, SelectionTab, Tab};
use std::collections::HashMap;
use tui_key_event_handler::{EventHandler, KeyEvent};

pub enum AppEventHandler {
    ServicesSelection(ServicesSelectionEventsHandler),
    ServicesSearch(ServicesSearchEventsHandler),
    MethodsSelection(MethodsSelectionEventsHandler),
    MethodsSearch(MethodsSearchEventsHandler),
    Headers(HeadersEventHandler),
    Request(RequestEventHandler),
    Response(ResponseEventHandler),
}

impl AppEventHandler {
    pub(crate) fn from_ctx(ctx: &AppContext) -> Self {
        match ctx.tab {
            Tab::Selection => match ctx.selection_tab {
                SelectionTab::Services => Self::ServicesSelection(ServicesSelectionEventsHandler),
                SelectionTab::Methods => Self::MethodsSelection(MethodsSelectionEventsHandler),
                SelectionTab::SearchServices => Self::ServicesSearch(ServicesSearchEventsHandler),
                SelectionTab::SearchMethods => Self::MethodsSearch(MethodsSearchEventsHandler),
            },
            Tab::Headers => Self::Headers(HeadersEventHandler),
            Tab::Messages => match ctx.messages_tab {
                MessagesTab::Request => Self::Request(RequestEventHandler),
                MessagesTab::Response => Self::Response(ResponseEventHandler),
            },
        }
    }

    pub(crate) fn handle_event(&self, event: KeyEvent, ctx: &mut AppContext) {
        match self {
            AppEventHandler::ServicesSelection(inner) => inner.handle_key_event(event, ctx),
            AppEventHandler::ServicesSearch(inner) => inner.handle_key_event(event, ctx),
            AppEventHandler::MethodsSelection(inner) => inner.handle_key_event(event, ctx),
            AppEventHandler::MethodsSearch(inner) => inner.handle_key_event(event, ctx),
            AppEventHandler::Headers(inner) => inner.handle_key_event(event, ctx),
            AppEventHandler::Request(inner) => inner.handle_key_event(event, ctx),
            AppEventHandler::Response(inner) => inner.handle_key_event(event, ctx),
        }
    }

    pub(crate) fn get_key_mappings(ctx: &AppContext) -> HashMap<String, String> {
        let slf = Self::from_ctx(ctx);
        match slf {
            AppEventHandler::ServicesSelection(inner) => inner
                .key_event_mappings(ctx)
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            AppEventHandler::ServicesSearch(inner) => inner
                .key_event_mappings(ctx)
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            AppEventHandler::MethodsSelection(inner) => inner
                .key_event_mappings(ctx)
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            AppEventHandler::MethodsSearch(inner) => inner
                .key_event_mappings(ctx)
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            AppEventHandler::Headers(inner) => inner
                .key_event_mappings(ctx)
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            AppEventHandler::Request(inner) => inner
                .key_event_mappings(ctx)
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            AppEventHandler::Response(inner) => inner
                .key_event_mappings(ctx)
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}
