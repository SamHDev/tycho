use std::ops::{Deref, DerefMut};

/// Maps to `Vec<u8>`, an unsized array of bytes
pub struct Bytes(pub Vec<u8>);


impl Default for Bytes {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Deref for Bytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Bytes {
    /// Create a new array of bytes
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

#[cfg(feature="serde_support")]
use serde::{Serialize, Serializer};

#[cfg(feature="serde_support")]
impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_bytes(&self.0)
    }
}
