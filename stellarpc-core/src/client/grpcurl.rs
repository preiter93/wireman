use std::collections::HashMap;

use http::Uri;
use prost_reflect::MethodDescriptor;

use crate::Config;

/// Returns the grpc request as `grpcurl` command
///
/// # Errors
/// - Serialize message to json
pub fn request_as_grpcurl<T: Into<Uri>>(
    cfg: &Config,
    uri: T,
    message: &str,
    method_desc: &MethodDescriptor,
    metadata: HashMap<String, String>,
) -> crate::error::Result<String> {
    // The include paths
    let import = &cfg.workspace;

    // The name of the proto file
    let file_desc = method_desc.parent_file();
    let proto = file_desc.file_descriptor_proto().name();

    // The host
    let uri = uri.into();
    let host = uri.host().unwrap_or("");
    let port = uri.port_u16().unwrap_or(80);

    // The method name
    let method = method_desc.full_name();

    // The metadata if available
    let metadata = metadata
        .iter()
        .map(|(key, val)| format!(" -H \"{}: {}\"", key, val))
        .collect::<Vec<_>>()
        .join("");

    let cmd = format!(
        "grpcurl -d @ -import-path {} -proto {}{} -plaintext {}:{} {} <<EOM\n{}\nEOM",
        import, proto, metadata, host, port, method, message
    );
    Ok(cmd)
}

#[cfg(test)]
mod test {
    use crate::descriptor::RequestMessage;
    use crate::{client::tls::TlsConfig, ProtoDescriptor};

    use super::*;

    #[test]
    fn test_request_as_grpcurl() {
        // given
        let given_cfg = Config {
            workspace: "/Users/myworkspace".to_string(),
            files: vec!["test_files/test.proto".to_string()],
            tls: TlsConfig::default(),
            address: String::new(),
        };
        let given_uri = Uri::from_static("http://localhost:50051");
        let test_message = load_test_message("Simple");
        let given_method = test_message.method_descriptor();
        let given_message = "{\n  \"number\": 0\n}";
        let expected = "grpcurl -d @ -import-path /Users/myworkspace -proto test_files/test.proto -plaintext localhost:50051 proto.TestService.Simple <<EOM\n{\n  \"number\": 0\n}\nEOM";

        // when
        let cmd = request_as_grpcurl(
            &given_cfg,
            given_uri,
            given_message,
            &given_method,
            HashMap::new(),
        )
        .unwrap();

        // then
        assert_eq!(cmd, expected);
    }

    fn load_test_message(method: &str) -> RequestMessage {
        let files = vec!["test_files/test.proto"];
        let includes = vec!["."];

        let desc = ProtoDescriptor::from_files(files, includes).unwrap();

        let method = desc
            .get_method_by_name("proto.TestService", method)
            .unwrap();
        let request = method.input();
        RequestMessage::new(request, method)
    }
}
