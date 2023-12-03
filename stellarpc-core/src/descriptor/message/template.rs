use prost_reflect::{DynamicMessage, Kind, MessageDescriptor, ReflectMessage, Value};

/// The default length of vectors used for message templates.
const TEMPLATE_VEC_LENGTH: usize = 1;

/// The max message depth to avoid stack overflow on recursive protos.
const MAX_MESSAGE_RECURSION: usize = 10;

/// Applies default values to fields of a `DynamicMessage`.
///
/// This function iterates over each field of the message and sets default values
/// based on the field's data type.
pub(super) fn apply_template_for_message(msg: &mut DynamicMessage, recursion_depth: usize) {
    if recursion_depth > MAX_MESSAGE_RECURSION {
        return;
    }
    for field in msg.descriptor().fields() {
        let kind = field.kind();
        let value = msg.get_field_mut(&field);

        match value {
            Value::List(_) => *value = default_value_list(&kind, recursion_depth),
            Value::Message(msg) => {
                *value = default_value_message(&msg.descriptor(), recursion_depth)
            }
            Value::Map(_) => {
                // println!("TODO: IMPLEMENT MAP!!");
            }
            _ => *value = default_value(&kind, recursion_depth),
        }
    }
}

/// Get the default value for a specific data type (Kind).
fn default_value(kind: &Kind, recursion_depth: usize) -> Value {
    match kind {
        Kind::String => Value::String("".to_string()),
        Kind::Message(desc) => default_value_message(desc, recursion_depth),
        _ => Value::default_value(kind),
    }
}

/// Get the default value for a list of a specific data type (Kind).
fn default_value_list(kind: &Kind, recursion_depth: usize) -> Value {
    let default = default_value(kind, recursion_depth);
    Value::List(vec![default; TEMPLATE_VEC_LENGTH])
}

/// Get the default value for a message.
fn default_value_message(desc: &MessageDescriptor, recursion_depth: usize) -> Value {
    let mut msg = DynamicMessage::new(desc.clone());
    apply_template_for_message(&mut msg, recursion_depth + 1);
    Value::Message(msg)
}
