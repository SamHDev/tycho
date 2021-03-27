use std::collections::HashMap;

use serde::de::{DeserializeSeed, MapAccess};

use crate::{Element, Value};
use crate::error::TychoError;
use crate::serde::de::TychoDeserializer;

pub struct MapDeserializer {
    map: HashMap<Value, Element>,
    value: Option<Element>
}

impl MapDeserializer {
    pub fn new(m: HashMap<Value, Element>) -> Self {
        Self { map: m, value: None}
    }
}

impl<'de> MapAccess<'de> for MapDeserializer {
    type Error = TychoError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<<K as DeserializeSeed<'de>>::Value>, Self::Error> where
        K: DeserializeSeed<'de> {
        if self.map.len() == 0 {
            return Ok(None)
        } else {
            let f_key = self.map.keys().nth(0).unwrap().clone();
            let (key, value) = self.map.remove_entry(&f_key).unwrap();

            self.value = Some(value);

            Ok(Some(seed.deserialize(TychoDeserializer::new(Element::Value(key)))?))
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<<V as DeserializeSeed<'de>>::Value, Self::Error> where
        V: DeserializeSeed<'de> {
        match std::mem::replace(&mut self.value, None) {
            Some(x) => seed.deserialize(TychoDeserializer::new(x)),
            None => seed.deserialize(TychoDeserializer::new(Element::Unit))
        }

    }
}