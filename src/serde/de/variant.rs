use serde::de::{DeserializeSeed, EnumAccess, VariantAccess, Visitor};
use serde::Deserializer;

use crate::Element;
use crate::error::TychoError;
use crate::serde::de::ident::TychoIdentDeserializer;
use crate::serde::de::TychoDeserializer;

pub struct EnumDeserializer {
    name: String,
    value: Element
}

impl EnumDeserializer {
    pub fn new(x: &str, value: Element) -> Self {
        Self {
            name: x.to_string(),
            value
        }
    }
}

impl<'de> EnumAccess<'de> for EnumDeserializer {
    type Error = TychoError;
    type Variant = VariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(<V as DeserializeSeed<'de>>::Value, Self::Variant), Self::Error> where
        V: DeserializeSeed<'de> {
        Ok((
            seed.deserialize(TychoIdentDeserializer::new(&self.name))?,
            VariantDeserializer::new(self.value)
        ))
    }
}

pub struct VariantDeserializer {
    value: Element
}
impl VariantDeserializer {
    pub fn new(v: Element) -> Self {
        VariantDeserializer { value: v}
    }
}
impl<'de> VariantAccess<'de> for VariantDeserializer {
    type Error = TychoError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as DeserializeSeed<'de>>::Value, Self::Error> where
        T: DeserializeSeed<'de> {
        seed.deserialize(TychoDeserializer::new(self.value))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        TychoDeserializer::new(self.value).deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        TychoDeserializer::new(self.value).deserialize_struct("", fields, visitor)
    }
}