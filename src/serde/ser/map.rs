use std::collections::HashMap;

use serde::ser::{Error, SerializeMap};
use serde::{Serialize, Serializer};

use crate::{Element, Value};
use crate::error::TychoError;
use crate::ident::ValueIdent;
use crate::into::ident::Ident;
use crate::serde::ser::TychoSerializer;
use std::fmt;

pub struct MapSerializer {
    content: HashMap<Value, Element>,
    map_type: ValueIdent,
    key: Option<Value>
}

impl MapSerializer {
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
            map_type: ValueIdent::Null,
            key: None
        }
    }

    pub fn typed(x: ValueIdent) -> Self {
        Self {
            content: HashMap::new(),
            map_type: x,
            key: None
        }
    }
}

impl SerializeMap for MapSerializer {
    type Ok = Element;
    type Error = TychoError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where
        T: Serialize {
        let data = key.serialize(TychoSerializer)?;

        if let Element::Value(value) = data {
            if self.map_type == ValueIdent::Null {
              self.map_type = value.ident();
            } else if value.ident() != self.map_type {
                return Err(TychoError::MismatchedType { found: value.ident(), expected: self.map_type.clone() })
            }
            self.key = Some(value);
        } else {
            return Err(TychoError::InvalidKeyType { found: data.ident() })
        }

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {

        let value = value.serialize(TychoSerializer)?;
        let key = std::mem::replace(&mut self.key, None);
        if let Some(k) = key {
            self.content.insert(k, value);
        } else {
            return Err(TychoError::custom("Invalid key state."));
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Map(self.map_type, self.content))
    }
}

impl Serializer for MapSerializer {
    type Ok = Element;
    type Error = TychoError;
    type SerializeSeq = serde::ser::Impossible<Element, TychoError>;
    type SerializeTuple = serde::ser::Impossible<Element, TychoError>;
    type SerializeTupleStruct = serde::ser::Impossible<Element, TychoError>;
    type SerializeTupleVariant = serde::ser::Impossible<Element, TychoError>;
    type SerializeMap = Self;
    type SerializeStruct = serde::ser::Impossible<Element, TychoError>;
    type SerializeStructVariant = serde::ser::Impossible<Element, TychoError>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: fmt::Display {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }
}