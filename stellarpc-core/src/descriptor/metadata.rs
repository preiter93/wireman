use std::ops::{Deref, DerefMut};

use tonic::metadata::MetadataMap;

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
