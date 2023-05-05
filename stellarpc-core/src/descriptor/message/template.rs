use prost_reflect::{DynamicMessage, Kind, MessageDescriptor, ReflectMessage, Value};

const TEMPLATE_VEC_LENGTH: usize = 1;

pub(super) fn apply_template_for_message(msg: &mut DynamicMessage) {
    for field in msg.descriptor().fields() {
        let kind = field.kind();
        let value = msg.get_field_mut(&field);

        match value {
            Value::List(_) => *value = default_value_list(&kind),
            Value::Message(msg) => *value = default_value_message(msg),
            _ => *value = default_value(&kind),
        }
    }
}

fn default_value(kind: &Kind) -> Value {
    match kind {
        Kind::String => Value::String("Hello".to_string()),
        Kind::Message(desc) => default_value_descriptor(desc),
        _ => Value::default_value(kind),
    }
}

fn default_value_list(kind: &Kind) -> Value {
    let default = default_value(kind);
    Value::List(vec![default; TEMPLATE_VEC_LENGTH])
}

fn default_value_message(msg: &DynamicMessage) -> Value {
    let mut msg = msg.clone();
    apply_template_for_message(&mut msg);
    Value::Message(msg)
}

fn default_value_descriptor(desc: &MessageDescriptor) -> Value {
    let mut msg = DynamicMessage::new(desc.clone());
    apply_template_for_message(&mut msg);
    Value::Message(msg)
}
