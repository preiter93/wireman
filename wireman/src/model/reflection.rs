#![allow(clippy::module_name_repetitions)]
use super::core_client::CoreClient;
use super::headers::HeadersModel;
use super::SelectionModel;
use crate::events::InternalStreamData;
use core::descriptor::ReflectionRequest;
use core::ProtoDescriptor;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc::Sender;

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
    /// An error that occurred during reflection
    pub error: Option<String>,
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
            error: None,
        }
    }

    pub fn dispatch_reflection(&mut self) {
        self.dispatch_reflection = true;
    }

    pub fn handle_reflection(&mut self, sx: Sender<InternalStreamData>) {
        let request = self.build_request();
        self.dispatch_reflection = false;
        tokio::spawn(async move {
            let event = match ProtoDescriptor::from_reflection(request).await {
                Ok(desc) => InternalStreamData::Reflection(Ok(desc)),
                Err(err) => {
                    InternalStreamData::Reflection(Err(format!("Server reflection failed: {err}")))
                }
            };
            let _ = sx.send(event).await;
        });
    }

    // Builds the grpc request
    pub fn build_request(&mut self) -> ReflectionRequest {
        let headers = self.headers.borrow();

        // Address
        let address = headers.address();
        let mut req = ReflectionRequest::new(&address);

        // Metadata
        for (key, val) in headers.auth_headers_expanded() {
            if !key.is_empty() {
                let _ = req.insert_metadata(&key, &val);
            }
        }

        req
    }
}
