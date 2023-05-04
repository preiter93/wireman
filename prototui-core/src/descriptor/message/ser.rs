use serde::{ser::SerializeMap, Serialize, Serializer};

use super::DynMessage;

/// Options to control serialization of messages.
///
/// Taken from `prost_reflect`.
pub struct SerializeOptions {
    stringify_64_bit_integers: bool,
    use_enum_numbers: bool,
    use_proto_field_name: bool,
    skip_default_fields: bool,
}

impl SerializeOptions {
    pub fn new() -> Self {
        Self {
            stringify_64_bit_integers: true,
            use_enum_numbers: false,
            use_proto_field_name: false,
            skip_default_fields: true,
        }
    }

    pub fn stringify_64_bit_integers(mut self, stringify_64_bit_integers: bool) -> Self {
        self.stringify_64_bit_integers = stringify_64_bit_integers;
        self
    }

    pub fn skip_default_fields(mut self, skip_default_fields: bool) -> Self {
        self.skip_default_fields = skip_default_fields;
        self
    }
}

/// Adapted from `prost_reflect`.
struct SerializeWrapper<'a, T> {
    value: T,
    options: &'a SerializeOptions,
}

pub(super) fn serialize_message<S>(
    message: &DynMessage,
    serializer: S,
    options: &SerializeOptions,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    SerializeWrapper {
        value: message,
        options,
    }
    .serialize(serializer)
}

impl<'a> Serialize for SerializeWrapper<'a, &'a DynMessage> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.end()
    }
}
