use serde::{Serialize, Serializer};
use crate::{Element, Value};
use serde::__private::ser::serialize_tagged_newtype;
use serde::ser::{SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTupleVariant, SerializeMap};
use std::ops::Deref;

// I'm so fucking drunk rn i's unreal, this is gonna be a shit storm, i'll fix it if i remember.

impl Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        match &self {
            Element::Unit => serializer.serialize_unit(),
            Element::Value(v) => v.serialize(serializer),
            Element::Option(o) => match o {
                Some(v) => serializer.serialize_some(v),
                None => serializer.serialize_none()
            }
            Element::Array(a) => {
                let mut seq = serializer.serialize_seq(Some(a.len()))?;
                for e in a {
                    seq.serialize_element(e)?;
                }
                Ok(seq)
            }
            Element::Struct(s) => {
                let mut sct = serializer.serialize_struct("", s.len())?;
                for (k, v) in s.iter() {
                    sct.serialize_field(&k, &v)?;
                }
                Ok(seq)
            }
            Element::Variant(n, e) => {
                match e.deref() {
                    Element::Unit => serializer.serialize_unit_variant("", 0, &n),
                    Element::Struct(s) => {
                        let mut sct = serializer.serialize_struct_variant("", 0, &n, s.len())?;
                        for (k, v) in s.iter() {
                            sct.serialize_field(&k, &v)?;
                        }
                        Ok(sct)
                    },
                    Element::List(a) => {
                        let mut s = serializer.serialize_tuple_variant("", 0, &name, a.len())?;
                        for e in a {
                            s.serialize_field(e)?;
                        }
                        Ok(s)
                    }
                    Element::Array(a) => {
                        let mut s = serializer.serialize_tuple_variant("", 0, &name, a.len())?;
                        for e in a {
                            s.serialize_field(e)?;
                        }
                        Ok(s)
                    }
                    _ => serializer.serialize_newtype_variant("", 0, &name, e)
                }
            }
            Element::Map(m) => {
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(&k, &v)?;
                }
                Ok(map)
            }
            Element::List(l) => {
                let mut map = serializer.serialize_seq(Some(l.len()))?;
                for e in l {
                    map.serialize_element(&l);
                }
                return map;
            }
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        match &self {
            Value::Boolean(v) => serializer.serialize_bool(*v),
            Value::Unsigned8(v) => serializer.serialize_u8(*v),
            Value::Unsigned16(v) => serializer.serialize_u16(*v),
            Value::Unsigned32(v) => serializer.serialize_u32(*v),
            Value::Unsigned64(v) => serializer.serialize_u64(*v),
            // todo: fix this shite
            Value::Unsigned128(v) => serializer.serialize_u64(v as u64),
            Value::Signed8(v) => serializer.serialize_i8(*v),
            Value::Signed16(v) => serializer.serialize_i16(*v),
            Value::Signed32(v) => serializer.serialize_i32(*v),
            Value::Signed64(v) => serializer.serialize_i64(*v),
            // todo: also fix this shite
            Value::Signed128(v) => serializer.serialize_i128(v as i64),
            Value::Float32(v) => serializer.serialize_f32(*v),
            Value::Float64(v) => serializer.serialize_f64(*v),
            Value::String(v) => serializer.serialize_str(&v),
            Value::Char(v) => serializer.serialize_char(*v),
            Value::Bytes(v) => serializer.serialize_bytes(&v)
        }
    }
}