use std::ops::{Deref, DerefMut};

use serde::{ser::SerializeMap, Serializer};
use tonic::metadata::{Ascii, KeyRef, MetadataKey, MetadataMap, MetadataValue};

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Metadata {
    pub(crate) inner: MetadataMap,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            inner: MetadataMap::new(),
        }
    }

    pub fn insert_ascii(&mut self, key: &str, val: &str) -> crate::error::Result<()> {
        let key: MetadataKey<Ascii> = key.parse().map_err(|_| Error::ParseToAsciiError)?;
        let val: MetadataValue<Ascii> = val.parse().map_err(|_| Error::ParseToAsciiError)?;
        self.insert(key, val);
        Ok(())
    }

    pub fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let len = self.inner.len();
        let mut map = ser.serialize_map(Some(len))?;
        for key in self.inner.keys() {
            if let KeyRef::Ascii(key) = key {
                let value = self.inner.get(key).unwrap();
                let value_str = value.to_str().unwrap();
                map.serialize_entry(&key.to_string(), value_str)?;
            }
        }
        map.end()
    }
}

impl Deref for Metadata {
    type Target = MetadataMap;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Metadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_serialize() {
        // given
        let mut metadata = Metadata::new();
        metadata.insert_ascii("auth", "bearer").unwrap();
        metadata.insert_ascii("alias", "xx").unwrap();

        let mut s = serde_json::Serializer::new(Vec::new());

        // when
        metadata.serialize(&mut s).unwrap();
        let json = String::from_utf8(s.into_inner()).unwrap();

        // then
        let expected_json = "{\"auth\":\"bearer\",\"alias\":\"xx\"}";
        assert_eq!(json, expected_json);
    }
}
