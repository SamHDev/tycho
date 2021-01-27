use serde::de::{Visitor, SeqAccess, EnumAccess, MapAccess};
use crate::{Element, Value};
use serde::__private::Formatter;
use serde::Deserializer;
use std::error::Error;
use std::fmt;

pub struct TychoVisitor;

impl<'de> Visitor<'de> for TychoVisitor {
    type Value = Element;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Expecting another type lmao")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Boolean(v)))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Signed8(v)))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Signed16(v)))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Signed32(v)))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Signed64(v)))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de>, {
        unimplemented!()
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> where
        E: Error, {
        unimplemented!()
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de>, {
        unimplemented!()
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error> where
        A: SeqAccess<'de>, {
        unimplemented!()
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, <A as MapAccess<'de>>::Error> where
        A: MapAccess<'de>, {
        unimplemented!()
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, <A as EnumAccess<'de>>::Error> where
        A: EnumAccess<'de>, {
        unimplemented!()
    }
}