use serde::de::{DeserializeSeed, SeqAccess};

use crate::{Element, Value};
use crate::error::TychoError;
use crate::serde::de::TychoDeserializer;

pub struct SeqArrayDeserializer {
    array: Vec<Value>,
}

impl SeqArrayDeserializer {
    pub fn new(x: Vec<Value>) -> Self {
        Self { array: x }
    }
}

impl<'de> SeqAccess<'de> for SeqArrayDeserializer {
    type Error = TychoError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as DeserializeSeed<'de>>::Value>, Self::Error> where
        T: DeserializeSeed<'de> {
        if self.array.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(seed.deserialize(TychoDeserializer::new(Element::Value(self.array.remove(0))))?))
        }

    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.array.len())
    }
}

pub struct SeqListDeserializer {
    array: Vec<Element>,
}

impl SeqListDeserializer {
    pub fn new(x: Vec<Element>) -> Self {
        Self { array: x }
    }
}

impl<'de> SeqAccess<'de> for SeqListDeserializer {
    type Error = TychoError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as DeserializeSeed<'de>>::Value>, Self::Error> where
        T: DeserializeSeed<'de> {
        if self.array.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(seed.deserialize(TychoDeserializer::new(self.array.remove(0)))?))
        }

    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.array.len())
    }
}