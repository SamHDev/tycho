use serde::de::{VariantAccess, DeserializeSeed, Visitor, EnumAccess};
use crate::{Element, Value};
use crate::de::error::TychoDeserializeError;
use crate::ident::ElementIdent;
use crate::encode::element::ElementEncoder;
use crate::de::de::TychoDeserializer;
use crate::de::seq::{TychoArrayDeserializer, TychoListDeserializer};
use crate::de::sct::TychoStructDeserializer;

pub struct TychoVariantDeserializer {
    name: String,
    inner: Element
}

impl TychoVariantDeserializer {
    pub(crate) fn new(name: &str, inner: Element) -> Self {
        Self { name: name.to_string(), inner}
    }

    #[allow(dead_code)]
    pub(crate) fn from(e: Element) -> Result<Self, TychoDeserializeError> {
        if let Element::Variant(name, inner) = e {
            Ok(Self::new(&name, *inner))
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::Variant,
                found: e.ident()
            })
        }
    }
}

impl<'a> EnumAccess<'a> for TychoVariantDeserializer {
    type Error = TychoDeserializeError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(<V as DeserializeSeed<'a>>::Value, Self::Variant), Self::Error> where
        V: DeserializeSeed<'a> {
        Ok((seed.deserialize(TychoDeserializer::new(Element::Value(Value::String(self.name.clone()))))?, self))

    }
}

impl<'a> VariantAccess<'a> for TychoVariantDeserializer {
    type Error = TychoDeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        if let Element::Unit = self.inner {
            Ok(())
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::Unit,
                found: self.inner.ident()
            })
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as DeserializeSeed<'a>>::Value, Self::Error> where
        T: DeserializeSeed<'a> {
        seed.deserialize(TychoDeserializer::new(self.inner))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<<V as Visitor<'a>>::Value, Self::Error> where
        V: Visitor<'a> {
        if let Element::Array(array) = self.inner {
            visitor.visit_seq(TychoArrayDeserializer::new(array))
        } else if let Element::List(list) = self.inner {
            visitor.visit_seq(TychoListDeserializer::new(list))
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::Array,
                found: self.inner.ident()
            })
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'a>>::Value, Self::Error> where
        V: Visitor<'a> {
        if let Element::Struct(s) = self.inner {
            visitor.visit_map(TychoStructDeserializer::new(s))
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::Unit,
                found: self.inner.ident()
            })
        }
    }
}