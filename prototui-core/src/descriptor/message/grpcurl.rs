use crate::{descriptor::RequestMessage, ProtoTuiConfig};
use http::Uri;

/// Returns the grpc request as `grpcurl` command
pub fn request_as_grpcurl<T: Into<Uri>>(
    cfg: &ProtoTuiConfig,
    uri: T,
    req: &RequestMessage,
) -> String {
    // The include paths
    let import = &cfg.workspace;

    // The name of the proto file
    let file_desc = req.method_descriptor().parent_file();
    let proto = file_desc.file_descriptor_proto().name();

    // The host address
    let host: String = uri.into().to_string();

    // The method name
    let method_desc = req.method_descriptor();
    let method = method_desc.full_name();

    // The request message
    let data = req.message.to_json().unwrap();
    let parsed = serde_json::from_str::<serde_json::Value>(&data).unwrap();
    let json_data = serde_json::to_string_pretty(&parsed).unwrap();

    let cmd = format!(
        "grpcurl -d @ -import-path {} -proto {} -plaintext {} {} <<EOM\n{}\nEOM",
        import, proto, host, method, json_data
    );
    cmd
}

#[cfg(test)]
mod test {
    use crate::{client::tls::TlsConfig, ProtoDescriptor};

    use super::*;

    #[test]
    fn test_request_as_grpcurl() {
        // given
        let given_cfg = ProtoTuiConfig {
            workspace: "/Users/myworkspace".to_string(),
            files: vec!["test_files/test.proto".to_string()],
            tls: TlsConfig::default(),
            address: String::new(),
        };
        let given_uri = Uri::from_static("localhost:50051");
        let given_req = load_test_message("Simple");
        let expected = "grpcurl -d @ -import-path /Users/myworkspace -proto test_files/test.proto -plaintext localhost:50051 proto.TestService.Simple <<EOM\n{\n  \"number\": 0\n}\nEOM";

        // when
        let cmd = request_as_grpcurl(&given_cfg, given_uri, &given_req);

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
