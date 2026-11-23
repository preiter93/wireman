use prost_reflect::{FieldDescriptor, FileDescriptor, Kind, MessageDescriptor};
use serde_json::{json, Map, Value};

/// Returns the JSON Schema of a message in the JSON format that requests are written in.
#[must_use]
pub fn json_schema(message: &MessageDescriptor) -> Value {
    let mut definitions = Map::new();
    let mut schema = message_schema(message, &mut definitions);

    schema["$defs"] = Value::Object(definitions);
    schema
}

/// Adds the message and all messages it uses to the definitions and returns
/// a reference to it.
fn message_schema(message: &MessageDescriptor, definitions: &mut Map<String, Value>) -> Value {
    let name = message.full_name().to_string();
    if definitions.contains_key(&name) {
        return reference(message);
    }
    // Reserve the name first so that recursive messages stop here.
    definitions.insert(name.clone(), Value::Null);

    let mut properties = Map::new();
    for field in message.fields() {
        properties.insert(
            field.json_name().to_string(),
            field_schema(&field, definitions),
        );
    }

    let mut schema = json!({
        "type": "object",
        "properties": properties,
        "additionalProperties": false,
    });
    add_description(&mut schema, comment(&message.parent_file(), message.path()));
    definitions.insert(name, schema);

    reference(message)
}

fn field_schema(field: &FieldDescriptor, definitions: &mut Map<String, Value>) -> Value {
    let mut schema = match field.kind() {
        Kind::Message(entry) if field.is_map() => {
            let value = kind_schema(&entry.map_entry_value_field().kind(), definitions);
            json!({ "type": "object", "additionalProperties": value })
        }
        kind if field.is_list() => {
            json!({ "type": "array", "items": kind_schema(&kind, definitions) })
        }
        kind => kind_schema(&kind, definitions),
    };

    add_description(&mut schema, comment(&field.parent_file(), field.path()));
    if let Some(oneof) = field
        .containing_oneof()
        .filter(|oneof| !oneof.is_synthetic())
    {
        add_description(
            &mut schema,
            Some(format!("Only one field of `{}` may be set.", oneof.name())),
        );
    }

    schema
}

fn kind_schema(kind: &Kind, definitions: &mut Map<String, Value>) -> Value {
    match kind {
        Kind::Double | Kind::Float => json!({ "type": "number" }),
        Kind::Int32
        | Kind::Int64
        | Kind::Uint32
        | Kind::Uint64
        | Kind::Sint32
        | Kind::Sint64
        | Kind::Fixed32
        | Kind::Fixed64
        | Kind::Sfixed32
        | Kind::Sfixed64 => json!({ "type": "integer" }),
        Kind::Bool => json!({ "type": "boolean" }),
        Kind::String => json!({ "type": "string" }),
        Kind::Bytes => json!({ "type": "string", "contentEncoding": "base64" }),
        Kind::Enum(enumeration) => {
            let names: Vec<String> = enumeration
                .values()
                .map(|value| value.name().to_string())
                .collect();
            json!({ "type": "string", "enum": names })
        }
        Kind::Message(message) => well_known_schema(message, definitions)
            .unwrap_or_else(|| message_schema(message, definitions)),
    }
}

/// Returns the schema of the well-known types that have their own JSON format.
fn well_known_schema(
    message: &MessageDescriptor,
    definitions: &mut Map<String, Value>,
) -> Option<Value> {
    match message.full_name() {
        "google.protobuf.Timestamp" => Some(json!({ "type": "string", "format": "date-time" })),
        "google.protobuf.Duration" | "google.protobuf.FieldMask" => {
            Some(json!({ "type": "string" }))
        }
        "google.protobuf.Struct"
        | "google.protobuf.Value"
        | "google.protobuf.ListValue"
        | "google.protobuf.Any" => Some(json!({})),
        "google.protobuf.DoubleValue"
        | "google.protobuf.FloatValue"
        | "google.protobuf.Int64Value"
        | "google.protobuf.UInt64Value"
        | "google.protobuf.Int32Value"
        | "google.protobuf.UInt32Value"
        | "google.protobuf.BoolValue"
        | "google.protobuf.StringValue"
        | "google.protobuf.BytesValue" => {
            let value = message.get_field(1)?;
            Some(kind_schema(&value.kind(), definitions))
        }
        _ => None,
    }
}

fn reference(message: &MessageDescriptor) -> Value {
    json!({ "$ref": format!("#/$defs/{}", message.full_name()) })
}

/// Returns the comment written above a service, method, message or field in
/// the proto file.
pub fn comment(file: &FileDescriptor, path: &[i32]) -> Option<String> {
    let source_code_info = file.file_descriptor_proto().source_code_info.as_ref()?;
    let location = source_code_info
        .location
        .iter()
        .find(|location| location.path == path)?;
    let comment = location.leading_comments.as_deref()?.trim();

    (!comment.is_empty()).then(|| comment.to_string())
}

