use std::collections::HashMap;
use crate::{Value, Element};
use serde::de::{MapAccess, DeserializeSeed};
use crate::de::error::TychoDeserializeError;
use crate::de::de::TychoDeserializer;
use crate::ident::ElementIdent;
use crate::encode::element::ElementEncoder;

pub struct TychoMapDeserializer {
    map: HashMap<Value, Element>,
    key: Option<Value>,
    increment: usize
}

impl TychoMapDeserializer {
    pub(crate) fn new(v: HashMap<Value, Element>) -> Self {
        Self {
            map: v,
            key: None,
            increment: 0
        }
    }

    pub(crate) fn from(e: Element) -> Result<Self, TychoDeserializeError> {
        if let Element::Map(value) = e {
            Ok(Self::new(value))
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::Map,
                found: e.ident()
            })
        }
    }
}

impl<'a> MapAccess<'a> for TychoMapDeserializer {
    type Error = TychoDeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<<K as DeserializeSeed<'a>>::Value>, Self::Error> where
        K: DeserializeSeed<'a> {
        if let Some(key) = self.map.keys().nth(self.increment) {
            self.increment += 1;
            self.key = Some(key.clone());
            Ok(Some(seed.deserialize(TychoDeserializer::new(Element::Value(key.clone())))?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<<V as DeserializeSeed<'a>>::Value, Self::Error> where
        V: DeserializeSeed<'a> {
        if let Some(key) = self.key.clone() {
            if let Some(value) = self.map.get(&key) {
                self.key = None;
                seed.deserialize(TychoDeserializer::new(value.clone()))
            } else {
                Err(TychoDeserializeError::KeyError)
            }
        } else {
            Err(TychoDeserializeError::KeyError)
        }
    }
}