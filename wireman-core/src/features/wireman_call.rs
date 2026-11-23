use prost_reflect::MethodDescriptor;
use std::collections::HashMap;
use std::fmt::Write;

/// Builds a `wireman call` command that sends the same request.
///
/// Header values like `$(getToken.sh)` stay unexpanded, so the command can be
/// shared and whoever runs it uses their own token.
#[allow(clippy::implicit_hasher)]
#[must_use]
pub fn wireman_call(
    method: &MethodDescriptor,
    message: &str,
    headers: &HashMap<String, String>,
    address: &str,
) -> String {
    let mut command = format!("wireman call {}", method.full_name());

    if !address.is_empty() {
        let _ = write!(command, " -a {}", quote(address));
    }

    let mut headers: Vec<_> = headers.iter().collect();
    headers.sort();
    for (key, value) in headers {
        let _ = write!(command, " -H {}", quote(&format!("{key}: {value}")));
    }

    let message = serde_json::from_str::<serde_json::Value>(message)
        .map_or_else(|_| message.trim().to_string(), |json| json.to_string());
    let _ = write!(command, " -d {}", quote(&message));

    command
}

/// Wraps a value in single quotes, so the shell passes it on unchanged.
fn quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', r"'\''"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProtoDescriptor;

    #[test]
    fn test_wireman_call() {
        struct TestCase {
            name: &'static str,
            given_message: &'static str,
            given_headers: Vec<(&'static str, &'static str)>,
            expected_command: &'static str,
        }

        let test_cases = vec![
            TestCase {
                name: "success: no headers",
                given_message: "{\"number\":5}",
                given_headers: vec![],
                expected_command: "wireman call proto.TestService.Simple -a 'http://localhost:50051' -d '{\"number\":5}'",
            },
            TestCase {
                name: "success: headers keep commands unexpanded",
                given_message: "{}",
                given_headers: vec![
                    ("x-trace", "abc"),
                    ("authorization", "Bearer $(getToken.sh)"),
                ],
                expected_command: "wireman call proto.TestService.Simple -a 'http://localhost:50051' -H 'authorization: Bearer $(getToken.sh)' -H 'x-trace: abc' -d '{}'",
            },
            TestCase {
                name: "success: single quote in message is escaped",
                given_message: "{\"text\":\"it's\"}",
                given_headers: vec![],
                expected_command: "wireman call proto.TestService.Simple -a 'http://localhost:50051' -d '{\"text\":\"it'\\''s\"}'",
            },
            TestCase {
                name: "success: pretty JSON is made compact",
                given_message: "{\n  \"number\": 5\n}",
                given_headers: vec![],
                expected_command: "wireman call proto.TestService.Simple -a 'http://localhost:50051' -d '{\"number\":5}'",
            },
        ];

        let descriptor = ProtoDescriptor::new(vec!["."], vec!["test_files/test.proto"]).unwrap();
        let method = descriptor
            .get_method_by_name("proto.TestService", "Simple")
            .unwrap();

        for test_case in test_cases {
            let headers = test_case
                .given_headers
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect();

            let command = wireman_call(
                &method,
                test_case.given_message,
                &headers,
                "http://localhost:50051",
            );

            assert_eq!(command, test_case.expected_command, "{}", test_case.name);
        }
    }
}