fn add_description(schema: &mut Value, description: Option<String>) {
    let Some(description) = description else {
        return;
    };

    let description = match schema.get("description").and_then(Value::as_str) {
        Some(existing) => format!("{existing}\n{description}"),
        None => description,
    };
    schema["description"] = Value::String(description);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProtoDescriptor;

    #[test]
    fn test_json_schema() {
        struct TestCase {
            name: &'static str,
            given_message: &'static str,
            expected_schema: Value,
        }

        let test_cases = vec![
            TestCase {
                name: "success: scalar fields",
                given_message: "schema.Scalars",
                expected_schema: json!({
                    "$ref": "#/$defs/schema.Scalars",
                    "$defs": {
                        "schema.Scalars": {
                            "type": "object",
                            "properties": {
                                "name": { "type": "string" },
                                "count": { "type": "integer" },
                                "ratio": { "type": "number" },
                                "active": { "type": "boolean" },
                                "data": { "type": "string", "contentEncoding": "base64" },
                            },
                            "additionalProperties": false,
                        },
                    },
                }),
            },
            TestCase {
                name: "success: repeated and nested",
                given_message: "schema.Lists",
                expected_schema: json!({
                    "$ref": "#/$defs/schema.Lists",
                    "$defs": {
                        "schema.Lists": {
                            "type": "object",
                            "properties": {
                                "tags": { "type": "array", "items": { "type": "string" } },
                                "items": {
                                    "type": "array",
                                    "items": { "$ref": "#/$defs/schema.Item" },
                                },
                            },
                            "additionalProperties": false,
                        },
                        "schema.Item": {
                            "type": "object",
                            "properties": { "number": { "type": "integer" } },
                            "additionalProperties": false,
                        },
                    },
                }),
            },
            TestCase {
                name: "success: enum",
                given_message: "schema.Colored",
                expected_schema: json!({
                    "$ref": "#/$defs/schema.Colored",
                    "$defs": {
                        "schema.Colored": {
                            "type": "object",
                            "properties": {
                                "color": {
                                    "type": "string",
                                    "enum": ["COLOR_UNSPECIFIED", "COLOR_RED"],
                                },
                            },
                            "additionalProperties": false,
                        },
                    },
                }),
            },
            TestCase {
                name: "success: map",
                given_message: "schema.Labels",
                expected_schema: json!({
                    "$ref": "#/$defs/schema.Labels",
                    "$defs": {
                        "schema.Labels": {
                            "type": "object",
                            "properties": {
                                "counts": {
                                    "type": "object",
                                    "additionalProperties": { "type": "integer" },
                                },
                            },
                            "additionalProperties": false,
                        },
                    },
                }),
            },
            TestCase {
                name: "success: recursive message",
                given_message: "schema.Tree",
                expected_schema: json!({
                    "$ref": "#/$defs/schema.Tree",
                    "$defs": {
                        "schema.Tree": {
                            "type": "object",
                            "properties": {
                                "children": {
                                    "type": "array",
                                    "items": { "$ref": "#/$defs/schema.Tree" },
                                },
                            },
                            "additionalProperties": false,
                        },
                    },
                }),
            },
            TestCase {
                name: "success: oneof and timestamp",
                given_message: "schema.Choice",
                expected_schema: json!({
                    "$ref": "#/$defs/schema.Choice",
                    "$defs": {
                        "schema.Choice": {
                            "type": "object",
                            "properties": {
                                "id": {
                                    "type": "string",
                                    "description": "Only one field of `target` may be set.",
                                },
                                "name": {
                                    "type": "string",
                                    "description": "Only one field of `target` may be set.",
                                },
                                "createdAt": { "type": "string", "format": "date-time" },
                            },
                            "additionalProperties": false,
                        },
                    },
                }),
            },
            TestCase {
                name: "success: comment as description",
                given_message: "schema.User",
                expected_schema: json!({
                    "$ref": "#/$defs/schema.User",
                    "$defs": {
                        "schema.User": {
                            "type": "object",
                            "description": "A user of the system.",
                            "properties": {
                                "displayName": {
                                    "type": "string",
                                    "description": "The display name.",
                                },
                            },
                            "additionalProperties": false,
                        },
                    },
                }),
            },
        ];

        let descriptor = ProtoDescriptor::new(vec!["."], vec!["test_files/schema.proto"]).unwrap();

        for test_case in test_cases {
            let message = descriptor
                .get_message_by_name(test_case.given_message)
                .unwrap();

            let schema = json_schema(&message);

            assert_eq!(schema, test_case.expected_schema, "{}", test_case.name);
        }
    }
}
