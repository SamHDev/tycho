use crate::ser::seq::{TychoSeqSerializer, TychoVariantSeqSerializer};
use crate::ser::map::TychoMapSerializer;
use crate::ser::sct::{TychoStructSerializer, TychoVariantStructSerializer};
use std::fmt::Display;
use serde::{Serialize, Serializer};
use crate::{Element, Value};
use crate::ser::error::TychoSerializerError;

pub struct TychoSerializer;

impl Serializer for TychoSerializer {
    type Ok = Element;
    type Error = TychoSerializerError;
    type SerializeSeq = TychoSeqSerializer;
    type SerializeTuple = TychoSeqSerializer;
    type SerializeTupleStruct = TychoSeqSerializer;
    type SerializeTupleVariant = TychoVariantSeqSerializer;
    type SerializeMap = TychoMapSerializer;
    type SerializeStruct = TychoStructSerializer;
    type SerializeStructVariant = TychoVariantStructSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Boolean(v)))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Signed8(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Signed16(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Signed32(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Signed64(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Unsigned8(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Unsigned16(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Unsigned32(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Unsigned64(v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Float32(v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Value(Value::Float64(v)))
    }

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
        Ok(Element::Variant(variant.to_string(), Box::new(Element::Unit)))
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
        Ok(TychoSeqSerializer::new())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(TychoSeqSerializer::new())
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(TychoSeqSerializer::new())
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(TychoVariantSeqSerializer::new(variant))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(TychoMapSerializer::new())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(TychoStructSerializer::new())
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(TychoVariantStructSerializer::new(variant))
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Display {
        self.serialize_str(&value.to_string())
    }
}