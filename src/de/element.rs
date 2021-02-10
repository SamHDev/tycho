use serde::{Deserialize, Deserializer};
use crate::{Value, Element};
use serde::de::{Visitor, Error, SeqAccess, EnumAccess, MapAccess};
use std::fmt;
use serde::__private::Formatter;
use crate::encode::value::ValueEncoder;
use crate::ident::ValueIdent;
use std::collections::HashMap;

impl Visitor for Value {
    type Value = Value;

    fn expecting<'a>(&self, formatter: &mut fmt::Formatter<'a>) -> fmt::Result {
        formatter.write_str("valid value")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Signed8(v))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Signed16(v))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Signed32(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Signed64(v))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Unsigned8(v))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Unsigned16(v))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Unsigned32(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Unsigned64(v))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Float32(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Float64(v))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Char(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::String(String::from(v)))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::String(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Bytes(v.to_vec()))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Value::Bytes(v))
    }
}

impl Visitor for Element {
    type Value = Element;

    fn expecting<'a>(&self, formatter: &mut fmt::Formatter<'a>) -> fmt::Result {
        formatter.write_str("valid element")
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
        Ok(Element::Value(Value::Unsigned8(v)))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Unsigned16(v)))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Unsigned32(v)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Unsigned64(v)))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Float32(v)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Float64(v)))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Char(v)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::String(v.to_string())))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::String(v)))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Bytes(v.to_vec())))
    }


    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Value(Value::Bytes(v)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Option(None))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, <D as Deserializer>::Error> where
        D: Deserializer, {
        Ok(Element::Option(Some(deserializer.deserialize_any(Self)?)))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> where
        E: Error, {
        Ok(Element::Unit)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, <D as Deserializer>::Error> where
        D: Deserializer, {
        Ok(deserializer.deserialize_any(Self)?)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess>::Error> where
        A: SeqAccess, {
        let mut b = Vec::new();

        let mut list_ident: Option<ValueIdent> = None;
        let mut is_list = true;

        while let Some(v) = seq.next_element()? {
            if is_list {
                if let Element::Value(i) = &v {
                    if list_ident == None {
                        list_ident = Some(i.ident());
                    } else if list_ident.unwrap() != i.ident() {
                        is_list = false;
                    }
                } else {
                    is_list = false;
                }
            }
            b.push(v);
        }
        if is_list && !b.is_empty() {
            let ls = b.into_iter()
                .filter_map(|x| if let Element::Value(v) = x { Some(v) } else { None }).collect();
            return Ok(Element::List(ls))
        } else {
            return Ok(Element::Array(b))
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, <A as MapAccess>::Error> where
        A: MapAccess, {
        let mut m = HashMap::<Value, Element>::new();

        let mut map_ident: Option<ValueIdent> = None;
        let mut is_struct = true;

        while let Some((key, value)) = map.next_entry()? {
            if is_struct {
                if let Value::String(s) = &key {}
                else { is_struct = false; }
            }
            if map_ident == None {
                map_ident = Some(key.ident())
            } else {
                if map_ident != key.ident() {
                    return Err(A::Error::custom("Invalid map. Keys are not of the same type."))
                }
            }
            m.insert(key, value);
        }

        if is_struct {
            Ok(Element::Struct(m.into_iter().filter_map(|(k, v)| if let Value::String(s) = k { Some((s, v))} else { None }).collect()))
        } else {
            Ok(Element::Map(m))
        }
    }


}