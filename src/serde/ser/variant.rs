use crate::serde::ser::seq::SeqSerializer;
use serde::ser::SerializeTupleVariant;
use crate::Element;
use crate::error::TychoError;
use serde::Serialize;

pub struct VariantSeqSerializer {
    name: String,
    seq: SeqSerializer
}

impl SerializeTupleVariant for VariantSeqSerializer {
    type Ok = Element;
    type Error = TychoError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.seq.element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Variant(self.name, self.seq.finish()?))
    }
}