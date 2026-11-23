use std::process::Command;

/// The prefixes of an authentication header in front of its value.
const AUTH_PREFIXES: [&str; 2] = ["Bearer ", "Basic "];

/// Runs a `$(command)` header and returns its output, or the header unchanged.
#[must_use]
pub fn expand_header(raw: &str) -> String {
    if is_command(raw) {
        let command = &raw[2..raw.len() - 1];
        if let Some(output) = execute_command(command) {
            return output;
        };
    }
    raw.to_string()
}

/// Runs the `$(command)` part of a `Bearer` or `Basic` authentication header
/// and returns the header with its output.
#[must_use]
pub fn expand_auth_header(raw: &str) -> String {
    for prefix in AUTH_PREFIXES {
        if let Some(value) = raw.strip_prefix(prefix) {
            return format!("{prefix}{}", expand_header(value));
        }
    }
    expand_header(raw)
}

/// Returns whether a header, optionally with a `Bearer` or `Basic` prefix,
/// is a `$(command)` that runs when the request is sent.
#[must_use]
pub fn is_auth_command(raw: &str) -> bool {
    let value = AUTH_PREFIXES
        .iter()
        .find_map(|prefix| raw.strip_prefix(prefix))
        .unwrap_or(raw);
    is_command(value)
}

fn is_command(raw: &str) -> bool {
    raw.starts_with("$(") && raw.ends_with(')')
}

fn execute_command(command: &str) -> Option<String> {
    let mut parts = command.split_whitespace();
    let program = parts.next()?;
    let args: Vec<&str> = parts.collect();
    let output = Command::new(program).args(args).output();

    match output {
        Ok(output) if output.status.success() => {
            if let Ok(mut stdout) = String::from_utf8(output.stdout) {
                stdout.pop(); // Remove the newline character at the end
                return Some(stdout);
            }
        }
        _ => {}
    };
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_header() {
        struct TestCase {
            name: &'static str,
            given_value: &'static str,
            expected_value: &'static str,
        }

        let test_cases = vec![
            TestCase {
                name: "success: plain value unchanged",
                given_value: "Bearer token",
                expected_value: "Bearer token",
            },
            TestCase {
                name: "success: expand echo command",
                given_value: "$(echo Bearer token)",
                expected_value: "Bearer token",
            },
            TestCase {
                name: "failure: unknown command returns raw value",
                given_value: "$(wireman-unknown-command)",
                expected_value: "$(wireman-unknown-command)",
            },
        ];

        for test_case in test_cases {
            let value = expand_header(test_case.given_value);

            assert_eq!(value, test_case.expected_value, "{}", test_case.name);
        }
    }

    #[test]
    fn test_expand_auth_header() {
        struct TestCase {
            name: &'static str,
            given_value: &'static str,
            expected_value: &'static str,
        }

        let test_cases = vec![
            TestCase {
                name: "success: expand bearer command",
                given_value: "Bearer $(echo token)",
                expected_value: "Bearer token",
            },
            TestCase {
                name: "success: basic without command",
                given_value: "Basic dXNlcjpwYXNz",
                expected_value: "Basic dXNlcjpwYXNz",
            },
        ];

        for test_case in test_cases {
            let value = expand_auth_header(test_case.given_value);

            assert_eq!(value, test_case.expected_value, "{}", test_case.name);
        }
    }
}
