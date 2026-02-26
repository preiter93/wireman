use std::collections::HashMap;
use std::fmt::Write;

use http::Uri;
use prost_reflect::MethodDescriptor;

/// Generate a `grpcurl` command as a string for sending a `gRPC` request.
///
/// This function constructs a `grpcurl` command that can be used to send a `gRPC` request
/// to a specified `gRPC` server. The generated command includes information such as
/// URI, request message in JSON format, method descriptor, and metadata headers.
///
/// # Parameters
///
/// - `uri`: The address URI of the `gRPC` server (e.g., "localhost:50051").
/// - `message`: The request data in JSON format.
/// - `method_desc`: The method descriptor for the `gRPC` method.
/// - `metadata`: Key-value metadata headers to be included in the request.
#[allow(clippy::implicit_hasher)]
pub fn grpcurl<T: Into<Uri>>(
    _includes: &[String],
    uri: T,
    message: &str,
    method_desc: &MethodDescriptor,
    metadata: &HashMap<String, String>,
) -> String {
    // The host
    let uri = uri.into();
    let host = uri.host().unwrap_or("");
    let port = uri.port_u16().unwrap_or(80);

    // The method name
    let method = method_desc.full_name();

    // The metadata if available
    let metadata = metadata
        .iter()
        .fold(String::new(), |mut result, (key, val)| {
            let _ = write!(result, " -H \"{key}: {val}\"");
            result
        });

    format!("grpcurl -d @ -plaintext{metadata} {host}:{port} {method} <<EOM\n{message}\nEOM")
}

#[cfg(test)]
mod test {
    use crate::descriptor::RequestMessage;
    use crate::ProtoDescriptor;

    use super::*;

    #[test]
    fn test_request_as_grpcurl() {
        // given
        let includes = vec!["/Users/myworkspace".to_string()];
        let given_uri = Uri::from_static("http://localhost:50051");
        let test_message = load_test_message("Simple");
        let given_method = test_message.method_descriptor();
        let given_message = "{\n  \"number\": 0\n}";
        let expected = "grpcurl -d @ -plaintext localhost:50051 proto.TestService.Simple <<EOM\n{\n  \"number\": 0\n}\nEOM";

        // when
        let cmd = grpcurl(
            &includes,
            given_uri,
            given_message,
            &given_method,
            &HashMap::new(),
        );

        // then
        assert_eq!(cmd, expected);
    }

    #[test]
    fn test_request_as_grpcurl_with_metadata() {
        // given
        let includes = vec!["/Users/myworkspace".to_string()];
        let given_uri = Uri::from_static("http://localhost:50051");
        let test_message = load_test_message("Simple");
        let given_method = test_message.method_descriptor();
        let given_message = "{\n  \"number\": 0\n}";
        let mut metadata = HashMap::new();
        metadata.insert("authorization".to_string(), "Bearer $TOKEN".to_string());

        // when
        let cmd = grpcurl(
            &includes,
            given_uri,
            given_message,
            &given_method,
            &metadata,
        );

        // then
        assert!(cmd.contains("-H \"authorization: Bearer $TOKEN\""));
        assert!(cmd.contains("localhost:50051"));
        assert!(cmd.contains("proto.TestService.Simple"));
    }

    fn load_test_message(method: &str) -> RequestMessage {
        let files = vec!["test_files/test.proto"];
        let includes = vec!["."];

        let desc = ProtoDescriptor::new(includes, files).unwrap();

        let method = desc
            .get_method_by_name("proto.TestService", method)
            .unwrap();
        let request = method.input();
        RequestMessage::new(request, method)
    }
}
