use std::{collections::HashMap, str::FromStr};

use http::Uri;
use prost::Message;
use prost_types::FileDescriptorProto;
use tokio_stream::StreamExt;
use tonic::{transport::Channel, Request};
use tonic_reflection::pb::v1::{
    server_reflection_client::ServerReflectionClient, server_reflection_request::MessageRequest,
    server_reflection_response::MessageResponse, ServerReflectionRequest,
};

use crate::error::Error;

pub async fn make_file_by_symbol_reflection_request(
    host: &str,
    containing_symbol: &str,
) -> Result<FileDescriptorProto, Error> {
    let request = MessageRequest::FileContainingSymbol(String::from(containing_symbol));
    make_file_reflection_request(host, request).await
}

pub async fn make_file_by_filename_reflection_request(
    host: &str,
    filename: &str,
) -> Result<FileDescriptorProto, Error> {
    let request = MessageRequest::FileByFilename(String::from(filename));
    make_file_reflection_request(host, request).await
}

pub async fn make_file_reflection_request(
    host: &str,
    request: MessageRequest,
) -> Result<FileDescriptorProto, Error> {
    let request = ServerReflectionRequest {
        host: host.to_string(),
        message_request: Some(request),
    };
    let request = Request::new(tokio_stream::once(request));

    let uri = Uri::from_str(host).unwrap();
    let builder = Channel::builder(uri);
    let channel = builder.connect_lazy();

    let mut client = ServerReflectionClient::new(channel);
    let mut inbound = client.server_reflection_info(request).await?.into_inner();

    let response = inbound
        .next()
        .await
        .ok_or(Error::Internal("No streamed response".to_string()))??
        .message_response
        .ok_or(Error::Internal("No message response".to_string()))?;

    assert!(inbound.next().await.is_none());

    let MessageResponse::FileDescriptorResponse(descriptor) = response else {
        let internal =
            Error::Internal("File descriptor reflection response is of incorrect type".to_string());
        return Err(internal);
    };

    let buf = descriptor.file_descriptor_proto.first().unwrap().as_ref();

    let file_descriptor = FileDescriptorProto::decode(buf).expect("Failed to decode");

    Ok(file_descriptor)
}

pub async fn make_list_service_reflection_request(host: &str) -> Result<Vec<String>, Error> {
    let message_request = MessageRequest::ListServices(String::new());
    let request = ServerReflectionRequest {
        host: host.to_string(),
        message_request: Some(message_request),
    };
    let request = Request::new(tokio_stream::once(request));

    let uri = Uri::from_str(host).unwrap();
    let builder = Channel::builder(uri);
    let channel = builder.connect_lazy();

    let mut client = ServerReflectionClient::new(channel);
    let mut inbound = client.server_reflection_info(request).await?.into_inner();

    let response = inbound
        .next()
        .await
        .ok_or(Error::Internal("No streamed response".to_string()))??
        .message_response
        .ok_or(Error::Internal("No message response".to_string()))?;

    assert!(inbound.next().await.is_none());

    let MessageResponse::ListServicesResponse(response) = response else {
        let internal =
            Error::Internal("List Service reflection response is of incorrect type".to_string());
        return Err(internal);
    };

    Ok(response.service.into_iter().map(|s| s.name).collect())
}

pub async fn handle_reflection_dependencies(
    host: &str,
    file_descriptor: &FileDescriptorProto,
    file_descriptors: &mut HashMap<String, FileDescriptorProto>,
) -> Result<(), Error> {
    for dependency in file_descriptor.dependency.clone() {
        if file_descriptors.contains_key(&dependency) {
            continue;
        }

        let dep_descriptor = make_file_by_filename_reflection_request(host, &dependency).await?;

        file_descriptors.insert(dep_descriptor.name().to_string(), dep_descriptor.clone());

        // Recursively handle the dependencies
        Box::pin(handle_reflection_dependencies(
            host,
            &dep_descriptor,
            file_descriptors,
        ))
        .await?;
    }

    Ok(())
}
