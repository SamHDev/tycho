use serde::ser::{SerializeStructVariant, SerializeStruct};
use crate::Element;
use crate::ser::error::TychoSerializerError;
use serde::Serialize;
use crate::ser::ser::TychoSerializer;
use std::collections::HashMap;

pub struct TychoStructSerializer {
    data: HashMap<String, Element>
}

impl TychoStructSerializer {
    pub(crate) fn new() -> Self {
        Self { data: HashMap::new() }
    }
}

impl SerializeStruct for TychoStructSerializer {
    type Ok = Element;
    type Error = TychoSerializerError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.data.insert(
            key.to_string(),
            value.serialize(TychoSerializer)?
        );
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Struct(self.data))
    }
}

pub struct TychoVariantStructSerializer {
    inner: TychoStructSerializer,
    name: String
}

impl TychoVariantStructSerializer {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            inner: TychoStructSerializer::new(),
            name: name.to_string()
        }
    }
}

impl SerializeStructVariant for TychoVariantStructSerializer {
    type Ok = Element;
    type Error = TychoSerializerError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.inner.serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Variant(self.name, Box::new(self.inner.end()?)))
    }
}