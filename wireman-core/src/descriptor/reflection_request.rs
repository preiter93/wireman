use super::metadata::Metadata;
use crate::error::{Error, Result};
use tonic::metadata::{Ascii, MetadataKey, MetadataValue};

/// Holds all the necessary data for a reflection request.
#[derive(Debug, Clone)]
pub struct ReflectionRequest {
    /// The host address.
    pub host: String,
    /// The requests metadata.
    pub metadata: Option<Metadata>,
}

impl ReflectionRequest {
    /// Creates a new reflection request with a given hostname.
    #[must_use]
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            metadata: None,
        }
    }

    /// Insert metadata into the reflection request.
    ///
    /// # Errors
    ///
    /// - Failed to parse metadata value/key to ascii
    pub fn insert_metadata(&mut self, key: &str, val: &str) -> Result<()> {
        let key: MetadataKey<Ascii> = key.parse().map_err(|_| Error::ParseToAsciiError)?;
        let val: MetadataValue<Ascii> = val.parse().map_err(|_| Error::ParseToAsciiError)?;
        let map = self.metadata.get_or_insert(Metadata::new());
        map.insert(key, val);
        Ok(())
    }
}
