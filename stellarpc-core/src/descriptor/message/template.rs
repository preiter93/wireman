use prost_reflect::{DynamicMessage, Kind, MessageDescriptor, ReflectMessage, Value};

/// The default length of vectors used for message templates.
const TEMPLATE_VEC_LENGTH: usize = 1;

/// Applies default values to fields of a `DynamicMessage`.
///
/// This function iterates over each field of the message and sets default values
/// based on the field's data type.
pub(super) fn apply_template_for_message(msg: &mut DynamicMessage) {
    for field in msg.descriptor().fields() {
        let kind = field.kind();
        let value = msg.get_field_mut(&field);

        match value {
            Value::List(_) => *value = default_value_list(&kind),
            Value::Message(msg) => *value = default_value_message(msg),
            Value::Map(_) => {
                // println!("TODO: IMPLEMENT MAP!!");
            }
            _ => *value = default_value(&kind),
        }
    }
}

/// Get the default value for a specific data type (Kind).
fn default_value(kind: &Kind) -> Value {
    match kind {
        Kind::String => Value::String("".to_string()),
        Kind::Message(desc) => default_value_descriptor(desc),
        _ => Value::default_value(kind),
    }
}

/// Get the default value for a list of a specific data type (Kind).
fn default_value_list(kind: &Kind) -> Value {
    let default = default_value(kind);
    Value::List(vec![default; TEMPLATE_VEC_LENGTH])
}

/// Get the default value for a nested message.
fn default_value_message(msg: &DynamicMessage) -> Value {
    let mut msg = msg.clone();
    apply_template_for_message(&mut msg);
    Value::Message(msg)
}

/// Get the default value for a message descriptor.
fn default_value_descriptor(desc: &MessageDescriptor) -> Value {
    let mut msg = DynamicMessage::new(desc.clone());
    apply_template_for_message(&mut msg);
    Value::Message(msg)
}
