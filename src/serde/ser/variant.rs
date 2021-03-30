use serde::ser::{SerializeStruct, SerializeStructVariant, SerializeTupleVariant};
use serde::Serialize;

use crate::Element;
use crate::error::TychoError;
use crate::serde::ser::seq::SeqSerializer;
use crate::serde::ser::struct_::StructSerializer;

pub struct VariantSeqSerializer {
    name: String,
    seq: SeqSerializer
}

impl VariantSeqSerializer {
    pub fn new(name: &str, seq: SeqSerializer) -> Self {
        Self {
            name: name.to_string(),
            seq
        }
    }
}

impl SerializeTupleVariant for VariantSeqSerializer {
    type Ok = Element;
    type Error = TychoError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.seq.element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Variant(self.name, Box::from(self.seq.finish()?)))
    }
}

pub struct VariantStructSerializer {
    name: String,
    inner: StructSerializer
}

impl VariantStructSerializer {
    pub fn new(s: &str, inner: StructSerializer) -> Self {
        Self {
            name: s.to_string(),
            inner
        }
    }
}

impl SerializeStructVariant for VariantStructSerializer {
    type Ok = Element;
    type Error = TychoError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.inner.serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Variant(self.name, Box::from(self.inner.end()?)))
    }
}