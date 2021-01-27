use std::collections::HashMap;
use crate::{Element, Value};
use serde::de::{MapAccess, DeserializeSeed};
use crate::de::error::TychoDeserializeError;
use crate::de::de::TychoDeserializer;
use crate::ident::ElementIdent;
use crate::encode::element::ElementEncoder;

pub struct TychoStructDeserializer {
    data: HashMap<String, Element>,
    key: Option<String>
}

impl TychoStructDeserializer {
    pub(crate) fn new(data: HashMap<String, Element>) -> Self {
        Self {
            data,
            key: None
        }
    }
    pub(crate) fn from(e: Element) -> Result<Self, TychoDeserializeError> {
        if let Element::Struct(value) = e {
            Ok(Self::new(value))
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::Struct,
                found: e.ident()
            })
        }
    }
}

impl<'a> MapAccess<'a> for TychoStructDeserializer {
    type Error = TychoDeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<<K as DeserializeSeed<'a>>::Value>, Self::Error> where
        K: DeserializeSeed<'a> {
        match self.data.keys().nth(0) {
            Some(key) => {
                let k = seed.deserialize(TychoDeserializer::new(Element::Value(Value::String(key.clone()))))?;
                self.key = Some(key.clone());
                Ok(Some(k))
            }
            None => Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<<V as DeserializeSeed<'a>>::Value, Self::Error> where
        V: DeserializeSeed<'a> {
        match self.key.clone() {
            Some(key) => {
                self.key = None;
                if let Some(value) = self.data.remove(&key) {
                    seed.deserialize(TychoDeserializer::new(value))
                } else {
                    Err(TychoDeserializeError::KeyError)
                }
            },
            None => Err(TychoDeserializeError::KeyError)
        }
    }
}