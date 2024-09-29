use std::{collections::HashMap, str::FromStr};

use http::Uri;
use prost::Message;
use prost_types::FileDescriptorProto;
use tokio_stream::{once, StreamExt};
use tonic::{transport::Channel, Request};
use tonic_reflection::pb::v1::{
    server_reflection_client::ServerReflectionClient, server_reflection_request::MessageRequest,
    server_reflection_response::MessageResponse, ServerReflectionRequest,
};

use crate::{descriptor::ReflectionRequest, error::Error};

/// # Errors
///
/// Fails if server reflection fails.
pub async fn make_file_by_symbol_reflection_request(
    request: &ReflectionRequest,
    containing_symbol: &str,
) -> Result<FileDescriptorProto, Error> {
    let message = MessageRequest::FileContainingSymbol(String::from(containing_symbol));
    make_file_reflection_request(request, message).await
}

/// # Errors
///
/// Fails if server reflection fails.
pub async fn make_file_by_filename_reflection_request(
    request: &ReflectionRequest,
    filename: &str,
) -> Result<FileDescriptorProto, Error> {
    let message = MessageRequest::FileByFilename(String::from(filename));
    make_file_reflection_request(request, message).await
}

/// # Errors
///
/// Fails if server reflection fails.
pub async fn make_file_reflection_request(
    request: &ReflectionRequest,
    message: MessageRequest,
) -> Result<FileDescriptorProto, Error> {
    let reflection_request = ServerReflectionRequest {
        host: request.host.clone(),
        message_request: Some(message),
    };
    let mut reflection_request = Request::new(once(reflection_request));

    // Address
    let uri = Uri::from_str(&request.host).map_err(|_| {
        Error::Internal(format!("Could not create uri from string {}", request.host))
    })?;

    // Metadata
    let metadata = request.metadata.clone();
    if let Some(meta) = metadata {
        *reflection_request.metadata_mut() = meta.inner;
    }

    // Build channel
    let builder = Channel::builder(uri);
    let channel = builder.connect_lazy();

    let mut client = ServerReflectionClient::new(channel);
    let mut inbound = client
        .server_reflection_info(reflection_request)
        .await?
        .into_inner();

    let response = inbound
        .next()
        .await
        .ok_or(Error::Internal("No streamed response".to_string()))??
        .message_response
        .ok_or(Error::Internal("No message response".to_string()))?;

    debug_assert!(inbound.next().await.is_none());

    let MessageResponse::FileDescriptorResponse(descriptor) = response else {
        let internal =
            Error::Internal("File descriptor reflection response is of incorrect type".to_string());
        return Err(internal);
    };

    let buf = descriptor
        .file_descriptor_proto
        .first()
        .ok_or(Error::Internal(
            "No file descriptor proto found".to_string(),
        ))?
        .as_ref();

    let file_descriptor = FileDescriptorProto::decode(buf)
        .map_err(|_| Error::Internal("Failed to decode".to_string()))?;

    Ok(file_descriptor)
}

/// # Errors
///
/// Fails if server reflection fails.
pub async fn make_list_service_reflection_request(
    request: &ReflectionRequest,
) -> Result<Vec<String>, Error> {
    let host = request.host.clone();

    let message_request = MessageRequest::ListServices(String::new());
    let reflection_request = ServerReflectionRequest {
        host: host.clone(),
        message_request: Some(message_request),
    };
    let mut reflection_request = Request::new(tokio_stream::once(reflection_request));

    // Address
    let uri = Uri::from_str(&host).map_err(|_| {
        Error::Internal(format!("Could not create uri from string {}", request.host))
    })?;

    // Metadata
    let metadata = request.metadata.clone();
    if let Some(meta) = metadata {
        *reflection_request.metadata_mut() = meta.inner;
    }

    // Build channel
    let builder = Channel::builder(uri);
    let channel = builder.connect_lazy();

    let mut client = ServerReflectionClient::new(channel);
    let mut inbound = client
        .server_reflection_info(reflection_request)
        .await?
        .into_inner();

    let response = inbound
        .next()
        .await
        .ok_or(Error::Internal("No streamed response".to_string()))??
        .message_response
        .ok_or(Error::Internal("No message response".to_string()))?;

    debug_assert!(inbound.next().await.is_none());

    let MessageResponse::ListServicesResponse(response) = response else {
        let internal =
            Error::Internal("List Service reflection response is of incorrect type".to_string());
        return Err(internal);
    };

    Ok(response.service.into_iter().map(|s| s.name).collect())
}

/// # Errors
///
/// Fails if server reflection fails.
#[allow(clippy::implicit_hasher)]
pub async fn handle_reflection_dependencies(
    request: &ReflectionRequest,
    file_descriptor: &FileDescriptorProto,
    file_descriptors: &mut HashMap<String, FileDescriptorProto>,
) -> Result<(), Error> {
    for dependency in file_descriptor.dependency.clone() {
        if file_descriptors.contains_key(&dependency) {
            continue;
        }

        let dep_descriptor = make_file_by_filename_reflection_request(request, &dependency).await?;

        file_descriptors.insert(dep_descriptor.name().to_string(), dep_descriptor.clone());

        // Recursively handle the dependencies
        Box::pin(handle_reflection_dependencies(
            request,
            &dep_descriptor,
            file_descriptors,
        ))
        .await?;
    }

    Ok(())
}
