use paste;
use serde::{Serialize, Serializer};

use crate::{Element, Number};
use crate::error::TychoError;
use crate::serde::ser::map::MapSerializer;
use crate::serde::ser::seq::{SeqSerializer, SeqSerializerType};
use crate::serde::ser::struct_::StructSerializer;
use crate::serde::ser::variant::{VariantSeqSerializer, VariantStructSerializer};
use crate::Value;

macro_rules! serialize_number {
    ($ident: ident, $type: ty) => {
        paste::item! {
            fn [< serialize_ $type >](self, v: $type) -> Result<Self::Ok, Self::Error> {
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
    type SerializeTupleStruct = SeqSerializer;
    type SerializeTupleVariant = VariantSeqSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = StructSerializer;
    type SerializeStructVariant = VariantStructSerializer;

    fn is_human_readable(&self) -> bool { false }


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
        Ok(Element::Option(Some(Box::new(value.serialize(Self)?))))
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
        Ok(value.serialize(Self)?)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Ok(Element::Variant(variant.to_string(), Box::new(value.serialize(Self)?)))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Self::SerializeSeq::new(SeqSerializerType::Default))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Self::SerializeSeq::new(SeqSerializerType::Default))
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Self::SerializeSeq::new(SeqSerializerType::Default))
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(Self::SerializeTupleVariant::new(variant, SeqSerializer::new(SeqSerializerType::Default)))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Self::SerializeMap::new())
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Self::SerializeStruct::new(name))
    }

    fn serialize_struct_variant(self, name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(Self::SerializeStructVariant::new(variant, StructSerializer::new(name)))
    }
}
