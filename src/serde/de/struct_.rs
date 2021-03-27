use std::collections::HashMap;

use serde::de::{DeserializeSeed, MapAccess};

use crate::Element;
use crate::error::TychoError;
use crate::serde::de::ident::TychoIdentDeserializer;
use crate::serde::de::TychoDeserializer;

pub struct StructDeserializer {
    map: HashMap<String, Element>,
    value: Option<Element>
}

impl StructDeserializer {
    pub fn new(m: HashMap<String, Element>) -> Self {
        Self {
            map: m,
            value: None
        }
    }
}

impl<'de> MapAccess<'de> for StructDeserializer {
    type Error = TychoError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<<K as DeserializeSeed<'de>>::Value>, Self::Error> where
        K: DeserializeSeed<'de> {
        if self.map.len() == 0 {
            return Ok(None)
        } else {
            let f_key = self.map.keys().nth(0).unwrap().clone();
            let (key, value) = self.map.remove_entry(&f_key).unwrap();

            self.value = Some(value);

            Ok(Some(seed.deserialize(TychoIdentDeserializer::new(&key))?))
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