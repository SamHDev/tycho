use serde::{Serializer, Serialize};
use crate::{Element, Number};
use crate::error::TychoError;
use crate::Value;
use paste;
use crate::serde::ser::seq::SeqSerializer;


macro_rules! serialize_number {
    ($ident: ident, $type: ty) => {
        paste::item! {
            fn [<  serialize_ $type >](self, v: $type) {
                Ok(Element::Value(Value::Number(Number::$ident(v))))
            }

        }
    };
}

pub(crate) struct TychoSerializer;

impl Serializer for TychoSerializer {
    type Ok = Element;
    type Error = TychoError;
    type SerializeSeq = SeqSerializer;
    type SerializeTuple = SeqSerializer;
    type SerializeTupleStruct = ();
    type SerializeTupleVariant = ();
    type SerializeMap = ();
    type SerializeStruct = ();
    type SerializeStructVariant = ();

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Boolean(v)))
    }


    serialize_number!(Unsigned8, u8);
    serialize_number!(Signed8, i8);
    serialize_number!(Unsigned16, u16);
    serialize_number!(Signed16, i16);
    serialize_number!(Unsigned32, u32);
    serialize_number!(Signed32, i32);
    serialize_number!(Unsigned64, u64);
    serialize_number!(Signed64, i64);
    serialize_number!(Unsigned128, u128);
    serialize_number!(Signed128, i128);
    serialize_number!(Float32, f32);
    serialize_number!(Float64, f64);

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Char(v)))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::String(v.to_string())))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Bytes(v.to_vec())))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Option(None))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Ok(Element::Option(Some(value.serialize(value)?)))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Unit)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Unit)
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Variant(variant.to_string(), Element::Unit.into()))
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Ok(value.serialize()?)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Ok(Element::Variant(variant.to_string(), value.serialize()?))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.serialize_seq()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}
