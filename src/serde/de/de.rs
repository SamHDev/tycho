use serde::de::Visitor;
use serde::Deserializer;

use crate::{Element, Number, Value};
use crate::error::TychoError;
use crate::serde::de::map::MapDeserializer;
use crate::serde::de::seq::{SeqArrayDeserializer, SeqListDeserializer};
use crate::serde::de::struct_::StructDeserializer;
use crate::serde::de::variant::EnumDeserializer;

pub struct TychoDeserializer(Element);

impl TychoDeserializer {
    pub fn new(e: Element) -> Self {
        Self(e)
    }
}

/*macro_rules! deserialize_number {
    ($ident: ident, $type: ty) => {
        paste::item! {
            fn [< deserialize_ $type >]<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
               if let Element::Value(element) = self.0 {
                   if let Value::Number(Number::$ident(number)) = element {
                        return visitor.[visit_ $type ]()
                   } else {
                        return Err(TychoError::ExpectedType {
                            found: element.ident().to_string(),
                            expecting: "" $type "".to_string(),
                        })
                   }
               } else {
                    return Err(TychoError::ExpectedType {
                        found: self.0.ident().to_string(),
                        expecting: "" $type "".to_string(),
                    })
               }
            }

        }
    };
}
*/

impl<'de> Deserializer<'de> for TychoDeserializer {
    type Error = TychoError;

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            Element::Unit => visitor.visit_unit(),
            Element::Value(value) => match value {
                Value::Null => visitor.visit_unit(),
                Value::Boolean(v) => visitor.visit_bool(v),
                Value::String(v) => visitor.visit_string(v),
                Value::Char(v) => visitor.visit_char(v),
                Value::Number(number) => match number {
                    Number::Bit(v) => visitor.visit_bool(v),
                    Number::Unsigned8(v) => visitor.visit_u8(v),
                    Number::Signed8(v) => visitor.visit_i8(v),
                    Number::Unsigned16(v) => visitor.visit_u16(v),
                    Number::Signed16(v) => visitor.visit_i16(v),
                    Number::Unsigned32(v) => visitor.visit_u32(v),
                    Number::Signed32(v) => visitor.visit_i32(v),
                    Number::Unsigned64(v) => visitor.visit_u64(v),
                    Number::Signed64(v) => visitor.visit_i64(v),
                    Number::Unsigned128(_) => visitor.visit_unit(), //todo: fix this shit
                    Number::Signed128(_) => visitor.visit_unit(), //todo: fix this shit
                    Number::Float32(v) => visitor.visit_f32(v),
                    Number::Float64(v) => visitor.visit_f64(v)
                }
                Value::Bytes(v) => visitor.visit_byte_buf(v),
                Value::UUID(v) => visitor.visit_bytes(v.as_bytes())
            },
            Element::Option(option) => match option {
                Some(v) => visitor.visit_some(TychoDeserializer::new(*v)),
                None => visitor.visit_none()
            }
            Element::Variant(name, x) => visitor.visit_enum(EnumDeserializer::new(&name, *x)),
            Element::Struct(x) => visitor.visit_map(StructDeserializer::new(x)),
            Element::List(x) => visitor.visit_seq(SeqListDeserializer::new(x)),
            Element::Array(_, x) => visitor.visit_seq(SeqArrayDeserializer::new(x)),
            Element::Map(_, x) => visitor.visit_map(MapDeserializer::new(x)),
            Element::Compression(x) => TychoDeserializer::new(*x).deserialize_any(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_i8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_i16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_i32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_i64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_u8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_u16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_u32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_u64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_f32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_f64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
        where V: Visitor<'de> { self.deserialize_any(visitor) }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        // todo; unsure if this is correct
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_any(visitor)
    }
}