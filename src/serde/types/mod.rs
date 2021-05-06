use serde::{Serialize, Serializer};
use crate::{Element, Value, Number};
use serde::ser::{SerializeStruct, SerializeSeq, SerializeMap};

#[cfg(feature="serde_types")]
impl Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        match self {
            Element::Unit => serializer.serialize_unit(),
            Element::Value(x) => x.serialize(serializer),
            Element::Option(o) => match o {
                Some(x) => serializer.serialize_some(x),
                None => serializer.serialize_none()
            },
            Element::Variant(n, v) => serializer.serialize_newtype_variant("", 0, &*n.to_string(), v),
            Element::Struct(x) => {
                let mut s = serializer.serialize_struct("___tycho___/struct", 1)?;
                s.serialize_field("inner", x);
                s.end()
            },
            Element::List(x) => {
                let mut s = serializer.serialize_struct("___tycho___/list", 1)?;
                s.serialize_field("inner", x);
                s.end()
            }
            Element::Array(i, x) => {
                let mut s = serializer.serialize_struct("___tycho___/array", 2)?;
                s.serialize_field("ident", &i.to_internal_prefix())?;
                s.serialize_field("inner", x)?;
                s.end()
            }
            Element::Map(i, x) => {
                let mut s = serializer.serialize_struct("___tycho___/map", 2)?;
                s.serialize_field("ident", &i.to_internal_prefix())?;
                s.serialize_field("inner", x)?;
                s.end()
            }
            Element::Compression(c) => {
                let mut s = serializer.serialize_struct("___tycho___/compression", 1)?;
                s.serialize_field("inner", c);
                s.end()
            }
        }
    }
}

#[cfg(not(feature="serde_types"))]
impl Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        match self {
            Element::Unit => serializer.serialize_unit(),
            Element::Value(x) => x.serialize(serializer),
            Element::Option(o) => match o {
                Some(x) => serializer.serialize_some(x),
                None => serializer.serialize_none()
            },
            Element::Variant(n, v) => serializer.serialize_newtype_variant("", 0, &n.to_string(), v),
            Element::Struct(x) => x.serialize(serializer),
            Element::List(x) => x.serialize(serializer),
            Element::Array(_i, x) => x.serialize(serializer),
            Element::Map(_i, x) => x.serialize(serializer),
            Element::Compression(c) => c.serialize(serializer),
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Boolean(x) => serializer.serialize_bool(*x),
            Value::String(x) => serializer.serialize_str(x),
            Value::Char(x) => serializer.serialize_char(*x),
            Value::Number(num) => num.serialize(serializer),
            Value::Bytes(x) => serializer.serialize_bytes(x),
            Value::UUID(x) => x.serialize(serializer)
        }
    }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        match self {
            Number::Bit(x) => serializer.serialize_bool(*x),
            Number::Unsigned8(x) => serializer.serialize_u8(*x),
            Number::Signed8(x) => serializer.serialize_i8(*x),
            Number::Unsigned16(x) => serializer.serialize_u16(*x),
            Number::Signed16(x) => serializer.serialize_i16(*x),
            Number::Unsigned32(x) => serializer.serialize_u32(*x),
            Number::Signed32(x) => serializer.serialize_i32(*x),
            Number::Unsigned64(x) => serializer.serialize_u64(*x),
            Number::Signed64(x) => serializer.serialize_i64(*x),

            #[cfg(not(feature="serde_types"))]
            Number::Unsigned128(x) => serializer.serialize_bytes(&x.to_be_bytes()),
            #[cfg(not(feature="serde_types"))]
            Number::Signed128(x) => serializer.serialize_bytes(&x.to_be_bytes()),

            #[cfg(feature="serde_types")]
            Number::Unsigned128(x) => {
                let mut stu = serializer.serialize_struct("___tycho___/u128", 2)?;
                stu.serialize_field("0", &((x >> 64) as u64))?;
                stu.serialize_field("1", &((x & 0xFFFF_FFFF_FFFF_FFFF) as u64))?;
                stu.end()
            }

            #[cfg(feature="serde_types")]
            Number::Signed128(x) => {
                let mut stu = serializer.serialize_struct("___tycho___/i128", 2)?;
                stu.serialize_field("0", &(((*x as u128) >> 64) as u64))?;
                stu.serialize_field("1", &(((*x as u128) & 0xFFFF_FFFF_FFFF_FFFF) as u64))?;
                stu.end()
            }

            Number::Float32(x) => serializer.serialize_f32(*x),
            Number::Float64(x) => serializer.serialize_f64(*x),
        }
    }
}