#![allow(clippy::module_name_repetitions)]
use core::ProtoDescriptor;
use logger::Logger;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc::Sender;

use crate::events::InternalStreamData;

use super::core_client::CoreClient;
use super::headers::HeadersModel;
use super::SelectionModel;

#[derive(Clone)]
pub struct ReflectionModel {
    /// Core client retrieves proto descriptors
    core_client: Rc<RefCell<CoreClient>>,
    /// A reference to the headers model
    pub headers: Rc<RefCell<HeadersModel>>,
    /// A reference to the selection model
    pub selection: Rc<RefCell<SelectionModel>>,
    /// Dispatch a reflection event
    pub dispatch_reflection: bool,
}

impl ReflectionModel {
    /// Instantiates a [`ReflectionModel`]. Requires the core client to
    /// retrieve the proto services and methods.
    pub fn new(
        core_client: Rc<RefCell<CoreClient>>,
        headers: Rc<RefCell<HeadersModel>>,
        selection: Rc<RefCell<SelectionModel>>,
    ) -> Self {
        Self {
            core_client,
            headers,
            selection,
            dispatch_reflection: false,
        }
    }

    pub fn dispatch_reflection(&mut self) {
        Logger::debug("boom toggle");
        self.dispatch_reflection = true;
    }

    pub fn handle_reflection(&mut self, sx: Sender<InternalStreamData>) {
        let host = self.headers.borrow().address();
        self.dispatch_reflection = false;
        tokio::spawn(async move {
            Logger::debug("spawn");
            match ProtoDescriptor::reflect(&host).await {
                Ok(desc) => {
                    let _ = sx.send(InternalStreamData::Reflection(Ok(desc))).await;
                }
                Err(err) => {
                    let err_str = format!("Server reflection failed: {err}");
                    let _ = sx
                        .send(InternalStreamData::Reflection(Err(err_str.clone())))
                        .await;
                    Logger::critical(err_str)
                }
            }
        });
    }
}
