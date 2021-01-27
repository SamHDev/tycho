use crate::{Element, Value};
use serde::de::{Visitor};
use serde::Deserializer;
use crate::de::error::TychoDeserializeError;
use crate::ident::{ElementIdent, ValueIdent};
use crate::encode::element::ElementEncoder;
use crate::encode::value::ValueEncoder;
use crate::de::seq::{TychoArrayDeserializer, TychoListDeserializer};
use crate::de::map::TychoMapDeserializer;
use crate::de::sct::TychoStructDeserializer;
use crate::de::var::TychoVariantDeserializer;

pub struct TychoDeserializer(Element);

impl TychoDeserializer {
    pub(crate) fn new(e: Element) -> Self {
        Self(e)
    }

    pub(crate) fn get_value(self) -> Result<Value, TychoDeserializeError> {
        if let Element::Value(value) = self.0 {
           Ok(value)
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::Value,
                found: self.0.ident()
            })
        }
    }
}

impl<'de> Deserializer<'de> for TychoDeserializer {
    type Error = TychoDeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            Element::Unit => visitor.visit_unit(),
            Element::Value(value) => match value {
                Value::Boolean(value) => visitor.visit_bool(value),
                Value::Unsigned8(value) => visitor.visit_u8(value),
                Value::Unsigned16(value) => visitor.visit_u16(value),
                Value::Unsigned32(value) => visitor.visit_u32(value),
                Value::Unsigned64(value) => visitor.visit_u64(value),
                Value::Unsigned128(value) => visitor.visit_bytes(&value.to_be_bytes().to_vec()),
                Value::Signed8(value) => visitor.visit_i8(value),
                Value::Signed16(value) => visitor.visit_i16(value),
                Value::Signed32(value) => visitor.visit_i32(value),
                Value::Signed64(value) => visitor.visit_i64(value),
                Value::Signed128(value) => visitor.visit_bytes(&value.to_be_bytes().to_vec()),
                Value::Float32(value) => visitor.visit_f32(value),
                Value::Float64(value) => visitor.visit_f64(value),
                Value::String(value) => visitor.visit_str(&value),
                Value::Char(value) => visitor.visit_char(value),
                Value::Bytes(value) => visitor.visit_bytes(&value)
            }
            Element::Option(opt) => match opt {
                Some(some) => visitor.visit_some(TychoDeserializer::new(*some)),
                None => visitor.visit_none()
            }
            Element::Array(array) => visitor.visit_seq(TychoArrayDeserializer::new(array)),
            Element::List(list) => visitor.visit_seq(TychoListDeserializer::new(list)),
            Element::Struct(sct) => visitor.visit_map(TychoStructDeserializer::new(sct)),
            Element::Variant(name, inner) => visitor.visit_enum(TychoVariantDeserializer::new(&name, *inner)),
            Element::Map(map) => visitor.visit_map(TychoMapDeserializer::new(map))
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Boolean(value) = v {
            visitor.visit_bool(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Boolean,
                found: v.ident()
            })
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Signed8(value) = v {
            visitor.visit_i8(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Signed8,
                found: v.ident()
            })
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Signed16(value) = v {
            visitor.visit_i16(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Signed16,
                found: v.ident()
            })
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Signed32(value) = v {
            visitor.visit_i32(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Signed32,
                found: v.ident()
            })
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Signed64(value) = v {
            visitor.visit_i64(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Signed64,
                found: v.ident()
            })
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Unsigned8(value) = v {
            visitor.visit_u8(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Unsigned8,
                found: v.ident()
            })
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Unsigned16(value) = v {
            visitor.visit_u16(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Unsigned16,
                found: v.ident()
            })
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Unsigned32(value) = v {
            visitor.visit_u32(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Unsigned32,
                found: v.ident()
            })
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Unsigned64(value) = v {
            visitor.visit_u64(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Unsigned64,
                found: v.ident()
            })
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Float32(value) = v {
            visitor.visit_f32(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Float32,
                found: v.ident()
            })
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Float64(value) = v {
            visitor.visit_f64(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Float64,
                found: v.ident()
            })
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Char(value) = v {
            visitor.visit_char(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Char,
                found: v.ident()
            })
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::String(value) = v {
            visitor.visit_str(&value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::String,
                found: v.ident()
            })
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::String(value) = v {
            visitor.visit_string(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::String,
                found: v.ident()
            })
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Bytes(value) = v {
            visitor.visit_bytes(&value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Bytes,
                found: v.ident()
            })
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let v = self.get_value()?;
        if let Value::Bytes(value) = v {
            visitor.visit_byte_buf(value)
        } else {
            Err(TychoDeserializeError::ExpectingValueType {
                expecting: ValueIdent::Bytes,
                found: v.ident()
            })
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Element::Option(value) = self.0 {
            match value {
                Some(v) => visitor.visit_some(Self::new(*v)),
                None => visitor.visit_none()
            }
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::Option,
                found: self.0.ident()
            })
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Element::Unit = self.0 {
            visitor.visit_unit()
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::Unit,
                found: self.0.ident()
            })
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Element::Array(array) = self.0 {
            visitor.visit_seq(TychoArrayDeserializer::new(array))
        } else if let Element::List(list) = self.0 {
            visitor.visit_seq(TychoListDeserializer::new(list))
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::Array,
                found: self.0.ident()
            })
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_map(TychoMapDeserializer::from(self.0)?)
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_map(TychoStructDeserializer::from(self.0)?)
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Element::Variant(var_name, inner) = self.0 {
            visitor.visit_enum(TychoVariantDeserializer::new(&var_name, *inner))
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::Variant,
                found: self.0.ident()
            })
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Element::Value( Value::String(ident)) = self.0 {
            visitor.visit_string(ident)
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::Variant,
                found: self.0.ident()
            })
        }
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }
}